// Copyright (c) Facebook, Inc. and its affiliates.
//
// This source code is licensed under both the MIT license found in the
// LICENSE-MIT file in the root directory of this source tree and the Apache
// License, Version 2.0 found in the LICENSE-APACHE file in the root directory
// of this source tree.

//! Contains the messages used for OPAQUE

use core::ops::Add;

use derive_where::derive_where;
use digest::core_api::{BlockSizeUser, CoreProxy};
use digest::{Output, OutputSizeUser};
use generic_array::sequence::Concat;
use generic_array::typenum::{IsLess, IsLessOrEqual, Le, NonZero, Sum, Unsigned, U256};
use generic_array::{ArrayLength, GenericArray};
use rand::{CryptoRng, RngCore};
use subtle::ConstantTimeEq;
use voprf::Group;

use crate::ciphersuite::{CipherSuite, OprfGroup, OprfHash};
use crate::envelope::{Envelope, EnvelopeLen};
use crate::errors::utils::{check_slice_size, check_slice_size_atleast};
use crate::errors::ProtocolError;
use crate::hash::{Hash, OutputSize, ProxyHash};
use crate::key_exchange::group::KeGroup;
use crate::key_exchange::traits::{
    FromBytes, Ke1MessageLen, Ke2MessageLen, Ke3MessageLen, KeyExchange, ToBytes,
};
use crate::key_exchange::tripledh::NonceLen;
use crate::keypair::{PublicKey, SecretKey};
use crate::opaque::{MaskedResponse, MaskedResponseLen, ServerSetup};

////////////////////////////
// High-level API Structs //
// ====================== //
////////////////////////////

/// The message sent by the client to the server, to initiate registration
#[derive_where(Clone)]
#[derive_where(Debug, Eq, Hash, Ord, PartialEq, PartialOrd; voprf::BlindedElement<CS::OprfCs>)]
pub struct RegistrationRequest<CS: CipherSuite>
where
    <OprfHash<CS> as OutputSizeUser>::OutputSize:
        IsLess<U256> + IsLessOrEqual<<OprfHash<CS> as BlockSizeUser>::BlockSize>,
    OprfHash<CS>: Hash,
    <OprfHash<CS> as CoreProxy>::Core: ProxyHash,
    <<OprfHash<CS> as CoreProxy>::Core as BlockSizeUser>::BlockSize: IsLess<U256>,
    Le<<<OprfHash<CS> as CoreProxy>::Core as BlockSizeUser>::BlockSize, U256>: NonZero,
{
    /// blinded password information
    pub(crate) blinded_element: voprf::BlindedElement<CS::OprfCs>,
}

impl_serialize_and_deserialize_for!(RegistrationRequest);

/// The answer sent by the server to the user, upon reception of the
/// registration attempt
#[derive_where(Clone)]
#[derive_where(Debug, Eq, Hash, Ord, PartialEq, PartialOrd; voprf::EvaluationElement<CS::OprfCs>, <CS::KeGroup as KeGroup>::Pk)]
pub struct RegistrationResponse<CS: CipherSuite>
where
    <OprfHash<CS> as OutputSizeUser>::OutputSize:
        IsLess<U256> + IsLessOrEqual<<OprfHash<CS> as BlockSizeUser>::BlockSize>,
    OprfHash<CS>: Hash,
    <OprfHash<CS> as CoreProxy>::Core: ProxyHash,
    <<OprfHash<CS> as CoreProxy>::Core as BlockSizeUser>::BlockSize: IsLess<U256>,
    Le<<<OprfHash<CS> as CoreProxy>::Core as BlockSizeUser>::BlockSize, U256>: NonZero,
{
    /// The server's oprf output
    pub(crate) evaluation_element: voprf::EvaluationElement<CS::OprfCs>,
    /// Server's static public key
    pub(crate) server_s_pk: PublicKey<CS::KeGroup>,
}

impl_serialize_and_deserialize_for!(
    RegistrationResponse
    where
        // RegistrationResponse: KgPk + KePk
        <OprfGroup<CS> as Group>::ElemLen: Add<<CS::KeGroup as KeGroup>::PkLen>,
        RegistrationResponseLen<CS>: ArrayLength<u8>,
);

/// The final message from the client, containing sealed cryptographic
/// identifiers
#[derive_where(Clone, ZeroizeOnDrop)]
#[derive_where(Debug, Eq, Hash, Ord, PartialEq, PartialOrd; <CS::KeGroup as KeGroup>::Pk)]
pub struct RegistrationUpload<CS: CipherSuite>
where
    <OprfHash<CS> as OutputSizeUser>::OutputSize:
        IsLess<U256> + IsLessOrEqual<<OprfHash<CS> as BlockSizeUser>::BlockSize>,
    OprfHash<CS>: Hash,
    <OprfHash<CS> as CoreProxy>::Core: ProxyHash,
    <<OprfHash<CS> as CoreProxy>::Core as BlockSizeUser>::BlockSize: IsLess<U256>,
    Le<<<OprfHash<CS> as CoreProxy>::Core as BlockSizeUser>::BlockSize, U256>: NonZero,
{
    /// The "envelope" generated by the user, containing sealed cryptographic
    /// identifiers
    pub(crate) envelope: Envelope<CS>,
    /// The masking key used to mask the envelope
    pub(crate) masking_key: Output<OprfHash<CS>>,
    /// The user's public key
    pub(crate) client_s_pk: PublicKey<CS::KeGroup>,
}

impl_serialize_and_deserialize_for!(
    RegistrationUpload
    where
        // Envelope: Nonce + Hash
        NonceLen: Add<OutputSize<OprfHash<CS>>>,
        EnvelopeLen<CS>: ArrayLength<u8>,
        // RegistrationUpload: (KePk + Hash) + Envelope
        <CS::KeGroup as KeGroup>::PkLen: Add<OutputSize<OprfHash<CS>>>,
        Sum<<CS::KeGroup as KeGroup>::PkLen, OutputSize<OprfHash<CS>>>:
            ArrayLength<u8> | Add<EnvelopeLen<CS>>,
        RegistrationUploadLen<CS>: ArrayLength<u8>,
);

/// The message sent by the user to the server, to initiate registration
#[derive_where(Clone, ZeroizeOnDrop)]
#[derive_where(
    Debug, Eq, Hash, PartialEq;
    voprf::BlindedElement<CS::OprfCs>,
    <CS::KeyExchange as KeyExchange<OprfHash<CS>, CS::KeGroup>>::KE1Message,
)]
pub struct CredentialRequest<CS: CipherSuite>
where
    <OprfHash<CS> as OutputSizeUser>::OutputSize:
        IsLess<U256> + IsLessOrEqual<<OprfHash<CS> as BlockSizeUser>::BlockSize>,
    OprfHash<CS>: Hash,
    <OprfHash<CS> as CoreProxy>::Core: ProxyHash,
    <<OprfHash<CS> as CoreProxy>::Core as BlockSizeUser>::BlockSize: IsLess<U256>,
    Le<<<OprfHash<CS> as CoreProxy>::Core as BlockSizeUser>::BlockSize, U256>: NonZero,
{
    pub(crate) blinded_element: voprf::BlindedElement<CS::OprfCs>,
    pub(crate) ke1_message: <CS::KeyExchange as KeyExchange<OprfHash<CS>, CS::KeGroup>>::KE1Message,
}

impl_serialize_and_deserialize_for!(
    CredentialRequest
    where
        // CredentialRequest: KgPk + Ke1Message
        <OprfGroup<CS> as Group>::ElemLen: Add<Ke1MessageLen<CS>>,
        CredentialRequestLen<CS>: ArrayLength<u8>,
);

/// The answer sent by the server to the user, upon reception of the login
/// attempt
#[derive_where(Clone)]
#[derive_where(
    Debug, Eq, Hash, PartialEq;
    voprf::EvaluationElement<CS::OprfCs>,
    <CS::KeyExchange as KeyExchange<OprfHash<CS>, CS::KeGroup>>::KE2Message,
)]
pub struct CredentialResponse<CS: CipherSuite>
where
    <OprfHash<CS> as OutputSizeUser>::OutputSize:
        IsLess<U256> + IsLessOrEqual<<OprfHash<CS> as BlockSizeUser>::BlockSize>,
    OprfHash<CS>: Hash,
    <OprfHash<CS> as CoreProxy>::Core: ProxyHash,
    <<OprfHash<CS> as CoreProxy>::Core as BlockSizeUser>::BlockSize: IsLess<U256>,
    Le<<<OprfHash<CS> as CoreProxy>::Core as BlockSizeUser>::BlockSize, U256>: NonZero,
{
    /// the server's oprf output
    pub(crate) evaluation_element: voprf::EvaluationElement<CS::OprfCs>,
    pub(crate) masking_nonce: GenericArray<u8, NonceLen>,
    pub(crate) masked_response: MaskedResponse<CS>,
    pub(crate) ke2_message: <CS::KeyExchange as KeyExchange<OprfHash<CS>, CS::KeGroup>>::KE2Message,
}

impl_serialize_and_deserialize_for!(
    CredentialResponse
    where
        // CredentialResponseWithoutKeLen: (KgPk + Nonce) + MaskedResponse
        <OprfGroup<CS> as Group>::ElemLen: Add<NonceLen>,
        Sum<<OprfGroup<CS> as Group>::ElemLen, NonceLen>:
            ArrayLength<u8> | Add<MaskedResponseLen<CS>>,
        CredentialResponseWithoutKeLen<CS>: ArrayLength<u8>,
        // MaskedResponse: (Nonce + Hash) + KePk
        NonceLen: Add<OutputSize<OprfHash<CS>>>,
        Sum<NonceLen, OutputSize<OprfHash<CS>>>:
            ArrayLength<u8> | Add<<CS::KeGroup as KeGroup>::PkLen>,
        MaskedResponseLen<CS>: ArrayLength<u8>,
        // CredentialResponse: CredentialResponseWithoutKeLen + Ke2Message
        CredentialResponseWithoutKeLen<CS>: Add<Ke2MessageLen<CS>>,
        CredentialResponseLen<CS>: ArrayLength<u8>,
);

/// The answer sent by the client to the server, upon reception of the sealed
/// envelope
#[derive_where(Clone)]
#[derive_where(
    Debug, Eq, Hash, PartialEq;
    <CS::KeyExchange as KeyExchange<OprfHash<CS>, CS::KeGroup>>::KE3Message,
)]
pub struct CredentialFinalization<CS: CipherSuite>
where
    <OprfHash<CS> as OutputSizeUser>::OutputSize:
        IsLess<U256> + IsLessOrEqual<<OprfHash<CS> as BlockSizeUser>::BlockSize>,
    OprfHash<CS>: Hash,
    <OprfHash<CS> as CoreProxy>::Core: ProxyHash,
    <<OprfHash<CS> as CoreProxy>::Core as BlockSizeUser>::BlockSize: IsLess<U256>,
    Le<<<OprfHash<CS> as CoreProxy>::Core as BlockSizeUser>::BlockSize, U256>: NonZero,
{
    pub(crate) ke3_message: <CS::KeyExchange as KeyExchange<OprfHash<CS>, CS::KeGroup>>::KE3Message,
}

impl_serialize_and_deserialize_for!(CredentialFinalization);

////////////////////////////////
// High-level Implementations //
// ========================== //
////////////////////////////////

/// Length of [`RegistrationRequest`] in bytes for serialization.
pub type RegistrationRequestLen<CS: CipherSuite> = <OprfGroup<CS> as Group>::ElemLen;

impl<CS: CipherSuite> RegistrationRequest<CS>
where
    <OprfHash<CS> as OutputSizeUser>::OutputSize:
        IsLess<U256> + IsLessOrEqual<<OprfHash<CS> as BlockSizeUser>::BlockSize>,
    OprfHash<CS>: Hash,
    <OprfHash<CS> as CoreProxy>::Core: ProxyHash,
    <<OprfHash<CS> as CoreProxy>::Core as BlockSizeUser>::BlockSize: IsLess<U256>,
    Le<<<OprfHash<CS> as CoreProxy>::Core as BlockSizeUser>::BlockSize, U256>: NonZero,
{
    /// Only used for testing purposes
    #[cfg(test)]
    pub fn get_blinded_element_for_testing(&self) -> voprf::BlindedElement<CS::OprfCs> {
        self.blinded_element.clone()
    }

    /// Serialization into bytes
    pub fn serialize(&self) -> GenericArray<u8, RegistrationRequestLen<CS>> {
        <OprfGroup<CS> as Group>::serialize_elem(self.blinded_element.value())
    }

    /// Deserialization from bytes
    pub fn deserialize(input: &[u8]) -> Result<Self, ProtocolError> {
        Ok(Self {
            blinded_element: voprf::BlindedElement::deserialize(input)?,
        })
    }
}

/// Length of [`RegistrationResponse`] in bytes for serialization.
pub type RegistrationResponseLen<CS: CipherSuite> =
    Sum<<OprfGroup<CS> as Group>::ElemLen, <CS::KeGroup as KeGroup>::PkLen>;

impl<CS: CipherSuite> RegistrationResponse<CS>
where
    <OprfHash<CS> as OutputSizeUser>::OutputSize:
        IsLess<U256> + IsLessOrEqual<<OprfHash<CS> as BlockSizeUser>::BlockSize>,
    OprfHash<CS>: Hash,
    <OprfHash<CS> as CoreProxy>::Core: ProxyHash,
    <<OprfHash<CS> as CoreProxy>::Core as BlockSizeUser>::BlockSize: IsLess<U256>,
    Le<<<OprfHash<CS> as CoreProxy>::Core as BlockSizeUser>::BlockSize, U256>: NonZero,
{
    /// Serialization into bytes
    pub fn serialize(&self) -> GenericArray<u8, RegistrationResponseLen<CS>>
    where
        // RegistrationResponse: KgPk + KePk
        <OprfGroup<CS> as Group>::ElemLen: Add<<CS::KeGroup as KeGroup>::PkLen>,
        RegistrationResponseLen<CS>: ArrayLength<u8>,
    {
        <OprfGroup<CS> as Group>::serialize_elem(self.evaluation_element.value())
            .concat(self.server_s_pk.to_bytes())
    }

    /// Deserialization from bytes
    pub fn deserialize(input: &[u8]) -> Result<Self, ProtocolError> {
        let elem_len = <OprfGroup<CS> as Group>::ElemLen::USIZE;
        let key_len = <CS::KeGroup as KeGroup>::PkLen::USIZE;
        let checked_slice =
            check_slice_size(input, elem_len + key_len, "registration_response_bytes")?;

        // Ensure that public key is valid
        let server_s_pk = PublicKey::from_bytes(&checked_slice[elem_len..])?;

        Ok(Self {
            evaluation_element: voprf::EvaluationElement::deserialize(&checked_slice[..elem_len])?,
            server_s_pk,
        })
    }

    #[cfg(test)]
    /// Only used for tests, where we can set the beta value to test for the
    /// reflection error case
    pub fn set_evaluation_element_for_testing(&self, beta: <OprfGroup<CS> as Group>::Elem) -> Self {
        Self {
            evaluation_element: voprf::EvaluationElement::from_value_unchecked(beta),
            server_s_pk: self.server_s_pk.clone(),
        }
    }
}

/// Length of [`RegistrationUpload`] in bytes for serialization.
pub type RegistrationUploadLen<CS: CipherSuite> =
    Sum<Sum<<CS::KeGroup as KeGroup>::PkLen, OutputSize<OprfHash<CS>>>, EnvelopeLen<CS>>;

impl<CS: CipherSuite> RegistrationUpload<CS>
where
    <OprfHash<CS> as OutputSizeUser>::OutputSize:
        IsLess<U256> + IsLessOrEqual<<OprfHash<CS> as BlockSizeUser>::BlockSize>,
    OprfHash<CS>: Hash,
    <OprfHash<CS> as CoreProxy>::Core: ProxyHash,
    <<OprfHash<CS> as CoreProxy>::Core as BlockSizeUser>::BlockSize: IsLess<U256>,
    Le<<<OprfHash<CS> as CoreProxy>::Core as BlockSizeUser>::BlockSize, U256>: NonZero,
{
    /// Serialization into bytes
    pub fn serialize(&self) -> GenericArray<u8, RegistrationUploadLen<CS>>
    where
        // Envelope: Nonce + Hash
        NonceLen: Add<OutputSize<OprfHash<CS>>>,
        EnvelopeLen<CS>: ArrayLength<u8>,
        // RegistrationUpload: (KePk + Hash) + Envelope
        <CS::KeGroup as KeGroup>::PkLen: Add<OutputSize<OprfHash<CS>>>,
        Sum<<CS::KeGroup as KeGroup>::PkLen, OutputSize<OprfHash<CS>>>:
            ArrayLength<u8> + Add<EnvelopeLen<CS>>,
        RegistrationUploadLen<CS>: ArrayLength<u8>,
    {
        self.client_s_pk
            .to_bytes()
            .concat(self.masking_key.clone())
            .concat(self.envelope.serialize())
    }

    /// Deserialization from bytes
    pub fn deserialize(input: &[u8]) -> Result<Self, ProtocolError> {
        let key_len = <CS::KeGroup as KeGroup>::PkLen::USIZE;
        let hash_len = OutputSize::<OprfHash<CS>>::USIZE;
        let checked_slice =
            check_slice_size_atleast(input, key_len + hash_len, "registration_upload_bytes")?;
        let envelope = Envelope::<CS>::deserialize(&checked_slice[key_len + hash_len..])?;
        Ok(Self {
            envelope,
            masking_key: GenericArray::clone_from_slice(
                &checked_slice[key_len..key_len + hash_len],
            ),
            client_s_pk: PublicKey::from_bytes(&checked_slice[..key_len])?,
        })
    }

    // Creates a dummy instance used for faking a [CredentialResponse]
    pub(crate) fn dummy<R: RngCore + CryptoRng, S: SecretKey<CS::KeGroup>>(
        rng: &mut R,
        server_setup: &ServerSetup<CS, S>,
    ) -> Self {
        let mut masking_key = Output::<OprfHash<CS>>::default();
        rng.fill_bytes(&mut masking_key);

        Self {
            envelope: Envelope::<CS>::dummy(),
            masking_key,
            client_s_pk: server_setup.fake_keypair.public().clone(),
        }
    }
}

/// Length of [`CredentialRequest`] in bytes for serialization.
pub type CredentialRequestLen<CS: CipherSuite> =
    Sum<<OprfGroup<CS> as Group>::ElemLen, Ke1MessageLen<CS>>;

impl<CS: CipherSuite> CredentialRequest<CS>
where
    <OprfHash<CS> as OutputSizeUser>::OutputSize:
        IsLess<U256> + IsLessOrEqual<<OprfHash<CS> as BlockSizeUser>::BlockSize>,
    OprfHash<CS>: Hash,
    <OprfHash<CS> as CoreProxy>::Core: ProxyHash,
    <<OprfHash<CS> as CoreProxy>::Core as BlockSizeUser>::BlockSize: IsLess<U256>,
    Le<<<OprfHash<CS> as CoreProxy>::Core as BlockSizeUser>::BlockSize, U256>: NonZero,
{
    /// Serialization into bytes
    pub fn serialize(&self) -> GenericArray<u8, CredentialRequestLen<CS>>
    where
        // CredentialRequest: KgPk + Ke1Message
        <OprfGroup<CS> as Group>::ElemLen: Add<Ke1MessageLen<CS>>,
        CredentialRequestLen<CS>: ArrayLength<u8>,
    {
        <OprfGroup<CS> as Group>::serialize_elem(self.blinded_element.value())
            .concat(self.ke1_message.to_bytes())
    }

    pub(crate) fn serialize_iter<'a>(
        blinded_element: &'a GenericArray<u8, <OprfGroup<CS> as Group>::ElemLen>,
        ke1_message: &'a GenericArray<u8, Ke1MessageLen<CS>>,
    ) -> impl Iterator<Item = &'a [u8]> {
        [blinded_element.as_slice(), ke1_message].into_iter()
    }

    /// Deserialization from bytes
    pub fn deserialize(input: &[u8]) -> Result<Self, ProtocolError> {
        let elem_len = <OprfGroup<CS> as Group>::ElemLen::USIZE;

        let checked_slice = check_slice_size_atleast(input, elem_len, "login_first_message_bytes")?;

        // Check that the message is actually containing an element of the correct
        // subgroup
        let blinded_element =
            voprf::BlindedElement::<CS::OprfCs>::deserialize(&checked_slice[..elem_len])?;

        // Throw an error if the identity group element is encountered
        if bool::from(<OprfGroup<CS> as Group>::identity_elem().ct_eq(&blinded_element.value())) {
            return Err(ProtocolError::IdentityGroupElementError);
        }

        let ke1_message =
            <CS::KeyExchange as KeyExchange<OprfHash<CS>, CS::KeGroup>>::KE1Message::from_bytes(
                &checked_slice[elem_len..],
            )?;

        Ok(Self {
            blinded_element,
            ke1_message,
        })
    }

    /// Only used for testing purposes
    #[cfg(test)]
    pub fn get_blinded_element_for_testing(&self) -> voprf::BlindedElement<CS::OprfCs> {
        self.blinded_element.clone()
    }
}

/// Length of [`CredentialResponse`] in bytes for serialization.
pub type CredentialResponseLen<CS: CipherSuite> =
    Sum<CredentialResponseWithoutKeLen<CS>, Ke2MessageLen<CS>>;

pub(crate) type CredentialResponseWithoutKeLen<CS: CipherSuite> =
    Sum<Sum<<OprfGroup<CS> as Group>::ElemLen, NonceLen>, MaskedResponseLen<CS>>;

impl<CS: CipherSuite> CredentialResponse<CS>
where
    <OprfHash<CS> as OutputSizeUser>::OutputSize:
        IsLess<U256> + IsLessOrEqual<<OprfHash<CS> as BlockSizeUser>::BlockSize>,
    OprfHash<CS>: Hash,
    <OprfHash<CS> as CoreProxy>::Core: ProxyHash,
    <<OprfHash<CS> as CoreProxy>::Core as BlockSizeUser>::BlockSize: IsLess<U256>,
    Le<<<OprfHash<CS> as CoreProxy>::Core as BlockSizeUser>::BlockSize, U256>: NonZero,
{
    /// Serialization into bytes
    pub fn serialize(&self) -> GenericArray<u8, CredentialResponseLen<CS>>
    where
        // CredentialResponseWithoutKeLen: (KgPk + Nonce) + MaskedResponse
        <OprfGroup<CS> as Group>::ElemLen: Add<NonceLen>,
        Sum<<OprfGroup<CS> as Group>::ElemLen, NonceLen>:
            ArrayLength<u8> + Add<MaskedResponseLen<CS>>,
        CredentialResponseWithoutKeLen<CS>: ArrayLength<u8>,
        // MaskedResponse: (Nonce + Hash) + KePk
        NonceLen: Add<OutputSize<OprfHash<CS>>>,
        Sum<NonceLen, OutputSize<OprfHash<CS>>>:
            ArrayLength<u8> + Add<<CS::KeGroup as KeGroup>::PkLen>,
        MaskedResponseLen<CS>: ArrayLength<u8>,
        // CredentialResponse: CredentialResponseWithoutKeLen + Ke2Message
        CredentialResponseWithoutKeLen<CS>: Add<Ke2MessageLen<CS>>,
        CredentialResponseLen<CS>: ArrayLength<u8>,
    {
        <OprfGroup<CS> as Group>::serialize_elem(self.evaluation_element.value())
            .concat(self.masking_nonce)
            .concat(self.masked_response.serialize())
            .concat(self.ke2_message.to_bytes())
    }

    pub(crate) fn serialize_without_ke<'a>(
        beta: &'a GenericArray<u8, <OprfGroup<CS> as Group>::ElemLen>,
        masking_nonce: &'a GenericArray<u8, NonceLen>,
        masked_response: &'a MaskedResponse<CS>,
    ) -> impl Iterator<Item = &'a [u8]> {
        [beta.as_slice(), masking_nonce.as_slice()]
            .into_iter()
            .chain(masked_response.iter())
    }

    /// Deserialization from bytes
    pub fn deserialize(input: &[u8]) -> Result<Self, ProtocolError> {
        let elem_len = <OprfGroup<CS> as Group>::ElemLen::USIZE;
        let key_len = <CS::KeGroup as KeGroup>::PkLen::USIZE;
        let nonce_len = NonceLen::USIZE;
        let envelope_len = Envelope::<CS>::len();
        let masked_response_len = key_len + envelope_len;
        let ke2_message_len = Ke2MessageLen::<CS>::USIZE;

        let checked_slice = check_slice_size_atleast(
            input,
            elem_len + nonce_len + masked_response_len + ke2_message_len,
            "credential_response_bytes",
        )?;

        // Check that the message is actually containing an element of the correct
        // subgroup
        let beta_bytes = &checked_slice[..elem_len];
        let evaluation_element = voprf::EvaluationElement::<CS::OprfCs>::deserialize(beta_bytes)?;

        // Throw an error if the identity group element is encountered
        if bool::from(<OprfGroup<CS> as Group>::identity_elem().ct_eq(&evaluation_element.value()))
        {
            return Err(ProtocolError::IdentityGroupElementError);
        }

        let masking_nonce =
            GenericArray::clone_from_slice(&checked_slice[elem_len..elem_len + nonce_len]);
        let masked_response = MaskedResponse::deserialize(
            &checked_slice[elem_len + nonce_len..elem_len + nonce_len + masked_response_len],
        );
        let ke2_message =
            <CS::KeyExchange as KeyExchange<OprfHash<CS>, CS::KeGroup>>::KE2Message::from_bytes(
                &checked_slice[elem_len + nonce_len + masked_response_len..],
            )?;

        Ok(Self {
            evaluation_element,
            masking_nonce,
            masked_response,
            ke2_message,
        })
    }

    #[cfg(test)]
    /// Only used for tests, where we can set the beta value to test for the
    /// reflection error case
    pub fn set_evaluation_element_for_testing(&self, beta: <OprfGroup<CS> as Group>::Elem) -> Self {
        Self {
            evaluation_element: voprf::EvaluationElement::from_value_unchecked(beta),
            masking_nonce: self.masking_nonce,
            masked_response: self.masked_response.clone(),
            ke2_message: self.ke2_message.clone(),
        }
    }
}

/// Length of [`CredentialFinalization`] in bytes for serialization.
pub type CredentialFinalizationLen<CS: CipherSuite> = Ke3MessageLen<CS>;

impl<CS: CipherSuite> CredentialFinalization<CS>
where
    <OprfHash<CS> as OutputSizeUser>::OutputSize:
        IsLess<U256> + IsLessOrEqual<<OprfHash<CS> as BlockSizeUser>::BlockSize>,
    OprfHash<CS>: Hash,
    <OprfHash<CS> as CoreProxy>::Core: ProxyHash,
    <<OprfHash<CS> as CoreProxy>::Core as BlockSizeUser>::BlockSize: IsLess<U256>,
    Le<<<OprfHash<CS> as CoreProxy>::Core as BlockSizeUser>::BlockSize, U256>: NonZero,
{
    /// Serialization into bytes
    pub fn serialize(&self) -> GenericArray<u8, CredentialFinalizationLen<CS>> {
        self.ke3_message.to_bytes()
    }

    /// Deserialization from bytes
    pub fn deserialize(input: &[u8]) -> Result<Self, ProtocolError> {
        let ke3_message =
            <CS::KeyExchange as KeyExchange<OprfHash<CS>, CS::KeGroup>>::KE3Message::from_bytes(
                input,
            )?;
        Ok(Self { ke3_message })
    }
}
