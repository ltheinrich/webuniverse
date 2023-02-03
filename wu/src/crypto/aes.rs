//! AES256-GCM encryption/decryption

use crate::{Fail, Result};
use aes_gcm::aead::generic_array::{
    typenum::bit::{B0, B1},
    typenum::uint::{UInt, UTerm},
    GenericArray,
};
use aes_gcm::aead::{Aead, KeyInit};
pub use aes_gcm::Aes256Gcm;

/// AES 256-bit key
pub type Aes256Key =
    GenericArray<u8, UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B0>, B0>>;

/// AES 12-byte nonce
pub type AesNonce = GenericArray<u8, UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B0>>;

/// Intialize Aes256Gcm with key
pub fn init_aead(key: impl AsRef<[u8]>) -> Aes256Gcm {
    let key = GenericArray::clone_from_slice(key.as_ref());
    Aes256Gcm::new(&key)
}

/// AES256-GCM encryption/decryption
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
    pub fn encrypt(&self, data: impl AsRef<[u8]>) -> Result<Vec<u8>> {
        self.aead
            .encrypt(&self.nonce, data.as_ref())
            .or_else(|err| Fail::from(format!("failed to encrypt: {err:?}")))
    }

    /// Decrypt data
    pub fn decrypt(&self, data: impl AsRef<[u8]>) -> Result<Vec<u8>> {
        self.aead
            .decrypt(&self.nonce, data.as_ref())
            .or_else(|err| Fail::from(format!("failed to decrypt: {err:?}")))
    }

    /// Get AEAD
    pub fn aead(&self) -> &Aes256Gcm {
        self.aead
    }
}
