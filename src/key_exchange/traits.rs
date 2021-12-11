// Copyright (c) Facebook, Inc. and its affiliates.
//
// This source code is licensed under both the MIT license found in the
// LICENSE-MIT file in the root directory of this source tree and the Apache
// License, Version 2.0 found in the LICENSE-APACHE file in the root directory
// of this source tree.

use crate::key_exchange::group::KeGroup;
use crate::{
    ciphersuite::CipherSuite,
    errors::ProtocolError,
    hash::Hash,
    keypair::{PrivateKey, PublicKey, SecretKey},
};
use alloc::vec::Vec;
use digest::Digest;
use generic_array::GenericArray;
use rand::{CryptoRng, RngCore};
use zeroize::Zeroize;

#[cfg(not(test))]
pub type GenerateKe2Result<K, D, G> = (
    <K as KeyExchange<D, G>>::KE2State,
    <K as KeyExchange<D, G>>::KE2Message,
);
#[cfg(test)]
pub type GenerateKe2Result<K, D, G> = (
    <K as KeyExchange<D, G>>::KE2State,
    <K as KeyExchange<D, G>>::KE2Message,
    GenericArray<u8, <D as Digest>::OutputSize>,
    GenericArray<u8, <D as Digest>::OutputSize>,
);
#[cfg(not(test))]
pub type GenerateKe3Result<K, D, G> = (
    GenericArray<u8, <D as Digest>::OutputSize>,
    <K as KeyExchange<D, G>>::KE3Message,
);
#[cfg(test)]
pub type GenerateKe3Result<K, D, G> = (
    GenericArray<u8, <D as Digest>::OutputSize>,
    <K as KeyExchange<D, G>>::KE3Message,
    GenericArray<u8, <D as Digest>::OutputSize>,
    GenericArray<u8, <D as Digest>::OutputSize>,
);

pub trait KeyExchange<D: Hash, G: KeGroup> {
    type KE1State: FromBytes + ToBytes + Zeroize + Clone;
    type KE2State: FromBytes + ToBytes + Zeroize + Clone;
    type KE1Message: FromBytes + ToBytes + Clone;
    type KE2Message: FromBytes + ToBytes + Clone;
    type KE3Message: FromBytes + ToBytes + Clone;

    fn generate_ke1<R: RngCore + CryptoRng>(
        rng: &mut R,
    ) -> Result<(Self::KE1State, Self::KE1Message), ProtocolError>;

    #[allow(clippy::too_many_arguments, clippy::type_complexity)]
    fn generate_ke2<'a, R: RngCore + CryptoRng, S: SecretKey<G>>(
        rng: &mut R,
        l1_bytes: Vec<u8>,
        l2_bytes: Vec<u8>,
        ke1_message: Self::KE1Message,
        client_s_pk: PublicKey<G>,
        server_s_sk: S,
        id_u: impl Iterator<Item = &'a [u8]>,
        id_s: impl Iterator<Item = &'a [u8]>,
        context: &[u8],
    ) -> Result<GenerateKe2Result<Self, D, G>, ProtocolError<S::Error>>;

    #[allow(clippy::too_many_arguments, clippy::type_complexity)]
    fn generate_ke3<'a>(
        l2_component: Vec<u8>,
        ke2_message: Self::KE2Message,
        ke1_state: &Self::KE1State,
        serialized_credential_request: &[u8],
        server_s_pk: PublicKey<G>,
        client_s_sk: PrivateKey<G>,
        id_u: impl Iterator<Item = &'a [u8]>,
        id_s: impl Iterator<Item = &'a [u8]>,
        context: &[u8],
    ) -> Result<GenerateKe3Result<Self, D, G>, ProtocolError>;

    #[allow(clippy::type_complexity)]
    fn finish_ke(
        ke3_message: Self::KE3Message,
        ke2_state: &Self::KE2State,
    ) -> Result<GenericArray<u8, D::OutputSize>, ProtocolError>;

    fn ke2_message_size() -> usize;
}

pub trait FromBytes: Sized {
    fn from_bytes<CS: CipherSuite>(input: &[u8]) -> Result<Self, ProtocolError>;
}

pub trait ToBytes {
    fn to_bytes(&self) -> Vec<u8>;
}
