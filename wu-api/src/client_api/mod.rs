//! Client API

use std::net::TcpListener;
use std::sync::Arc;
use std::thread;
use std::time::Duration;
use wu::crypto::init_aead;
use wu::net::ConnBuilder;
use wu::Fail;

/// Listen for clients
pub fn listen_clients(addr: &str, api_key: &str) -> Result<(), Fail> {
    let listener = TcpListener::bind(addr).or_else(Fail::from)?;
    println!("API server available on {}", addr);
    let aead = Arc::new(init_aead(api_key));
    loop {
        if let Ok((stream, _)) = listener.accept() {
            let aead = aead.clone();
            thread::spawn(move || {
                stream
                    .set_read_timeout(Some(Duration::from_secs(10)))
                    .unwrap();
                let mut conn = ConnBuilder::new(stream, &aead).accept().unwrap();
                while let Ok(recv) = conn.read() {
                    print!("{}", String::from_utf8(recv).unwrap());
                }
            });
        }
    }
}
