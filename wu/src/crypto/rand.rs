//! Random

use rand::{distributions::Alphanumeric, thread_rng, Rng};

/// Generate random vector
pub fn random(size: usize) -> Vec<u8> {
    let mut rng = thread_rng();
    (0..size).map(|_| rng.gen()).collect()
}

/// Generate random alphanumeric string
pub fn random_an(len: usize) -> String {
    let rand_an = thread_rng().sample_iter(&Alphanumeric).take(len).collect();
    String::from_utf8(rand_an).unwrap()
}
