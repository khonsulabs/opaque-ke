// Copyright (c) Facebook, Inc. and its affiliates.
//
// This source code is licensed under the MIT license found in the
// LICENSE file in the root directory of this source tree.

use crate::{
    ciphersuite::CipherSuite,
    errors::{utils::check_slice_size, InternalError, ProtocolError},
    hash::{Hash, ProxyHash},
    key_exchange::group::KeGroup,
    keypair::{KeyPair, PublicKey},
    opaque::{bytestrings_from_identifiers, Identifiers},
};
use alloc::vec;
use alloc::vec::Vec;
use core::convert::TryFrom;
use derive_where::DeriveWhere;
use digest::core_api::{BlockSizeUser, CoreProxy};
use digest::{Digest, Output};
use generic_array::sequence::Concat;
use generic_array::typenum::{IsLess, Le, NonZero, Unsigned, U256, U32};
use generic_array::GenericArray;
use hkdf::Hkdf;
use hmac::{Hmac, Mac};
use rand::{CryptoRng, RngCore};
use voprf::group::Group;
use zeroize::Zeroize;

// Constant string used as salt for HKDF computation
const STR_AUTH_KEY: [u8; 7] = *b"AuthKey";
const STR_EXPORT_KEY: [u8; 9] = *b"ExportKey";
const STR_PRIVATE_KEY: [u8; 10] = *b"PrivateKey";
const STR_OPAQUE_DERIVE_AUTH_KEY_PAIR: [u8; 24] = *b"OPAQUE-DeriveAuthKeyPair";
type NonceLen = U32;

#[derive(Clone, Debug, Eq, Hash, PartialEq, Zeroize)]
#[zeroize(drop)]
pub(crate) enum InnerEnvelopeMode {
    Zero = 0,
    Internal = 1,
}

impl TryFrom<u8> for InnerEnvelopeMode {
    type Error = ProtocolError;
    fn try_from(x: u8) -> Result<Self, Self::Error> {
        match x {
            1 => Ok(InnerEnvelopeMode::Internal),
            _ => Err(ProtocolError::SerializationError),
        }
    }
}

/// This struct is an instantiation of the envelope as described in
/// <https://tools.ietf.org/html/draft-krawczyk-cfrg-opaque-06#section-4>
///
/// Note that earlier versions of this specification described an
/// implementation of this envelope using an encryption scheme that
/// satisfied random-key robustness
/// (<https://tools.ietf.org/html/draft-krawczyk-cfrg-opaque-05#section-4>).
/// The specification update has simplified this assumption by taking
/// an XOR-based approach without compromising on security, and to avoid
/// the confusion around the implementation of an RKR-secure encryption.
#[derive(DeriveWhere)]
#[derive_where(Clone, Debug, Eq, Hash, PartialEq, Zeroize(drop))]
pub(crate) struct Envelope<CS: CipherSuite>
where
    <CS::Hash as CoreProxy>::Core: ProxyHash,
    <<CS::Hash as CoreProxy>::Core as BlockSizeUser>::BlockSize: IsLess<U256>,
    Le<<<CS::Hash as CoreProxy>::Core as BlockSizeUser>::BlockSize, U256>: NonZero,
{
    mode: InnerEnvelopeMode,
    nonce: GenericArray<u8, NonceLen>,
    hmac: Output<CS::Hash>,
}

// Note that this struct represents an envelope that has been "opened" with the asssociated
// key. This key is also used to derive the export_key parameter, which is technically
// unrelated to the envelope's encrypted and authenticated contents.
pub(crate) struct OpenedEnvelope<CS: CipherSuite>
where
    <CS::Hash as CoreProxy>::Core: ProxyHash,
    <<CS::Hash as CoreProxy>::Core as BlockSizeUser>::BlockSize: IsLess<U256>,
    Le<<<CS::Hash as CoreProxy>::Core as BlockSizeUser>::BlockSize, U256>: NonZero,
{
    pub(crate) client_static_keypair: KeyPair<CS::KeGroup>,
    pub(crate) export_key: Output<CS::Hash>,
    pub(crate) id_u: Vec<u8>,
    pub(crate) id_s: Vec<u8>,
}

pub(crate) struct OpenedInnerEnvelope<D: Hash>
where
    <D as CoreProxy>::Core: ProxyHash,
    <<D as CoreProxy>::Core as BlockSizeUser>::BlockSize: IsLess<U256>,
    Le<<<D as CoreProxy>::Core as BlockSizeUser>::BlockSize, U256>: NonZero,
{
    pub(crate) export_key: Output<D>,
}

#[cfg(not(test))]
type SealRawResult<CS> = (Envelope<CS>, Output<<CS as CipherSuite>::Hash>);
#[cfg(test)]
type SealRawResult<CS> = (Envelope<CS>, Output<<CS as CipherSuite>::Hash>, Vec<u8>);
#[cfg(not(test))]
type SealResult<CS> = (
    Envelope<CS>,
    PublicKey<<CS as CipherSuite>::KeGroup>,
    Output<<CS as CipherSuite>::Hash>,
);
#[cfg(test)]
type SealResult<CS> = (
    Envelope<CS>,
    PublicKey<<CS as CipherSuite>::KeGroup>,
    Output<<CS as CipherSuite>::Hash>,
    Vec<u8>,
);

impl<CS: CipherSuite> Envelope<CS>
where
    <CS::Hash as CoreProxy>::Core: ProxyHash,
    <<CS::Hash as CoreProxy>::Core as BlockSizeUser>::BlockSize: IsLess<U256>,
    Le<<<CS::Hash as CoreProxy>::Core as BlockSizeUser>::BlockSize, U256>: NonZero,
{
    #[allow(clippy::type_complexity)]
    pub(crate) fn seal<R: RngCore + CryptoRng>(
        rng: &mut R,
        randomized_pwd_hasher: Hkdf<CS::Hash>,
        server_s_pk: &[u8],
        optional_ids: Option<Identifiers>,
    ) -> Result<SealResult<CS>, ProtocolError> {
        let mut nonce = GenericArray::default();
        rng.fill_bytes(&mut nonce);

        let (mode, client_s_pk) = (
            InnerEnvelopeMode::Internal,
            build_inner_envelope_internal::<CS>(randomized_pwd_hasher.clone(), nonce)?,
        );

        let (id_u, id_s) =
            bytestrings_from_identifiers(&optional_ids, &client_s_pk.to_arr(), server_s_pk)?;
        let aad = construct_aad(&id_u, &id_s, server_s_pk);

        let result = Self::seal_raw(randomized_pwd_hasher, nonce, &aad, mode)?;
        Ok((
            result.0,
            client_s_pk,
            result.1,
            #[cfg(test)]
            result.2,
        ))
    }

    /// Uses a key to convert the plaintext into an envelope, authenticated by the aad field.
    /// Note that a new nonce is sampled for each call to seal.
    #[allow(clippy::type_complexity)]
    pub(crate) fn seal_raw(
        randomized_pwd_hasher: Hkdf<CS::Hash>,
        nonce: GenericArray<u8, NonceLen>,
        aad: &[u8],
        mode: InnerEnvelopeMode,
    ) -> Result<SealRawResult<CS>, InternalError> {
        let mut hmac_key = vec![0u8; Self::hmac_key_size()];
        let mut export_key = vec![0u8; Self::export_key_size()];

        randomized_pwd_hasher
            .expand(&nonce.concat(STR_AUTH_KEY.into()), &mut hmac_key)
            .map_err(|_| InternalError::HkdfError)?;
        randomized_pwd_hasher
            .expand(&nonce.concat(STR_EXPORT_KEY.into()), &mut export_key)
            .map_err(|_| InternalError::HkdfError)?;

        let mut hmac =
            Hmac::<CS::Hash>::new_from_slice(&hmac_key).map_err(|_| InternalError::HmacError)?;
        hmac.update(&nonce);
        hmac.update(aad);

        let hmac_bytes = hmac.finalize().into_bytes();

        Ok((
            Self {
                mode,
                nonce,
                hmac: hmac_bytes,
            },
            GenericArray::clone_from_slice(&export_key),
            #[cfg(test)]
            hmac_key,
        ))
    }

    pub(crate) fn open(
        &self,
        randomized_pwd_hasher: Hkdf<CS::Hash>,
        server_s_pk: &[u8],
        optional_ids: &Option<Identifiers>,
    ) -> Result<OpenedEnvelope<CS>, ProtocolError> {
        let client_static_keypair = match self.mode {
            InnerEnvelopeMode::Zero => {
                return Err(InternalError::IncompatibleEnvelopeModeError.into())
            }
            InnerEnvelopeMode::Internal => {
                recover_keys_internal::<CS>(randomized_pwd_hasher.clone(), self.nonce)?
            }
        };

        let (id_u, id_s) = bytestrings_from_identifiers(
            optional_ids,
            &client_static_keypair.public().to_arr(),
            server_s_pk,
        )?;
        let aad = construct_aad(&id_u, &id_s, server_s_pk);

        let opened = self.open_raw(randomized_pwd_hasher, &aad)?;

        Ok(OpenedEnvelope {
            client_static_keypair,
            export_key: opened.export_key,
            id_u,
            id_s,
        })
    }

    /// Attempts to decrypt the envelope using a key, which is successful only if the key and
    /// aad used to construct the envelope are the same.
    pub(crate) fn open_raw(
        &self,
        randomized_pwd_hasher: Hkdf<CS::Hash>,
        aad: &[u8],
    ) -> Result<OpenedInnerEnvelope<CS::Hash>, InternalError> {
        let mut hmac_key = vec![0u8; Self::hmac_key_size()];
        let mut export_key = vec![0u8; Self::export_key_size()];

        randomized_pwd_hasher
            .expand(&self.nonce.concat(STR_AUTH_KEY.into()), &mut hmac_key)
            .map_err(|_| InternalError::HkdfError)?;
        randomized_pwd_hasher
            .expand(&self.nonce.concat(STR_EXPORT_KEY.into()), &mut export_key)
            .map_err(|_| InternalError::HkdfError)?;

        let mut hmac =
            Hmac::<CS::Hash>::new_from_slice(&hmac_key).map_err(|_| InternalError::HmacError)?;
        hmac.update(&self.nonce);
        hmac.update(aad);
        hmac.verify(&self.hmac)
            .map_err(|_| InternalError::SealOpenHmacError)?;

        Ok(OpenedInnerEnvelope {
            export_key: Output::<<CS as CipherSuite>::Hash>::clone_from_slice(&export_key),
        })
    }

    // Creates a dummy envelope object that serializes to the all-zeros byte string
    pub(crate) fn dummy() -> Self {
        Self {
            mode: InnerEnvelopeMode::Zero,
            nonce: GenericArray::default(),
            hmac: GenericArray::default(),
        }
    }

    fn hmac_key_size() -> usize {
        <CS::Hash>::output_size()
    }

    fn export_key_size() -> usize {
        <CS::Hash>::output_size()
    }

    pub(crate) fn len() -> usize {
        <CS::Hash>::output_size() + NonceLen::USIZE
    }

    pub(crate) fn serialize(&self) -> Vec<u8> {
        [self.nonce.as_slice(), &self.hmac].concat()
    }
    pub(crate) fn deserialize(bytes: &[u8]) -> Result<Self, ProtocolError> {
        let mode = InnerEnvelopeMode::Internal; // Better way to hard-code this?

        if bytes.len() < NonceLen::USIZE {
            return Err(ProtocolError::SerializationError);
        }
        let nonce = GenericArray::clone_from_slice(&bytes[..NonceLen::USIZE]);

        let remainder = match mode {
            InnerEnvelopeMode::Zero => {
                return Err(InternalError::IncompatibleEnvelopeModeError.into())
            }
            InnerEnvelopeMode::Internal => bytes[NonceLen::USIZE..].to_vec(),
        };

        let hmac_key_size = Self::hmac_key_size();
        let hmac = check_slice_size(&remainder, hmac_key_size, "hmac_key_size")?;

        Ok(Self {
            mode,
            nonce,
            hmac: GenericArray::clone_from_slice(hmac),
        })
    }
}

// Helper functions

fn build_inner_envelope_internal<CS: CipherSuite>(
    randomized_pwd_hasher: Hkdf<CS::Hash>,
    nonce: GenericArray<u8, NonceLen>,
) -> Result<PublicKey<CS::KeGroup>, ProtocolError>
where
    <CS::Hash as CoreProxy>::Core: ProxyHash,
    <<CS::Hash as CoreProxy>::Core as BlockSizeUser>::BlockSize: IsLess<U256>,
    Le<<<CS::Hash as CoreProxy>::Core as BlockSizeUser>::BlockSize, U256>: NonZero,
{
    let mut keypair_seed = vec![0u8; <CS::KeGroup as KeGroup>::SkLen::USIZE];
    randomized_pwd_hasher
        .expand(&nonce.concat(STR_PRIVATE_KEY.into()), &mut keypair_seed)
        .map_err(|_| InternalError::HkdfError)?;
    let client_static_keypair = KeyPair::<CS::KeGroup>::from_private_key_slice(
        &CS::OprfGroup::scalar_as_bytes(CS::OprfGroup::hash_to_scalar::<CS::Hash, _, _>(
            Some(keypair_seed.as_slice()),
            GenericArray::from(STR_OPAQUE_DERIVE_AUTH_KEY_PAIR),
        )?),
    )?;

    Ok(client_static_keypair.public().clone())
}

fn recover_keys_internal<CS: CipherSuite>(
    randomized_pwd_hasher: Hkdf<CS::Hash>,
    nonce: GenericArray<u8, NonceLen>,
) -> Result<KeyPair<CS::KeGroup>, ProtocolError>
where
    <CS::Hash as CoreProxy>::Core: ProxyHash,
    <<CS::Hash as CoreProxy>::Core as BlockSizeUser>::BlockSize: IsLess<U256>,
    Le<<<CS::Hash as CoreProxy>::Core as BlockSizeUser>::BlockSize, U256>: NonZero,
{
    let mut keypair_seed = vec![0u8; <CS::KeGroup as KeGroup>::SkLen::USIZE];
    randomized_pwd_hasher
        .expand(&nonce.concat(STR_PRIVATE_KEY.into()), &mut keypair_seed)
        .map_err(|_| InternalError::HkdfError)?;
    let client_static_keypair = KeyPair::<CS::KeGroup>::from_private_key_slice(
        &CS::OprfGroup::scalar_as_bytes(CS::OprfGroup::hash_to_scalar::<CS::Hash, _, _>(
            Some(keypair_seed.as_slice()),
            GenericArray::from(STR_OPAQUE_DERIVE_AUTH_KEY_PAIR),
        )?),
    )?;

    Ok(client_static_keypair)
}

fn construct_aad(id_u: &[u8], id_s: &[u8], server_s_pk: &[u8]) -> Vec<u8> {
    [server_s_pk, id_s, id_u].concat()
}
