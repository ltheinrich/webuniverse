//! Webuniverse library

pub mod crypto;

pub use kern::*;

#[cfg(target_os = "linux")]
pub use aes_gcm;

#[cfg(target_arch = "wasm32")]
pub use wasm_bindgen;
