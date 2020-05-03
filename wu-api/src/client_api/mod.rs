//! Client API

pub mod server;

use crate::client_api::server::ServerBuilder;
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
            let aead = aead.clone();
            let shared = shared.clone();

            thread::spawn(move || {
                let conn = ConnBuilder::from(stream, &aead).accept().unwrap();
                let (server, mut manager) = ServerBuilder::new(conn).build();
                let name = String::from_utf8(manager.conn().read().unwrap()).unwrap();

                {
                    let shared = shared.write().unwrap();
                    let mut servers = shared.servers_mut();
                    servers.insert(name.clone(), server);
                    // drop write-access
                }

                while let Ok(data) = manager.conn().read() {
                    let shared = shared.read().unwrap();
                    let servers = shared.servers();
                    let mut server_data = servers.get(&name).unwrap().data_mut();
                    server_data.push_str(&String::from_utf8_lossy(&data));
                }

                let shared = shared.write().unwrap();
                shared.servers_mut().remove(&name);
            });
        }
    }
}
