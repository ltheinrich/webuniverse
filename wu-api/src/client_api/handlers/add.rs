//! Add server handler

use crate::client_api::server::ServerBuilder;
use crate::common::*;
use std::sync::RwLockReadGuard;
use wu::net::Connection;

pub fn add_server(conn: Connection, shared: RwLockReadGuard<'_, SharedData>, name: String) {
    // build server
    let (server, mut manager) = ServerBuilder::new(conn).build();

    {
        // add server to map
        let mut servers = shared.servers_mut();
        servers.insert(name.clone(), server);
        // drop write-access
    }

    // read from client
    while let Ok(data) = manager.conn().read() {
        // update server
        let servers = shared.servers();
        let mut server_data = servers.get(&name).unwrap().data_mut();
        server_data.push_str(&String::from_utf8_lossy(&data));
    }

    // remove server
    shared.servers_mut().remove(&name);
}
