//! Random

use rand::{RngExt, distr::Alphanumeric, rng};

/// Generate random vector
pub fn random(size: usize) -> Vec<u8> {
    let mut rng = rng();
    (0..size).map(|_| rng.random()).collect()
}

/// Generate random alphanumeric string
pub fn random_an(len: usize) -> String {
    let rand_an = rng().sample_iter(&Alphanumeric).take(len).collect();
    String::from_utf8(rand_an).unwrap()
}
