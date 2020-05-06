//! Argon2 password hashing

use argon2::{hash_encoded, verify_encoded, Config, Variant};
use kern::Fail;

/// Generate Argon2 password hash
pub fn argon2_hash(pwd: impl AsRef<[u8]>, salt: impl AsRef<[u8]>) -> Result<String, Fail> {
    let mut config = Config::default();
    config.variant = Variant::Argon2id;
    hash_encoded(pwd.as_ref(), salt.as_ref(), &config).or_else(Fail::from)
}

/// Verify Argon2 password hash
pub fn argon2_verify(encoded: impl AsRef<str>, pwd: impl AsRef<[u8]>) -> bool {
    verify_encoded(encoded.as_ref(), pwd.as_ref()).unwrap_or(false)
}
