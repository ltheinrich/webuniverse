//! Webuniverse Web
#![cfg(target_arch = "wasm32")]

#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

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
