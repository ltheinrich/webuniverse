//! Cryptography utils

mod rand;
mod sha;

pub use self::rand::*;
pub use sha::*;

#[cfg(target_os = "linux")]
mod aes;

#[cfg(target_os = "linux")]
pub use aes::*;
