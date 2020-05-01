//! Client API

use std::convert::TryInto;
use std::io::prelude::*;
use std::net::TcpListener;
use std::sync::Arc;
use std::thread;
use std::time::Duration;
use wu::aes_gcm::aead::{generic_array::GenericArray, Aead, NewAead};
use wu::aes_gcm::Aes256Gcm;
use wu::Fail;

/// Listen for clients
pub fn listen_clients(addr: &str, api_key: &str) -> Result<(), Fail> {
    let listener = TcpListener::bind(addr).or_else(Fail::from)?;
    println!("API server available on {}", addr);
    let key = Arc::new(GenericArray::clone_from_slice(api_key.as_bytes()));
    loop {
        if let Ok((mut stream, _)) = listener.accept() {
            let key = key.clone();
            thread::spawn(move || {
                stream
                    .set_read_timeout(Some(Duration::from_secs(10)))
                    .unwrap();
                let mut first_bytes = vec![0u8; 20];
                stream.read_exact(&mut first_bytes).unwrap();
                let size = usize::from_be_bytes((&first_bytes[..8]).try_into().unwrap());
                let mut buf = vec![0u8; size];
                stream.read_exact(&mut buf).unwrap();
                let aead = Aes256Gcm::new(*key);
                let nonce = GenericArray::from_slice(&first_bytes[8..]);
                let decrypted = aead
                    .decrypt(nonce, buf.as_ref())
                    .expect("decryption failure!");
                println!("{}", String::from_utf8(decrypted).unwrap());
            });
        }
    }
}
