//! Webuniverse API

use wu::kern::meta::init_version;

static CARGO_TOML: &str = include_str!("../Cargo.toml");

fn main() {
    println!(
        "Webuniverse API {} (c) 2020 Lennart Heinrich",
        init_version(CARGO_TOML)
    );
}
