//! Cryptography utils

use crate::Fail;
use aes_gcm::aead::generic_array::{
    typenum::bit::{B0, B1},
    typenum::uint::{UInt, UTerm},
    GenericArray,
};
use aes_gcm::aead::{Aead, NewAead};
use aes_gcm::Aes256Gcm;
pub use hex::{decode as hex_decode, encode as hex_encode};
use rand::{distributions::Alphanumeric, thread_rng, Rng};
use sha3::{Digest, Sha3_256};

/// AES 256-bit key
pub type Aes256Key =
    GenericArray<u8, UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B0>, B0>>;

/// AES 12-byte nonce
pub type AesNonce = GenericArray<u8, UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B0>>;

/// Generate password hash for API usage -> sha3-256(webuniverse + sha3-256(username + sha3-256(password))
pub fn hash_password(password: impl AsRef<[u8]>, username: impl AsRef<[u8]>) -> String {
    // init hasher and hash password
    let mut hasher = Sha3_256::new();
    hasher.input(password);
    let mut enc = hex_encode(hasher.result());

    // hash the hash with username
    hasher = Sha3_256::new();
    hasher.input(username);
    hasher.input(enc);
    enc = hex_encode(hasher.result());

    // hash the hash with webuniverse
    hasher = Sha3_256::new();
    hasher.input(b"webuniverse");
    hasher.input(enc);
    let result = hasher.result();

    // return hex encoded
    hex_encode(result)
}

/// SHA3-256 Hash
pub fn hash(plaintext: impl AsRef<[u8]>) -> String {
    let mut hasher = Sha3_256::new();
    hasher.input(plaintext);
    hex_encode(hasher.result())
}

/// Generate random vector
pub fn random(size: usize) -> Vec<u8> {
    let mut rng = thread_rng();
    (0..size).map(|_| rng.gen()).collect()
}

/// Generate random alphanumeric string
pub fn random_an(len: usize) -> String {
    thread_rng().sample_iter(&Alphanumeric).take(len).collect()
}

/// Intialize Aes256Gcm with key
pub fn init_aead(key: impl AsRef<[u8]>) -> Aes256Gcm {
    let key = GenericArray::clone_from_slice(key.as_ref());
    Aes256Gcm::new(key)
}

pub struct Crypter<'a> {
    aead: &'a Aes256Gcm,
    nonce: AesNonce,
}

impl<'a> Crypter<'a> {
    /// Create new crypter from existing aead
    pub fn new(aead: &'a Aes256Gcm, nonce: impl AsRef<[u8]>) -> Self {
        Self {
            aead,
            nonce: GenericArray::clone_from_slice(nonce.as_ref()),
        }
    }

    /// Encrypt data
    pub fn encrypt(&self, data: impl AsRef<[u8]>) -> Result<Vec<u8>, Fail> {
        self.aead
            .encrypt(&self.nonce, data.as_ref())
            .or_else(|err| Fail::from(format!("failed to encrypt: {:?}", err)))
    }

    /// Decrypt data
    pub fn decrypt(&self, data: impl AsRef<[u8]>) -> Result<Vec<u8>, Fail> {
        self.aead
            .decrypt(&self.nonce, data.as_ref())
            .or_else(|err| Fail::from(format!("failed to decrypt: {:?}", err)))
    }
}
