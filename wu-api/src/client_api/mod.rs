//! Client API

pub mod server;

mod handlers;

use crate::common::*;
use std::net::TcpListener;
use std::sync::{Arc, RwLock};
use std::thread;
use wu::crypto::init_aead;
use wu::net::ConnBuilder;
use wu::Fail;

/// Listen for clients
pub fn listen_clients(
    addr: &str,
    api_key: &str,
    shared: Arc<RwLock<SharedData>>,
) -> Result<(), Fail> {
    // listen
    let listener = TcpListener::bind(addr).or_else(Fail::from)?;
    let aead = Arc::new(init_aead(api_key));
    println!("API server available on {}", addr);

    loop {
        // accept connections
        if let Ok((stream, _)) = listener.accept() {
            // clone
            let aead = aead.clone();
            let shared = shared.clone();

            thread::spawn(move || {
                // accept connection
                let mut conn = ConnBuilder::from(stream, &aead).accept().unwrap();
                let htype = String::from_utf8(conn.read().unwrap()).unwrap();
                let name = String::from_utf8(conn.read().unwrap()).unwrap();

                // handle
                match htype.as_str() {
                    "add-server" => handlers::add_server(conn, shared.read().unwrap(), name),
                    "send-stats" => handlers::send_stats(conn, shared.read().unwrap(), name),
                    _ => {}
                }
            });
        }
    }
}
