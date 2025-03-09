//! Argon2 password hashing

use argon2::{Config, Variant, hash_encoded, verify_encoded};
use kern::{Fail, Result};

/// Generate Argon2 password hash
pub fn argon2_hash(pwd: impl AsRef<[u8]>, salt: impl AsRef<[u8]>) -> Result<String> {
    let config = Config {
        variant: Variant::Argon2id,
        ..Default::default()
    };
    hash_encoded(pwd.as_ref(), salt.as_ref(), &config).or_else(Fail::from)
}

/// Verify Argon2 password hash
pub fn argon2_verify(encoded: impl AsRef<str>, pwd: impl AsRef<[u8]>) -> bool {
    verify_encoded(encoded.as_ref(), pwd.as_ref()).unwrap_or(false)
}
