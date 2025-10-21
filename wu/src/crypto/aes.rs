//! AES256-GCM encryption/decryption

use crate::{Fail, Result};
pub use aes_gcm::Aes256Gcm;
use aes_gcm::aead::array::{
    Array,
    typenum::bit::{B0, B1},
    typenum::uint::{UInt, UTerm},
};
use aes_gcm::aead::{Aead, KeyInit};

/// AES 256-bit key
pub type Aes256Key = Array<u8, UInt<UInt<UInt<UInt<UInt<UInt<UTerm, B1>, B0>, B0>, B0>, B0>, B0>>;

/// AES 12-byte nonce
pub type AesNonce = Array<u8, UInt<UInt<UInt<UInt<UTerm, B1>, B1>, B0>, B0>>;

/// Intialize Aes256Gcm with key
pub fn init_aead(key: impl AsRef<[u8]>) -> Result<Aes256Gcm> {
    let key = key.as_ref().try_into()?;
    Ok(Aes256Gcm::new(&key))
}

/// AES256-GCM encryption/decryption
pub struct Crypter<'a> {
    aead: &'a Aes256Gcm,
    nonce: AesNonce,
}

impl<'a> Crypter<'a> {
    /// Create new crypter from existing aead
    pub fn new(aead: &'a Aes256Gcm, nonce: impl AsRef<[u8]>) -> Result<Self> {
        Ok(Self {
            aead,
            nonce: nonce.as_ref().try_into()?,
        })
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
