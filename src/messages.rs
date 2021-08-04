// Copyright (c) Facebook, Inc. and its affiliates.
//
// This source code is licensed under the MIT license found in the
// LICENSE file in the root directory of this source tree.

//! Contains the messages used for OPAQUE

use crate::{
    ciphersuite::CipherSuite,
    envelope::Envelope,
    errors::{
        utils::{check_slice_size, check_slice_size_atleast},
        PakeError, ProtocolError,
    },
    group::Group,
    key_exchange::traits::{FromBytes, KeyExchange, ToBytes},
    keypair::{KeyPair, PublicKey, SecretKey, SizedBytesExt},
    opaque::ServerSetup,
};
use digest::Digest;
use generic_array::{typenum::Unsigned, GenericArray};
use generic_bytes::SizedBytes;
use rand::{CryptoRng, RngCore};

// Messages
// =========

/// The message sent by the client to the server, to initiate registration
pub struct RegistrationRequest<CS: CipherSuite> {
    /// blinded password information
    pub(crate) alpha: CS::OprfGroup,
}

impl<CS: CipherSuite> RegistrationRequest<CS> {
    /// Only used for testing purposes
    #[cfg(test)]
    pub fn get_alpha_for_testing(&self) -> CS::OprfGroup {
        self.alpha
    }
}

// Cannot be derived because it would require for CS to be Clone.
impl<CS: CipherSuite> Clone for RegistrationRequest<CS> {
    fn clone(&self) -> Self {
        Self { alpha: self.alpha }
    }
}

impl_debug_eq_hash_for!(struct RegistrationRequest<CS: CipherSuite>, [alpha], [CS::OprfGroup]);

impl<CS: CipherSuite> RegistrationRequest<CS> {
    /// Serialization into bytes
    pub fn serialize(&self) -> Vec<u8> {
        self.alpha.to_arr().to_vec()
    }

    /// Deserialization from bytes
    pub fn deserialize(input: &[u8]) -> Result<Self, ProtocolError> {
        let elem_len = <CS::OprfGroup as Group>::ElemLen::to_usize();
        let checked_slice = check_slice_size(input, elem_len, "first_message_bytes")?;
        // Check that the message is actually containing an element of the
        // correct subgroup
        let arr = GenericArray::from_slice(checked_slice);
        let alpha = CS::OprfGroup::from_element_slice(arr)?;

        // Throw an error if the identity group element is encountered
        if alpha.is_identity() {
            return Err(PakeError::IdentityGroupElementError.into());
        }
        Ok(Self { alpha })
    }
}

impl_serialize_and_deserialize_for!(RegistrationRequest);

/// The answer sent by the server to the user, upon reception of the
/// registration attempt
pub struct RegistrationResponse<CS: CipherSuite> {
    /// The server's oprf output
    pub(crate) beta: CS::OprfGroup,
    /// Server's static public key
    pub(crate) server_s_pk: PublicKey<CS::KeGroup>,
}

// Cannot be derived because it would require for CS to be Clone.
impl<CS: CipherSuite> Clone for RegistrationResponse<CS> {
    fn clone(&self) -> Self {
        Self {
            beta: self.beta,
            server_s_pk: self.server_s_pk.clone(),
        }
    }
}

impl_debug_eq_hash_for!(
    struct RegistrationResponse<CS: CipherSuite>,
    [beta, server_s_pk],
    [CS::OprfGroup],
);

impl<CS: CipherSuite> RegistrationResponse<CS> {
    /// Serialization into bytes
    pub fn serialize(&self) -> Vec<u8> {
        [self.beta.to_arr().to_vec(), self.server_s_pk.to_vec()].concat()
    }

    /// Deserialization from bytes
    pub fn deserialize(input: &[u8]) -> Result<Self, ProtocolError> {
        let elem_len = <CS::OprfGroup as Group>::ElemLen::to_usize();
        let key_len = <PublicKey<CS::KeGroup> as SizedBytes>::Len::to_usize();
        let checked_slice =
            check_slice_size(input, elem_len + key_len, "registration_response_bytes")?;

        // Check that the message is actually containing an element of the
        // correct subgroup
        let arr = GenericArray::from_slice(&checked_slice[..elem_len]);
        let beta = CS::OprfGroup::from_element_slice(arr)?;

        // Throw an error if the identity group element is encountered
        if beta.is_identity() {
            return Err(PakeError::IdentityGroupElementError.into());
        }

        // Ensure that public key is valid
        let server_s_pk = KeyPair::<CS::KeGroup>::check_public_key(PublicKey::from_bytes(
            &checked_slice[elem_len..],
        )?)?;

        Ok(Self { beta, server_s_pk })
    }

    #[cfg(test)]
    /// Only used for tests, where we can set the beta value to test for the reflection
    /// error case
    pub fn set_beta_for_testing(&self, new_beta: CS::OprfGroup) -> Self {
        Self {
            beta: new_beta,
            server_s_pk: self.server_s_pk.clone(),
        }
    }
}

impl_serialize_and_deserialize_for!(RegistrationResponse);

/// The final message from the client, containing sealed cryptographic
/// identifiers
pub struct RegistrationUpload<CS: CipherSuite> {
    /// The "envelope" generated by the user, containing sealed
    /// cryptographic identifiers
    pub(crate) envelope: Envelope<CS>,
    /// The masking key used to mask the envelope
    pub(crate) masking_key: GenericArray<u8, <CS::Hash as Digest>::OutputSize>,
    /// The user's public key
    pub(crate) client_s_pk: PublicKey<CS::KeGroup>,
}

impl_clone_for!(
    struct RegistrationUpload<CS: CipherSuite>,
    [envelope, masking_key, client_s_pk],
);
impl_debug_eq_hash_for!(
    struct RegistrationUpload<CS: CipherSuite>,
    [envelope, masking_key, client_s_pk],
);

impl<CS: CipherSuite> RegistrationUpload<CS> {
    /// Serialization into bytes
    pub fn serialize(&self) -> Vec<u8> {
        [
            self.client_s_pk.to_arr().to_vec(),
            self.masking_key.to_vec(),
            self.envelope.serialize(),
        ]
        .concat()
    }

    /// Deserialization from bytes
    pub fn deserialize(input: &[u8]) -> Result<Self, ProtocolError> {
        let key_len = <PublicKey<CS::KeGroup> as SizedBytes>::Len::to_usize();
        let hash_len = <CS::Hash as Digest>::OutputSize::to_usize();
        let checked_slice =
            check_slice_size_atleast(input, key_len + hash_len, "registration_upload_bytes")?;
        let envelope = Envelope::<CS>::deserialize(&checked_slice[key_len + hash_len..])?;
        Ok(Self {
            envelope,
            masking_key: GenericArray::clone_from_slice(
                &checked_slice[key_len..key_len + hash_len],
            ),
            client_s_pk: KeyPair::<CS::KeGroup>::check_public_key(PublicKey::from_bytes(
                &checked_slice[..key_len],
            )?)?,
        })
    }

    // Creates a dummy instance used for faking a [CredentialResponse]
    pub(crate) fn dummy<R: RngCore + CryptoRng, S: SecretKey<CS::KeGroup>>(
        rng: &mut R,
        server_setup: &ServerSetup<CS, S>,
    ) -> Self {
        let mut masking_key = vec![0u8; <CS::Hash as Digest>::OutputSize::to_usize()];
        rng.fill_bytes(&mut masking_key);

        Self {
            envelope: Envelope::<CS>::dummy(),
            masking_key: GenericArray::clone_from_slice(&masking_key),
            client_s_pk: server_setup.fake_keypair.public().clone(),
        }
    }
}

impl_serialize_and_deserialize_for!(RegistrationUpload);

/// The message sent by the user to the server, to initiate registration
pub struct CredentialRequest<CS: CipherSuite> {
    /// blinded password information
    pub(crate) alpha: CS::OprfGroup,
    pub(crate) ke1_message: <CS::KeyExchange as KeyExchange<CS::Hash, CS::KeGroup>>::KE1Message,
}

// Cannot be derived because it would require for CS to be Clone.
impl<CS: CipherSuite> Clone for CredentialRequest<CS> {
    fn clone(&self) -> Self {
        Self {
            alpha: self.alpha,
            ke1_message: self.ke1_message.clone(),
        }
    }
}

impl_debug_eq_hash_for!(
    struct CredentialRequest<CS: CipherSuite>,
    [alpha, ke1_message],
    [
        CS::OprfGroup,
        <CS::KeyExchange as KeyExchange<CS::Hash, CS::KeGroup>>::KE1Message
    ],
);

impl<CS: CipherSuite> CredentialRequest<CS> {
    /// Serialization into bytes
    pub fn serialize(&self) -> Vec<u8> {
        [self.alpha.to_arr().to_vec(), self.ke1_message.to_bytes()].concat()
    }

    /// Deserialization from bytes
    pub fn deserialize(input: &[u8]) -> Result<Self, ProtocolError> {
        let elem_len = <CS::OprfGroup as Group>::ElemLen::to_usize();

        let checked_slice = check_slice_size_atleast(input, elem_len, "login_first_message_bytes")?;

        // Check that the message is actually containing an element of the
        // correct subgroup
        let arr = GenericArray::from_slice(&checked_slice[..elem_len]);
        let alpha = CS::OprfGroup::from_element_slice(arr)?;

        // Throw an error if the identity group element is encountered
        if alpha.is_identity() {
            return Err(PakeError::IdentityGroupElementError.into());
        }

        let ke1_message =
            <CS::KeyExchange as KeyExchange<CS::Hash, CS::KeGroup>>::KE1Message::from_bytes::<CS>(
                &checked_slice[elem_len..],
            )?;

        Ok(Self { alpha, ke1_message })
    }

    /// Only used for testing purposes
    #[cfg(test)]
    pub fn get_alpha_for_testing(&self) -> CS::OprfGroup {
        self.alpha
    }
}

impl_serialize_and_deserialize_for!(CredentialRequest);

/// The answer sent by the server to the user, upon reception of the
/// login attempt
pub struct CredentialResponse<CS: CipherSuite> {
    /// the server's oprf output
    pub(crate) beta: CS::OprfGroup,
    pub(crate) masking_nonce: Vec<u8>,
    pub(crate) masked_response: Vec<u8>,
    pub(crate) ke2_message: <CS::KeyExchange as KeyExchange<CS::Hash, CS::KeGroup>>::KE2Message,
}

// Cannot be derived because it would require for CS to be Clone.
impl<CS: CipherSuite> Clone for CredentialResponse<CS> {
    fn clone(&self) -> Self {
        Self {
            beta: self.beta,
            masking_nonce: self.masking_nonce.clone(),
            masked_response: self.masked_response.clone(),
            ke2_message: self.ke2_message.clone(),
        }
    }
}

impl_debug_eq_hash_for!(
    struct CredentialResponse<CS: CipherSuite>,
    [beta, masking_nonce, masked_response, ke2_message],
    [
        CS::OprfGroup,
        <CS::KeyExchange as KeyExchange<CS::Hash, CS::KeGroup>>::KE2Message,
    ],
);

impl<CS: CipherSuite> CredentialResponse<CS> {
    /// Serialization into bytes
    pub fn serialize(&self) -> Vec<u8> {
        [
            Self::serialize_without_ke(&self.beta, &self.masking_nonce, &self.masked_response),
            self.ke2_message.to_bytes(),
        ]
        .concat()
    }

    pub(crate) fn serialize_without_ke(
        beta: &CS::OprfGroup,
        masking_nonce: &[u8],
        masked_response: &[u8],
    ) -> Vec<u8> {
        [&beta.to_arr(), masking_nonce, masked_response].concat()
    }

    /// Deserialization from bytes
    pub fn deserialize(input: &[u8]) -> Result<Self, ProtocolError> {
        let elem_len = <CS::OprfGroup as Group>::ElemLen::to_usize();
        let key_len = <PublicKey<CS::KeGroup> as SizedBytes>::Len::to_usize();
        let nonce_len: usize = 32;
        let envelope_len = Envelope::<CS>::len();
        let masked_response_len = key_len + envelope_len;
        let ke2_message_len = CS::KeyExchange::ke2_message_size();

        let checked_slice = check_slice_size_atleast(
            input,
            elem_len + nonce_len + masked_response_len + ke2_message_len,
            "credential_response_bytes",
        )?;

        // Check that the message is actually containing an element of the
        // correct subgroup
        let beta_bytes = &checked_slice[..elem_len];
        let arr = GenericArray::from_slice(beta_bytes);
        let beta = CS::OprfGroup::from_element_slice(arr)?;

        // Throw an error if the identity group element is encountered
        if beta.is_identity() {
            return Err(PakeError::IdentityGroupElementError.into());
        }

        let masking_nonce = checked_slice[elem_len..elem_len + nonce_len].to_vec();
        let masked_response = checked_slice
            [elem_len + nonce_len..elem_len + nonce_len + masked_response_len]
            .to_vec();
        let ke2_message =
            <CS::KeyExchange as KeyExchange<CS::Hash, CS::KeGroup>>::KE2Message::from_bytes::<CS>(
                &checked_slice[elem_len + nonce_len + masked_response_len..],
            )?;

        Ok(Self {
            beta,
            masking_nonce,
            masked_response,
            ke2_message,
        })
    }

    #[cfg(test)]
    /// Only used for tests, where we can set the beta value to test for the reflection
    /// error case
    pub fn set_beta_for_testing(&self, new_beta: CS::OprfGroup) -> Self {
        Self {
            beta: new_beta,
            masking_nonce: self.masking_nonce.clone(),
            masked_response: self.masked_response.clone(),
            ke2_message: self.ke2_message.clone(),
        }
    }
}

impl_serialize_and_deserialize_for!(CredentialResponse);

/// The answer sent by the client to the server, upon reception of the
/// sealed envelope
pub struct CredentialFinalization<CS: CipherSuite> {
    pub(crate) ke3_message: <CS::KeyExchange as KeyExchange<CS::Hash, CS::KeGroup>>::KE3Message,
}

impl_clone_for!(struct CredentialFinalization<CS: CipherSuite>, [ke3_message]);
impl_debug_eq_hash_for!(
    struct CredentialFinalization<CS: CipherSuite>,
    [ke3_message],
    [<CS::KeyExchange as KeyExchange<CS::Hash, CS::KeGroup>>::KE3Message],
);

impl<CS: CipherSuite> CredentialFinalization<CS> {
    /// Serialization into bytes
    pub fn serialize(&self) -> Vec<u8> {
        self.ke3_message.to_bytes()
    }

    /// Deserialization from bytes
    pub fn deserialize(input: &[u8]) -> Result<Self, ProtocolError> {
        let ke3_message =
            <CS::KeyExchange as KeyExchange<CS::Hash, CS::KeGroup>>::KE3Message::from_bytes::<CS>(
                input,
            )?;
        Ok(Self { ke3_message })
    }
}

impl_serialize_and_deserialize_for!(CredentialFinalization);
