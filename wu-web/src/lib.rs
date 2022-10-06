//! Webuniverse Web
#![cfg(target_arch = "wasm32")]

use wu::crypto;
use wu::wasm_bindgen::{self, prelude::*};

#[wasm_bindgen]
pub fn set_panic_hook() {
    console_error_panic_hook::set_once();
}

#[wasm_bindgen]
pub fn hash_password(password: &str, username: &str) -> String {
    crypto::hash_password(password, username)
}

#[wasm_bindgen]
pub fn argon2_hash(password: &str, username: &str) -> String {
    let salt = crypto::random(16);
    let password_hash = crypto::hash_password(password, username);
    crypto::argon2_hash(password_hash, salt).unwrap()
}

#[wasm_bindgen]
pub fn str_encode(data: &str) -> String {
    crypto::hex_encode(data)
}

#[wasm_bindgen]
pub fn str_decode(data: &str) -> String {
    let decoded = crypto::hex_decode(data).unwrap();
    String::from_utf8(decoded).unwrap()
}
