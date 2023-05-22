// Copyright (c) Facebook, Inc. and its affiliates.
//
// This source code is licensed under both the MIT license found in the
// LICENSE-MIT file in the root directory of this source tree and the Apache
// License, Version 2.0 found in the LICENSE-APACHE file in the root directory
// of this source tree.

//! The OPAQUE test vectors taken from https://github.com/cfrg/draft-irtf-cfrg-opaque/blob/master/draft-irtf-cfrg-opaque.md,
//! which are presented in https://www.ietf.org/archive/id/draft-irtf-cfrg-opaque-08.txt

pub(crate) static VECTORS: &str = r#"
## Real Test Vectors {#real-vectors}

### OPAQUE-3DH Real Test Vector 1

#### Configuration

~~~
OPRF: ristretto255-SHA512
Hash: SHA512
KSF: Identity
KDF: HKDF-SHA512
MAC: HMAC-SHA512
Group: ristretto255
Context: 4f50415155452d504f43
Nh: 64
Npk: 32
Nsk: 32
Nm: 64
Nx: 64
Nok: 32
~~~

#### Input Values

~~~
oprf_seed: f433d0227b0b9dd54f7c4422b600e764e47fb503f1f9a0f0a47c6606b0
54a7fdc65347f1a08f277e22358bbabe26f823fca82c7848e9a75661f4ec5d5c1989e
f
credential_identifier: 31323334
password: 436f7272656374486f72736542617474657279537461706c65
envelope_nonce: ac13171b2f17bc2c74997f0fce1e1f35bec6b91fe2e12dbd323d2
3ba7a38dfec
masking_nonce: 38fe59af0df2c79f57b8780278f5ae47355fe1f817119041951c80
f612fdfc6d
server_private_key: 47451a85372f8b3537e249d7b54188091fb18edde78094b43
e2ba42b5eb89f0d
server_public_key: b2fe7af9f48cc502d016729d2fe25cdd433f2c4bc904660b2a
382c9b79df1a78
server_nonce: 71cd9960ecef2fe0d0f7494986fa3d8b2bb01963537e60efb13981e
138e3d4a1
client_nonce: da7e07376d6d6f034cfa9bb537d11b8c6b4238c334333d1f0aebb38
0cae6a6cc
server_public_keyshare: c8c39f573135474c51660b02425bca633e339cec4e1ac
c69c94dd48497fe4028
client_public_keyshare: 0c3a00c961fead8a16f818929cc976f0475e4f7235193
18b96f4947a7a5f9663
server_private_keyshare: 2e842960258a95e28bcfef489cffd19d8ec99cc1375d
840f96936da7dbb0b40d
client_private_keyshare: 22c919134c9bdd9dc0c5ef3450f18b54820f43f646a9
5223bf4a85b2018c2001
blind_registration: 76cfbfe758db884bebb33582331ba9f159720ca8784a2a070
a265d9c2d6abe01
blind_login: 6ecc102d2e7a7cf49617aad7bbe188556792d4acd60a1a8a8d2b65d4
b0790308
~~~

#### Intermediate Values

~~~
client_public_key: 2ec892bdbf9b3e2ea834be9eb11f5d187e64ba661ec041c0a3
b66db8b7d6cc30
auth_key: 6cd32316f18d72a9a927a83199fa030663a38ce0c11fbaef82aa9003773
0494fc555c4d49506284516edd1628c27965b7555a4ebfed2223199f6c67966dde822
randomized_password: aac48c25ab036e30750839d31d6e73007344cb1155289fb7
d329beb932e9adeea73d5d5c22a0ce1952f8aba6d66007615cd1698d4ac85ef1fcf15
0031d1435d9
envelope: ac13171b2f17bc2c74997f0fce1e1f35bec6b91fe2e12dbd323d23ba7a3
8dfecb9dbe7d48cf714fc3533becab6faf60b783c94d258477eb74ecc453413bf61c5
3fd58f0fb3c1175410b674c02e1b59b2d729a865b709db3dc4ee2bb45703d5a8
handshake_secret: 562564da0d4efdc73cb6efbb454388dabfa5052d4e7e83f4d02
40c5afd8352881e762755c2f1a9110e36b05fe770f0f48658489c9730dcd365e6c2d4
049c8fe3
server_mac_key: 59473632c53a647f9f4ab4d6c3b81e241dd9cb19ca05f0eabed7e
593f0407ff57e7f060621e5e48d5291be600a1959fbecbc26d4a7157bd227a993c37b
645f73
client_mac_key: f2d019bad603b45b2ac50376279a0a37d097723b5405aa4fb20a5
9f60cdbdd52ec043372cedcdbbdb634c54483e1be51a88d13a5798180acb84c10b129
7069fd
oprf_key: 5d4c6a8b7c7138182afb4345d1fae6a9f18a1744afbcc3854f8f5a2b4b4
c6d05
~~~

#### Output Values

~~~
registration_request: 5059ff249eb1551b7ce4991f3336205bde44a105a032e74
7d21bf382e75f7a71
registration_response: 7408a268083e03abc7097fc05b587834539065e86fb0c7
b6342fcf5e01e5b019b2fe7af9f48cc502d016729d2fe25cdd433f2c4bc904660b2a3
82c9b79df1a78
registration_upload: 2ec892bdbf9b3e2ea834be9eb11f5d187e64ba661ec041c0
a3b66db8b7d6cc301ac5844383c7708077dea41cbefe2fa15724f449e535dd7dd562e
66f5ecfb95864eadddec9db5874959905117dad40a4524111849799281fefe3c51fa8
2785c5ac13171b2f17bc2c74997f0fce1e1f35bec6b91fe2e12dbd323d23ba7a38dfe
cb9dbe7d48cf714fc3533becab6faf60b783c94d258477eb74ecc453413bf61c53fd5
8f0fb3c1175410b674c02e1b59b2d729a865b709db3dc4ee2bb45703d5a8
KE1: c4dedb0ba6ed5d965d6f250fbe554cd45cba5dfcce3ce836e4aee778aa3cd44d
da7e07376d6d6f034cfa9bb537d11b8c6b4238c334333d1f0aebb380cae6a6cc0c3a0
0c961fead8a16f818929cc976f0475e4f723519318b96f4947a7a5f9663
KE2: 7e308140890bcde30cbcea28b01ea1ecfbd077cff62c4def8efa075aabcbb471
38fe59af0df2c79f57b8780278f5ae47355fe1f817119041951c80f612fdfc6dd6ec6
0bcdb26dc455ddf3e718f1020490c192d70dfc7e403981179d8073d1146a4f9aa1ced
4e4cd984c657eb3b54ced3848326f70331953d91b02535af44d9fe0610f003be80cb2
098357928c8ea17bb065af33095f39d4e0b53b1687f02d522d96bad4ca354293d5c40
1177ccbd302cf565b96c327f71bc9eaf2890675d2fbb71cd9960ecef2fe0d0f749498
6fa3d8b2bb01963537e60efb13981e138e3d4a1c8c39f573135474c51660b02425bca
633e339cec4e1acc69c94dd48497fe40287f33611c2cf0eef57adbf48942737d9421e
6b20e4b9d6e391d4168bf4bf96ea57aa42ad41c977605e027a9ef706a349f4b2919fe
3562c8e86c4eeecf2f9457d4
KE3: df9a13cd256091f90f0fcb2ef6b3411e4aebff07bb0813299c0ec7f5dedd33a7
681231a001a82f1dece1777921f42abfeee551ee34392e1c9743c5cc1dc1ef8c
export_key: 1ef15b4fa99e8a852412450ab78713aad30d21fa6966c9b8c9fb3262a
970dc62950d4dd4ed62598229b1b72794fc0335199d9f7fcc6eaedde92cc04870e63f
16
session_key: 8a0f9f4928fc0c3b5bb261c4b7b3997600405424a8128632e85a5667
b4b742484ed791933971be6d3fcf2b23c56b8e8f7e7edcae19a03b8fd87f5999fce12
9d2
~~~

### OPAQUE-3DH Real Test Vector 2

#### Configuration

~~~
OPRF: ristretto255-SHA512
Hash: SHA512
KSF: Identity
KDF: HKDF-SHA512
MAC: HMAC-SHA512
Group: ristretto255
Context: 4f50415155452d504f43
Nh: 64
Npk: 32
Nsk: 32
Nm: 64
Nx: 64
Nok: 32
~~~

#### Input Values

~~~
client_identity: 616c696365
server_identity: 626f62
oprf_seed: f433d0227b0b9dd54f7c4422b600e764e47fb503f1f9a0f0a47c6606b0
54a7fdc65347f1a08f277e22358bbabe26f823fca82c7848e9a75661f4ec5d5c1989e
f
credential_identifier: 31323334
password: 436f7272656374486f72736542617474657279537461706c65
envelope_nonce: ac13171b2f17bc2c74997f0fce1e1f35bec6b91fe2e12dbd323d2
3ba7a38dfec
masking_nonce: 38fe59af0df2c79f57b8780278f5ae47355fe1f817119041951c80
f612fdfc6d
server_private_key: 47451a85372f8b3537e249d7b54188091fb18edde78094b43
e2ba42b5eb89f0d
server_public_key: b2fe7af9f48cc502d016729d2fe25cdd433f2c4bc904660b2a
382c9b79df1a78
server_nonce: 71cd9960ecef2fe0d0f7494986fa3d8b2bb01963537e60efb13981e
138e3d4a1
client_nonce: da7e07376d6d6f034cfa9bb537d11b8c6b4238c334333d1f0aebb38
0cae6a6cc
server_public_keyshare: c8c39f573135474c51660b02425bca633e339cec4e1ac
c69c94dd48497fe4028
client_public_keyshare: 0c3a00c961fead8a16f818929cc976f0475e4f7235193
18b96f4947a7a5f9663
server_private_keyshare: 2e842960258a95e28bcfef489cffd19d8ec99cc1375d
840f96936da7dbb0b40d
client_private_keyshare: 22c919134c9bdd9dc0c5ef3450f18b54820f43f646a9
5223bf4a85b2018c2001
blind_registration: 76cfbfe758db884bebb33582331ba9f159720ca8784a2a070
a265d9c2d6abe01
blind_login: 6ecc102d2e7a7cf49617aad7bbe188556792d4acd60a1a8a8d2b65d4
b0790308
~~~

#### Intermediate Values

~~~
client_public_key: 2ec892bdbf9b3e2ea834be9eb11f5d187e64ba661ec041c0a3
b66db8b7d6cc30
auth_key: 6cd32316f18d72a9a927a83199fa030663a38ce0c11fbaef82aa9003773
0494fc555c4d49506284516edd1628c27965b7555a4ebfed2223199f6c67966dde822
randomized_password: aac48c25ab036e30750839d31d6e73007344cb1155289fb7
d329beb932e9adeea73d5d5c22a0ce1952f8aba6d66007615cd1698d4ac85ef1fcf15
0031d1435d9
envelope: ac13171b2f17bc2c74997f0fce1e1f35bec6b91fe2e12dbd323d23ba7a3
8dfec1ac902dc5589e9a5f0de56ad685ea8486210ef41449cd4d8712828913c5d2b68
0b2b3af4a26c765cff329bfb66d38ecf1d6cfa9e7a73c222c6efe0d9520f7d7c
handshake_secret: bc2abaa979af9cbb6859856b7d5d201a038fbdfa7e10f11d131
d3f8f6fc3b263bde4db6d2d9207d4648ff80415a276d5f157f9d37a3eade559db2e5f
3fa026b2
server_mac_key: 2420461c589866700b08c8818cbf390c872629a14cf32a264dad3
375f85f33188c8f04bdb71880b2d4613187a0e416808ab62b45858b88319882602371
ef5f75
client_mac_key: 156e4ab0b9f71ef994bbbb73928e6d14d7335cf9561f113d61ac6
b41fab35f9c72fe827d3c4d7dd91d8398ee619810e4f9286e6b32f329eb6b1476ce18
fa8500
oprf_key: 5d4c6a8b7c7138182afb4345d1fae6a9f18a1744afbcc3854f8f5a2b4b4
c6d05
~~~

#### Output Values

~~~
registration_request: 5059ff249eb1551b7ce4991f3336205bde44a105a032e74
7d21bf382e75f7a71
registration_response: 7408a268083e03abc7097fc05b587834539065e86fb0c7
b6342fcf5e01e5b019b2fe7af9f48cc502d016729d2fe25cdd433f2c4bc904660b2a3
82c9b79df1a78
registration_upload: 2ec892bdbf9b3e2ea834be9eb11f5d187e64ba661ec041c0
a3b66db8b7d6cc301ac5844383c7708077dea41cbefe2fa15724f449e535dd7dd562e
66f5ecfb95864eadddec9db5874959905117dad40a4524111849799281fefe3c51fa8
2785c5ac13171b2f17bc2c74997f0fce1e1f35bec6b91fe2e12dbd323d23ba7a38dfe
c1ac902dc5589e9a5f0de56ad685ea8486210ef41449cd4d8712828913c5d2b680b2b
3af4a26c765cff329bfb66d38ecf1d6cfa9e7a73c222c6efe0d9520f7d7c
KE1: c4dedb0ba6ed5d965d6f250fbe554cd45cba5dfcce3ce836e4aee778aa3cd44d
da7e07376d6d6f034cfa9bb537d11b8c6b4238c334333d1f0aebb380cae6a6cc0c3a0
0c961fead8a16f818929cc976f0475e4f723519318b96f4947a7a5f9663
KE2: 7e308140890bcde30cbcea28b01ea1ecfbd077cff62c4def8efa075aabcbb471
38fe59af0df2c79f57b8780278f5ae47355fe1f817119041951c80f612fdfc6dd6ec6
0bcdb26dc455ddf3e718f1020490c192d70dfc7e403981179d8073d1146a4f9aa1ced
4e4cd984c657eb3b54ced3848326f70331953d91b02535af44d9fea502150b67fe367
95dd8914f164e49f81c7688a38928372134b7dccd50e09f8fed9518b7b2f94835b3c4
fe4c8475e7513f20eb97ff0568a39caee3fd6251876f71cd9960ecef2fe0d0f749498
6fa3d8b2bb01963537e60efb13981e138e3d4a1c8c39f573135474c51660b02425bca
633e339cec4e1acc69c94dd48497fe4028c463164503598ea84fab9005b9cd51b7bb3
206fb22a412e8a86b9cb6ffca18f5ea6b4c24fdc94865e8bf74248e6be15b85b16041
40ffad2175f9518452d381af
KE3: a86ece659d90525e2476aa1756d313b067581cb7b0643b97be6b8ab8d0f10843
57e514ecfaff9dc18f6cca37da630545f0048393f16bc175eb819653ebc45b60
export_key: 1ef15b4fa99e8a852412450ab78713aad30d21fa6966c9b8c9fb3262a
970dc62950d4dd4ed62598229b1b72794fc0335199d9f7fcc6eaedde92cc04870e63f
16
session_key: 0968e91efeb702d6aa09023a9a79803332d8bd3442a79b8ad09490b9
267161013bf475bed945238a5e976ef7d7de7ff41ae30439fe2fc39758fb3e56f2683
e60
~~~

### OPAQUE-3DH Real Test Vector 3

#### Configuration

~~~
OPRF: ristretto255-SHA512
Hash: SHA512
KSF: Identity
KDF: HKDF-SHA512
MAC: HMAC-SHA512
Group: curve25519
Context: 4f50415155452d504f43
Nh: 64
Npk: 32
Nsk: 32
Nm: 64
Nx: 64
Nok: 32
~~~

#### Input Values

~~~
oprf_seed: a78342ab84d3d30f08d5a9630c79bf311c31ed7f85d9d4959bf492ec67
a0eec8a67dfbf4497248eebd49e878aab173e5e4ff76354288fdd53e949a5f7c9f7f1
b
credential_identifier: 31323334
password: 436f7272656374486f72736542617474657279537461706c65
envelope_nonce: 40d6b67fdd7da7c49894750754514dbd2070a407166bd2a5237cc
a9bf44d6e0b
masking_nonce: 38fe59af0df2c79f57b8780278f5ae47355fe1f817119041951c80
f612fdfc6d
server_private_key: c06139381df63bfc91c850db0b9cfbec7a62e86d80040a41a
a7725bf0e79d564
server_public_key: a41e28269b4e97a66468cc00c5a57753e192e1527669897706
88aa90486ef031
server_nonce: 71cd9960ecef2fe0d0f7494986fa3d8b2bb01963537e60efb13981e
138e3d4a1
client_nonce: da7e07376d6d6f034cfa9bb537d11b8c6b4238c334333d1f0aebb38
0cae6a6cc
server_public_keyshare: 41f55f0bef355cfb34ccd468fdacad75865ee7efef95f
4cb6c25d477f7205026
client_public_keyshare: 10a83b9117d3798cb2957fbdb0268a0d63dbf9d66bde5
c00c78affd80026c911
server_private_keyshare: 00a4f54206eef1ba2f615bc0aa285cb22f26d1153b5b
40a1e85ff80da12f986f
client_private_keyshare: 80850a697b42a505f5b68fcdafce8c31f0af2b581f06
3cf1091933541936304b
blind_registration: c575731ffe1cb0ca5ba63b42c4699767b8b9ab78ba39316ee
04baddb2034a70a
blind_login: 6ecc102d2e7a7cf49617aad7bbe188556792d4acd60a1a8a8d2b65d4
b0790308
~~~

#### Intermediate Values

~~~
client_public_key: 0936ea94ab030ec332e29050d266c520e916731a052d05ced7
e0cfe751142b48
auth_key: 7e880ab484f750e80e6f839d975aff476070ce65066d85ea62523d1d576
4739d91307fac47186a4ab935e6a5c7f70cb47faa9473311947502c022cc67ae9440c
randomized_password: 3a602c295a9c323d9362fe286f104567ed6862b25dbe30fa
da844f19e41cf40047424b7118e15dc2c1a815a70fea5c8de6c30aa61440cd4b4b5e8
f3963fbb2e1
envelope: 40d6b67fdd7da7c49894750754514dbd2070a407166bd2a5237cca9bf44
d6e0b20c1e81fef28e92e897ca8287d49a55075b47c3988ff0fff367d79a3e350ccac
150b4a3ff48b4770c8e84e437b3d4e68d2b95833f7788f7eb93fa6a8afb85ecb
handshake_secret: 6936d2b78f13d865997dd37c42c741dfe4c0297199e55d7f889
1fa4771ed2357e990b44faec69733c607b7541442b5f27ea01513b4f0b84545e0ff75
81ea7764
server_mac_key: 62a11be878ecfcbd7c8875e86a0f479befcb4b1742480c9ac2d2a
5fa8d9e96c3bf60edb953ba15e32dc3e2cba60029c0c61481fdc7a80946f596b77fff
6b18ee
client_mac_key: e646055bf0a395c6c7c61078dabe0f0026ec6ba079ddb70f11ec9
ba41e5ff70e37a87d8fbeca11d7dc22e2aacf2828de6e1d776b451fa1bd4419b0a6b5
9cc353
oprf_key: 62ef7f7d9506a14600c34f642aaf6ef8019cc82a6755db4fded5248ea14
6030a
~~~

#### Output Values

~~~
registration_request: 26f3dbfd76b8e5f85b4da604f42889a7d4b1bc919f65538
1a67de02c59fd5436
registration_response: 506e8f1b89c098fb89b5b6210a05f7898cafdaea221761
e8d5272fc39e0f9f08a41e28269b4e97a66468cc00c5a57753e192e15276698977068
8aa90486ef031
registration_upload: 0936ea94ab030ec332e29050d266c520e916731a052d05ce
d7e0cfe751142b486d23c6ed818882f9bdfdcf91389fcbc0b7a3faf92bd0bd6be4a1e
7730277b694fc7c6ba327fbe786af18487688e0f7c148bbd54dc2fc80c28e7a976d9e
f53c3540d6b67fdd7da7c49894750754514dbd2070a407166bd2a5237cca9bf44d6e0
b20c1e81fef28e92e897ca8287d49a55075b47c3988ff0fff367d79a3e350ccac150b
4a3ff48b4770c8e84e437b3d4e68d2b95833f7788f7eb93fa6a8afb85ecb
KE1: c4dedb0ba6ed5d965d6f250fbe554cd45cba5dfcce3ce836e4aee778aa3cd44d
da7e07376d6d6f034cfa9bb537d11b8c6b4238c334333d1f0aebb380cae6a6cc10a83
b9117d3798cb2957fbdb0268a0d63dbf9d66bde5c00c78affd80026c911
KE2: 9a0e5a1514f62e005ea098b0d8cf6750e358c4389e6add1c52aed9500fa19d00
38fe59af0df2c79f57b8780278f5ae47355fe1f817119041951c80f612fdfc6d22cc3
1127d6f0096755be3c3d2dd6287795c317aeea10c9485bf4f419a786642c19a8f151c
eb5e8767d175248c62c017de94057398d28bf0ed00d1b50ee4f812fd9afddf98af8cd
58067ca43b0633b6cadd0e9d987f89623fed4d3583bdf6910c425600e90dab3c6b351
3188a465461a67f6bbc47aeba808f7f7e2c6d66f5c3271cd9960ecef2fe0d0f749498
6fa3d8b2bb01963537e60efb13981e138e3d4a141f55f0bef355cfb34ccd468fdacad
75865ee7efef95f4cb6c25d477f720502601bd116f3cb70f03d9cbc25d5606b8c5764
a1b9b11b28f2cbbd5630a836f9dbb2e7e8914639bca0fb9c99d58d42dfe3057cfe881
491b1d0812948aa4a7a7c7f7
KE3: 93a3c0da12392ad5336962e340b7c44ed445a67d61dae7bc5e2ccf891f6e9fac
6596f93350d3d559f7bbb182376a07ec7377e7966b7cc549fb8d0b6fb575f157
export_key: 9dec51d6d0f6ce7e4345f10961053713b07310cc2e45872f57bbd2fe5
070fdf0fb5b77c7ddaa2f3dc5c35132df7417ad7fefe0f690ad266e5a54a21d045c9c
38
session_key: a30c5ad775d0bc9aeb47757cc68a9332cb3acb7fc332ca07ba96d707
1847e492c8de3541997f97a605eb9b60cde6feb8dc57322e60ec42ae78c87a2e215e2
77f
~~~

### OPAQUE-3DH Real Test Vector 4

#### Configuration

~~~
OPRF: ristretto255-SHA512
Hash: SHA512
KSF: Identity
KDF: HKDF-SHA512
MAC: HMAC-SHA512
Group: curve25519
Context: 4f50415155452d504f43
Nh: 64
Npk: 32
Nsk: 32
Nm: 64
Nx: 64
Nok: 32
~~~

#### Input Values

~~~
client_identity: 616c696365
server_identity: 626f62
oprf_seed: a78342ab84d3d30f08d5a9630c79bf311c31ed7f85d9d4959bf492ec67
a0eec8a67dfbf4497248eebd49e878aab173e5e4ff76354288fdd53e949a5f7c9f7f1
b
credential_identifier: 31323334
password: 436f7272656374486f72736542617474657279537461706c65
envelope_nonce: 40d6b67fdd7da7c49894750754514dbd2070a407166bd2a5237cc
a9bf44d6e0b
masking_nonce: 38fe59af0df2c79f57b8780278f5ae47355fe1f817119041951c80
f612fdfc6d
server_private_key: c06139381df63bfc91c850db0b9cfbec7a62e86d80040a41a
a7725bf0e79d564
server_public_key: a41e28269b4e97a66468cc00c5a57753e192e1527669897706
88aa90486ef031
server_nonce: 71cd9960ecef2fe0d0f7494986fa3d8b2bb01963537e60efb13981e
138e3d4a1
client_nonce: da7e07376d6d6f034cfa9bb537d11b8c6b4238c334333d1f0aebb38
0cae6a6cc
server_public_keyshare: 41f55f0bef355cfb34ccd468fdacad75865ee7efef95f
4cb6c25d477f7205026
client_public_keyshare: 10a83b9117d3798cb2957fbdb0268a0d63dbf9d66bde5
c00c78affd80026c911
server_private_keyshare: 00a4f54206eef1ba2f615bc0aa285cb22f26d1153b5b
40a1e85ff80da12f986f
client_private_keyshare: 80850a697b42a505f5b68fcdafce8c31f0af2b581f06
3cf1091933541936304b
blind_registration: c575731ffe1cb0ca5ba63b42c4699767b8b9ab78ba39316ee
04baddb2034a70a
blind_login: 6ecc102d2e7a7cf49617aad7bbe188556792d4acd60a1a8a8d2b65d4
b0790308
~~~

#### Intermediate Values

~~~
client_public_key: 0936ea94ab030ec332e29050d266c520e916731a052d05ced7
e0cfe751142b48
auth_key: 7e880ab484f750e80e6f839d975aff476070ce65066d85ea62523d1d576
4739d91307fac47186a4ab935e6a5c7f70cb47faa9473311947502c022cc67ae9440c
randomized_password: 3a602c295a9c323d9362fe286f104567ed6862b25dbe30fa
da844f19e41cf40047424b7118e15dc2c1a815a70fea5c8de6c30aa61440cd4b4b5e8
f3963fbb2e1
envelope: 40d6b67fdd7da7c49894750754514dbd2070a407166bd2a5237cca9bf44
d6e0bb4c0eab6143959a650c5f6b32acf162b1fbe95bb36c5c4f99df53865c4d3537d
69061d80522d772cd0efdbe91f817f6bf7259a56e20b4eb9cbe9443702f4b759
handshake_secret: f5b8fa77a67e638114899eca80c3549aa2c8e277a3412bccbe0
a7e3943a5798d1e5ede2a847144759b17eb253f2f65efcccf82fe7b5f26e17175713d
be845786
server_mac_key: a8cdc5647342743be8ae6fc51e7105651b16bcb5fe4913834e7c2
139dbc06d84c75215e7e84e1785f431c925844eb8c9c0d14959239422368166f41485
e7847a
client_mac_key: b326230809ee373101e8b387aa33a865f7afa375f8c3e5a8fb592
b2e89e3117ccae85b0440421c75eb38a4ca4bde9355a549179b84748e21a3e378a3eb
538e37
oprf_key: 62ef7f7d9506a14600c34f642aaf6ef8019cc82a6755db4fded5248ea14
6030a
~~~

#### Output Values

~~~
registration_request: 26f3dbfd76b8e5f85b4da604f42889a7d4b1bc919f65538
1a67de02c59fd5436
registration_response: 506e8f1b89c098fb89b5b6210a05f7898cafdaea221761
e8d5272fc39e0f9f08a41e28269b4e97a66468cc00c5a57753e192e15276698977068
8aa90486ef031
registration_upload: 0936ea94ab030ec332e29050d266c520e916731a052d05ce
d7e0cfe751142b486d23c6ed818882f9bdfdcf91389fcbc0b7a3faf92bd0bd6be4a1e
7730277b694fc7c6ba327fbe786af18487688e0f7c148bbd54dc2fc80c28e7a976d9e
f53c3540d6b67fdd7da7c49894750754514dbd2070a407166bd2a5237cca9bf44d6e0
bb4c0eab6143959a650c5f6b32acf162b1fbe95bb36c5c4f99df53865c4d3537d6906
1d80522d772cd0efdbe91f817f6bf7259a56e20b4eb9cbe9443702f4b759
KE1: c4dedb0ba6ed5d965d6f250fbe554cd45cba5dfcce3ce836e4aee778aa3cd44d
da7e07376d6d6f034cfa9bb537d11b8c6b4238c334333d1f0aebb380cae6a6cc10a83
b9117d3798cb2957fbdb0268a0d63dbf9d66bde5c00c78affd80026c911
KE2: 9a0e5a1514f62e005ea098b0d8cf6750e358c4389e6add1c52aed9500fa19d00
38fe59af0df2c79f57b8780278f5ae47355fe1f817119041951c80f612fdfc6d22cc3
1127d6f0096755be3c3d2dd6287795c317aeea10c9485bf4f419a786642c19a8f151c
eb5e8767d175248c62c017de94057398d28bf0ed00d1b50ee4f812699bff7663be3c5
d59de94d8e7e58817c7da005b39c25d25555c929e1c5cf6c1b82837b1367c839aab56
a422c0d97719426a79a16f9869cf852100597b23b5a071cd9960ecef2fe0d0f749498
6fa3d8b2bb01963537e60efb13981e138e3d4a141f55f0bef355cfb34ccd468fdacad
75865ee7efef95f4cb6c25d477f7205026816ce1eb529f8f3c6cec676d8c08f5ca760
c4322016850f329c4a2fb07364768a11a5380564d4cbceae511c873627c22c9ee9f05
488278de0fcf646f0825efdd
KE3: f6325cf6a7bd808fca69d54546ef61f2b5ec62fe8b96ca3c9b3e054841ab35ae
552bc5d8eea5ec840f56578be2cd2c30b52fa03266a2f4518cf764d9ced467ec
export_key: 9dec51d6d0f6ce7e4345f10961053713b07310cc2e45872f57bbd2fe5
070fdf0fb5b77c7ddaa2f3dc5c35132df7417ad7fefe0f690ad266e5a54a21d045c9c
38
session_key: 867930cba2b9988acfbe8289bffff728f7c799153be737c8a915ed98
647a7bc7a7f5f9c03deb797d36eda54de6015683a60e08f34e746e37514c4714d0644
99f
~~~

### OPAQUE-3DH Real Test Vector 5

#### Configuration

~~~
OPRF: P256-SHA256
Hash: SHA256
KSF: Identity
KDF: HKDF-SHA256
MAC: HMAC-SHA256
Group: P256_XMD:SHA-256_SSWU_RO_
Context: 4f50415155452d504f43
Nh: 32
Npk: 33
Nsk: 32
Nm: 32
Nx: 32
Nok: 32
~~~

#### Input Values

~~~
oprf_seed: 62f60b286d20ce4fd1d64809b0021dad6ed5d52a2c8cf27ae6582543a0
a8dce2
credential_identifier: 31323334
password: 436f7272656374486f72736542617474657279537461706c65
envelope_nonce: a921f2a014513bd8a90e477a629794e89fec12d12206dde662ebd
cf65670e51f
masking_nonce: 38fe59af0df2c79f57b8780278f5ae47355fe1f817119041951c80
f612fdfc6d
server_private_key: c36139381df63bfc91c850db0b9cfbec7a62e86d80040a41a
a7725bf0e79d5e5
server_public_key: 035f40ff9cf88aa1f5cd4fe5fd3da9ea65a4923a5594f84fd9
f2092d6067784874
server_nonce: 71cd9960ecef2fe0d0f7494986fa3d8b2bb01963537e60efb13981e
138e3d4a1
client_nonce: ab3d33bde0e93eda72392346a7a73051110674bbf6b1b7ffab8be4f
91fdaeeb1
server_public_keyshare: 020e67941e94deba835214421d2d8c90de9b0f7f925d1
1e2032ce19b1832ae8e0f
client_public_keyshare: 03493f36ca12467d1f5eaaabea67ca31377c4869c1e9a
62346b6f01a991624b95d
server_private_keyshare: 9addab838c920fa7044f3a46b91ecaea24b0e7203992
8ee7d4c37a5b9bc17349
client_private_keyshare: 89d5a7e18567f255748a86beac13913df755a5adf776
d69e143147b545d22134
blind_registration: 411bf1a62d119afe30df682b91a0a33d777972d4f2daa4b34
ca527d597078153
blind_login: c497fddf6056d241e6cf9fb7ac37c384f49b357a221eb0a802c989b9
942256c1
~~~

#### Intermediate Values

~~~
client_public_key: 02dc91b178ba2c4bbf9b9403fca25457b906a7f507e59b6e70
3031e09114ba2be0
auth_key: 5bd4be1602516092dc5078f8d699f5721dc1720a49fb80d8e5c16377abd
0987b
randomized_password: 06be0a1a51d56557a3adad57ba29c5510565dcd8b5078fa3
19151b9382258fb0
envelope: a921f2a014513bd8a90e477a629794e89fec12d12206dde662ebdcf6567
0e51fe155412cb432898eda63529c3b2633521f770cccbd25d7548a4e20665a45e65a
handshake_secret: c59197dd9269abfdb3037ea1c203a97627e2c0aa142000d1c3f
06a2c8713077d
server_mac_key: a431a5c1d3cb5772cbc66af0c2851e23dd9ad153a0c8b99081c7d
0d543173fde
client_mac_key: 7329ffd54df21db5532fce8794fca78b505fef9397aad28a424f6
ea3f97c51ca
oprf_key: 2dfb5cb9aa1476093be74ca0d43e5b02862a05f5d6972614d7433acdc66
f7f31
~~~

#### Output Values

~~~
registration_request: 029e949a29cfa0bf7c1287333d2fb3dc586c41aa652f507
0d26a5315a1b50229f8
registration_response: 0350d3694c00978f00a5ce7cd08a00547e4ab5fb5fc2b2
f6717cdaa6c89136efef035f40ff9cf88aa1f5cd4fe5fd3da9ea65a4923a5594f84fd
9f2092d6067784874
registration_upload: 02dc91b178ba2c4bbf9b9403fca25457b906a7f507e59b6e
703031e09114ba2be07f0ed53532d3ae8e505ecc70d42d2b814b6b0e48156def71ea0
29148b2803aafa921f2a014513bd8a90e477a629794e89fec12d12206dde662ebdcf6
5670e51fe155412cb432898eda63529c3b2633521f770cccbd25d7548a4e20665a45e
65a
KE1: 037342f0bcb3ecea754c1e67576c86aa90c1de3875f390ad599a26686cdfee6e
07ab3d33bde0e93eda72392346a7a73051110674bbf6b1b7ffab8be4f91fdaeeb1034
93f36ca12467d1f5eaaabea67ca31377c4869c1e9a62346b6f01a991624b95d
KE2: 0246da9fe4d41d5ba69faa6c509a1d5bafd49a48615a47a8dd4b0823cc147648
1138fe59af0df2c79f57b8780278f5ae47355fe1f817119041951c80f612fdfc6d2f0
c547f70deaeca54d878c14c1aa5e1ab405dec833777132eea905c2fbb12504a67dcbe
0e66740c76b62c13b04a38a77926e19072953319ec65e41f9bfd2ae2687bd3348bfe3
3cb0bb9864fdb3b307f7dd68a17f3f150074a0bfc830ab889717d71cd9960ecef2fe0
d0f7494986fa3d8b2bb01963537e60efb13981e138e3d4a1020e67941e94deba83521
4421d2d8c90de9b0f7f925d11e2032ce19b1832ae8e0fb5166145361a2c344d9737dd
5c826fede3bbfafa418ad379ce4fa65fbb15db6e
KE3: 272d04758b2b436bf0239ba7b9bd0a1686a9b6542ceaaf08732054beda956498
export_key: c3c9a1b0e33ac84dd83d0b7e8af6794e17e7a3caadff289fbd9dc769a
853c64b
session_key: a224790a010afc0a3f37e23c1b7a5cb7f9e73e3d9a924116510d97d8
0e2a1e0c
~~~

### OPAQUE-3DH Real Test Vector 6

#### Configuration

~~~
OPRF: P256-SHA256
Hash: SHA256
KSF: Identity
KDF: HKDF-SHA256
MAC: HMAC-SHA256
Group: P256_XMD:SHA-256_SSWU_RO_
Context: 4f50415155452d504f43
Nh: 32
Npk: 33
Nsk: 32
Nm: 32
Nx: 32
Nok: 32
~~~

#### Input Values

~~~
client_identity: 616c696365
server_identity: 626f62
oprf_seed: 62f60b286d20ce4fd1d64809b0021dad6ed5d52a2c8cf27ae6582543a0
a8dce2
credential_identifier: 31323334
password: 436f7272656374486f72736542617474657279537461706c65
envelope_nonce: a921f2a014513bd8a90e477a629794e89fec12d12206dde662ebd
cf65670e51f
masking_nonce: 38fe59af0df2c79f57b8780278f5ae47355fe1f817119041951c80
f612fdfc6d
server_private_key: c36139381df63bfc91c850db0b9cfbec7a62e86d80040a41a
a7725bf0e79d5e5
server_public_key: 035f40ff9cf88aa1f5cd4fe5fd3da9ea65a4923a5594f84fd9
f2092d6067784874
server_nonce: 71cd9960ecef2fe0d0f7494986fa3d8b2bb01963537e60efb13981e
138e3d4a1
client_nonce: ab3d33bde0e93eda72392346a7a73051110674bbf6b1b7ffab8be4f
91fdaeeb1
server_public_keyshare: 020e67941e94deba835214421d2d8c90de9b0f7f925d1
1e2032ce19b1832ae8e0f
client_public_keyshare: 03493f36ca12467d1f5eaaabea67ca31377c4869c1e9a
62346b6f01a991624b95d
server_private_keyshare: 9addab838c920fa7044f3a46b91ecaea24b0e7203992
8ee7d4c37a5b9bc17349
client_private_keyshare: 89d5a7e18567f255748a86beac13913df755a5adf776
d69e143147b545d22134
blind_registration: 411bf1a62d119afe30df682b91a0a33d777972d4f2daa4b34
ca527d597078153
blind_login: c497fddf6056d241e6cf9fb7ac37c384f49b357a221eb0a802c989b9
942256c1
~~~

#### Intermediate Values

~~~
client_public_key: 02dc91b178ba2c4bbf9b9403fca25457b906a7f507e59b6e70
3031e09114ba2be0
auth_key: 5bd4be1602516092dc5078f8d699f5721dc1720a49fb80d8e5c16377abd
0987b
randomized_password: 06be0a1a51d56557a3adad57ba29c5510565dcd8b5078fa3
19151b9382258fb0
envelope: a921f2a014513bd8a90e477a629794e89fec12d12206dde662ebdcf6567
0e51f4d7773a36a208a866301dbb2858e40dc5638017527cf91aef32d3848eebe0971
handshake_secret: 0ee4a82c4a34992f72bfbcb5d2ce64044477dfe200b9d8c92bf
1759b219b3485
server_mac_key: 77ebd7511216a51e9c2f3368ce6c1e40513f24b6f42085ef18e7f
737b427aab5
client_mac_key: e48e2064cf570dbd18eb42550d4459c58ac4ae4e28881d1aefbab
d668f7f1df9
oprf_key: 2dfb5cb9aa1476093be74ca0d43e5b02862a05f5d6972614d7433acdc66
f7f31
~~~

#### Output Values

~~~
registration_request: 029e949a29cfa0bf7c1287333d2fb3dc586c41aa652f507
0d26a5315a1b50229f8
registration_response: 0350d3694c00978f00a5ce7cd08a00547e4ab5fb5fc2b2
f6717cdaa6c89136efef035f40ff9cf88aa1f5cd4fe5fd3da9ea65a4923a5594f84fd
9f2092d6067784874
registration_upload: 02dc91b178ba2c4bbf9b9403fca25457b906a7f507e59b6e
703031e09114ba2be07f0ed53532d3ae8e505ecc70d42d2b814b6b0e48156def71ea0
29148b2803aafa921f2a014513bd8a90e477a629794e89fec12d12206dde662ebdcf6
5670e51f4d7773a36a208a866301dbb2858e40dc5638017527cf91aef32d3848eebe0
971
KE1: 037342f0bcb3ecea754c1e67576c86aa90c1de3875f390ad599a26686cdfee6e
07ab3d33bde0e93eda72392346a7a73051110674bbf6b1b7ffab8be4f91fdaeeb1034
93f36ca12467d1f5eaaabea67ca31377c4869c1e9a62346b6f01a991624b95d
KE2: 0246da9fe4d41d5ba69faa6c509a1d5bafd49a48615a47a8dd4b0823cc147648
1138fe59af0df2c79f57b8780278f5ae47355fe1f817119041951c80f612fdfc6d2f0
c547f70deaeca54d878c14c1aa5e1ab405dec833777132eea905c2fbb12504a67dcbe
0e66740c76b62c13b04a38a77926e19072953319ec65e41f9bfd2ae268d7f10604202
1c80300e4c6f585980cf39fc51a4a6bba41b0729f9b240c729e5671cd9960ecef2fe0
d0f7494986fa3d8b2bb01963537e60efb13981e138e3d4a1020e67941e94deba83521
4421d2d8c90de9b0f7f925d11e2032ce19b1832ae8e0fdca637d2a5390f4c809a67b4
6977c536fe9f643f703178a17a413d14e4bb523c
KE3: 298cd0077d018f122bc95d706e5fef06537814c567f08d5e40b0c0ae918f9287
export_key: c3c9a1b0e33ac84dd83d0b7e8af6794e17e7a3caadff289fbd9dc769a
853c64b
session_key: 0c59872e9bcdde274f4f52f6ba0fd1acca211d6eb7db98677b457a73
9ef1f0d8
~~~

## Fake Test Vectors {#fake-vectors}

### OPAQUE-3DH Fake Test Vector 1

#### Configuration

~~~
OPRF: ristretto255-SHA512
Hash: SHA512
KSF: Identity
KDF: HKDF-SHA512
MAC: HMAC-SHA512
Group: ristretto255
Context: 4f50415155452d504f43
Nh: 64
Npk: 32
Nsk: 32
Nm: 64
Nx: 64
Nok: 32
~~~

#### Input Values

~~~
client_identity: 616c696365
server_identity: 626f62
oprf_seed: 743fc168d1f826ad43738933e5adb23da6fb95f95a1b069f0daa0522d0
a78b617f701fc6aa46d3e7981e70de7765dfcd6b1e13e3369a582eb8dc456b10aa53b
0
credential_identifier: 31323334
masking_nonce: 9c035896a043e70f897d87180c543e7a063b83c1bb728fbd189c61
9e27b6e5a6
client_private_key: 2b98980aa95ab53a0f39f0291903d2fdf04b00c167f081416
9922df873002409
client_public_key: 84f43f9492e19c22d8bdaa4447cc3d4db1cdb5427a9f852c47
07921212c36251
server_private_key: c788585ae8b5ba2942b693b849be0c0426384e41977c18d2e
81fbe30fd7c9f06
server_public_key: 825f832667480f08b0c9069da5083ac4d0e9ee31b49c4e0310
031fea04d52966
server_nonce: 1e10f6eeab2a7a420bf09da9b27a4639645622c46358de9cf7ae813
055ae2d12
server_public_keyshare: 5236e2e06d49f0b496db2a786f6ee1016f15b4fd6c0db
d95d6b117055d914157
server_private_keyshare: 6d8fba9741a357584770f85294430bce2252fe212a8a
372152a73c7ffe414503
masking_key: 39ebd51f0e39a07a1c2d2431995b0399bca9996c5d10014d6ebab445
3dc10ce5cef38ed3df6e56bfff40c2d8dd4671c2b4cf63c3d54860f31fe40220d690b
b71
KE1: b0a26dcaca2230b8f5e4b1bcab9c84b586140221bb8b2848486874b0be448905
42d4e61ed3f8d64cdd3b9d153343eca15b9b0d5e388232793c6376bd2d9cfd0a0e4ed
8bcc15f3dd01a30365c97c0c0de0a3dd3fbf5d3cbec55fb6ac1d3bf740f
~~~

#### Output Values

~~~
KE2: 928f79ad8df21963e91411b9f55165ba833dea918f441db967cdc09521d22925
9c035896a043e70f897d87180c543e7a063b83c1bb728fbd189c619e27b6e5a632b5a
b1bff96636144faa4f9f9afaac75dd88ea99cf5175902ae3f3b2195693f165f11929b
a510a5978e64dcdabecbd7ee1e4380ce270e58fea58e6462d92964a1aaef72698bca1
c673baeb04cc2bf7de5f3c2f5553464552d3a0f7698a9ca7f9c5e70c6cb1f706b2f17
5ab9d04bbd13926e816b6811a50b4aafa9799d5ed7971e10f6eeab2a7a420bf09da9b
27a4639645622c46358de9cf7ae813055ae2d125236e2e06d49f0b496db2a786f6ee1
016f15b4fd6c0dbd95d6b117055d914157cb5e11625c701e642293ad32bfcf88da653
c9b6e71efc8a89607fd46ed5e7b9bf7cc7dbb997a4fd41194a04bcd0c5d88052e080a
2f02c68d8d9e9c0ce15c92ff
~~~

### OPAQUE-3DH Fake Test Vector 2

#### Configuration

~~~
OPRF: ristretto255-SHA512
Hash: SHA512
KSF: Identity
KDF: HKDF-SHA512
MAC: HMAC-SHA512
Group: curve25519
Context: 4f50415155452d504f43
Nh: 64
Npk: 32
Nsk: 32
Nm: 64
Nx: 64
Nok: 32
~~~

#### Input Values

~~~
client_identity: 616c696365
server_identity: 626f62
oprf_seed: 66e650652a8266b2205f31fdd68adeb739a05b5e650b19e7edc75e734a
1296d6088188ca46c31ae8ccbd42a52ed338c06e53645387a7efbc94b6a0449526155
e
credential_identifier: 31323334
masking_nonce: 9c035896a043e70f897d87180c543e7a063b83c1bb728fbd189c61
9e27b6e5a6
client_private_key: 288bf63470199221847bb035d99f96531adf8badd14cb1571
b48f7a506649660
client_public_key: 3c64a3153854cc9f0c23aab3c1a19106ec8bab4730736d1d00
3880a1d5a59005
server_private_key: 30fbe7e830be1fe8d2187c97414e3826040cbe49b893b6422
9bab5e85a588846
server_public_key: 78b3040047ff26572a7619617601a61b9c81899bee92f00cfc
aa5eed96863555
server_nonce: 1e10f6eeab2a7a420bf09da9b27a4639645622c46358de9cf7ae813
055ae2d12
server_public_keyshare: 2d9055eb8f83e1b497370adad5cc2a417bf9be436a792
def0c7b7ccb92b9e275
server_private_keyshare: 300b0937f47d45f6123a4d8f0d0c0814b6120d840ebb
8bc5b4f6b62df07f7842
masking_key: 79ad2621b0757a447dff7108a8ae20a068ce67872095620f415ea611
c9dcc04972fa359538cd2fd6528775ca775487b2b56db642049b8a90526b975a38484
c6a
KE1: b0a26dcaca2230b8f5e4b1bcab9c84b586140221bb8b2848486874b0be448905
42d4e61ed3f8d64cdd3b9d153343eca15b9b0d5e388232793c6376bd2d9cfd0ac059b
7ba2aec863933ae48816360c7a9022e83d822704f3b0b86c0502a66e574
~~~

#### Output Values

~~~
KE2: 6606b6fedbb33f19a81a1feb5149c600fe77252f58acd3080d7504d3dad4922f
9c035896a043e70f897d87180c543e7a063b83c1bb728fbd189c619e27b6e5a67db39
8c0f65d8c298eac430abdae4c80e82b552fb940c00f0cbcea853c0f96c1c15099f3d4
b0e83ecc249613116d605b8d77bb68bdf76994c2bc507e2dcae4176f00afed68ad25c
f3040a0e991acece31ca532117f5c12816997372ff031ad04ebcdce06c501da24e7b4
db95343456e2ed260895ec362694230a1fa20e24a9c71e10f6eeab2a7a420bf09da9b
27a4639645622c46358de9cf7ae813055ae2d122d9055eb8f83e1b497370adad5cc2a
417bf9be436a792def0c7b7ccb92b9e27513c6a0d5d96e939563ad168990ed0156b8d
8fb82888ce111f217b1103b4c6d67ee9738777033dd0d85e39776993b5f1f7fa4252b
13a9c37c0fdd06204ca315c6
~~~

### OPAQUE-3DH Fake Test Vector 3

#### Configuration

~~~
OPRF: P256-SHA256
Hash: SHA256
KSF: Identity
KDF: HKDF-SHA256
MAC: HMAC-SHA256
Group: P256_XMD:SHA-256_SSWU_RO_
Context: 4f50415155452d504f43
Nh: 32
Npk: 33
Nsk: 32
Nm: 32
Nx: 32
Nok: 32
~~~

#### Input Values

~~~
client_identity: 616c696365
server_identity: 626f62
oprf_seed: bb1cd59e16ac09bc0cb6d528541695d7eba2239b1613a3db3ade77b362
80f725
credential_identifier: 31323334
masking_nonce: 9c035896a043e70f897d87180c543e7a063b83c1bb728fbd189c61
9e27b6e5a6
client_private_key: d423b87899fc61d014fc8330a4e26190fcfa470a3afe59243
24294af7dbbc1dd
client_public_key: 03b81708eae026a9370616c22e1e8542fe9dbebd36ce8a2661
b708e9628f4a57fc
server_private_key: 34fbe7e830be1fe8d2187c97414e3826040cbe49b893b6422
9bab5e85a5888c7
server_public_key: 0221e034c0e202fe883dcfc96802a7624166fed4cfcab4ae30
cf5f3290d01c88bf
server_nonce: 1e10f6eeab2a7a420bf09da9b27a4639645622c46358de9cf7ae813
055ae2d12
server_public_keyshare: 03f42965d5bcba2a590a49eb2418061effe40b5c29a34
b8e5163e0ef32044b2e4c
server_private_keyshare: 1a2a0ff27f3ca75221378a2a21fe5222ce0b439452f8
70475857a34197ba8f6d
masking_key: caecc6ccb4cae27cb54d8f3a1af1bac52a3d53107ce08497cdd362b1
992e4e5e
KE1: 0396875da2b4f7749bba411513aea02dc514a48d169d8a9531bd61d3af3fa9ba
ae42d4e61ed3f8d64cdd3b9d153343eca15b9b0d5e388232793c6376bd2d9cfd0a039
94d4f1221bfd205063469e92ea4d492f7cc76a327223633ab74590c30cf7285
~~~

#### Output Values

~~~
KE2: 0201198dcd13f9792eb75dcfa815f61b049abfe2e3e9456d4bbbceec5f442efd
049c035896a043e70f897d87180c543e7a063b83c1bb728fbd189c619e27b6e5a6fac
da65ce0a97b9085e7af07f61fd3fdd046d257cbf2183ce8766090b8041a8bf28d79dd
4c9031ddc75bb6ddb4c291e639937840e3d39fc0d5a3d6e7723c09f7945df485bcf9a
efe3fe82d149e84049e259bb5b33d6a2ff3b25e4bfb7eff0962821e10f6eeab2a7a42
0bf09da9b27a4639645622c46358de9cf7ae813055ae2d1203f42965d5bcba2a590a4
9eb2418061effe40b5c29a34b8e5163e0ef32044b2e4c196137813ed8ec48627f0b0d
90d9427f4ec137f8360769df167c25836eae5d91
~~~
"#;
