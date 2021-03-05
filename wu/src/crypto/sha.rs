//! SHA3-256 hashing

pub use hex::{decode as hex_decode, encode as hex_encode};
use sha3::{Digest, Sha3_256};

/// Generate password hash for API usage -> sha3-256(webuniverse + sha3-256(username + sha3-256(password))
pub fn hash_password(password: impl AsRef<[u8]>, username: impl AsRef<[u8]>) -> String {
    // init hasher and hash password
    let mut hasher = Sha3_256::new();
    hasher.update(password);
    let mut enc = hex_encode(hasher.finalize());

    // hash the hash with username
    hasher = Sha3_256::new();
    hasher.update(username);
    hasher.update(enc);
    enc = hex_encode(hasher.finalize());

    // hash the hash with webuniverse
    hasher = Sha3_256::new();
    hasher.update(b"webuniverse");
    hasher.update(enc);
    let result = hasher.finalize();

    // return hex encoded
    hex_encode(result)
}

/// SHA3-256 Hash
pub fn hash(plaintext: impl AsRef<[u8]>) -> String {
    let mut hasher = Sha3_256::new();
    hasher.update(plaintext);
    hex_encode(hasher.finalize())
}
