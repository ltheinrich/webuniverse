//! Server management

use std::convert::TryInto;
use std::sync::{RwLock, RwLockReadGuard, RwLockWriteGuard};
use wu::crypto::Aes256Gcm;
use wu::net::{ConnBuilder, Connection};
use wu::Fail;

/// Server builder
pub struct ServerBuilder<'a> {
    conn: Connection<'a>,
}

impl<'a> ServerBuilder<'a> {
    /// Create new server and manager
    pub fn new(conn: Connection<'a>) -> Self {
        Self { conn }
    }

    /// Build server and manager
    pub fn build(mut self) -> (Server, Manager<'a>) {
        let port = u16::from_be_bytes(self.conn.read().unwrap().as_slice().try_into().unwrap());
        let server = Server {
            data: RwLock::new(String::new()),
            addr: format!("{}:{}", self.conn.stream_ip(), port),
            aead: self.conn.crypter_aead().clone(),
        };

        let manager = Manager { conn: self.conn };
        (server, manager)
    }
}

/// Server representation
pub struct Server {
    data: RwLock<String>,
    addr: String,
    aead: Aes256Gcm,
}

impl Server {
    /// Get console data read-only
    pub fn data(&self) -> RwLockReadGuard<'_, String> {
        self.data.read().unwrap()
    }

    /// Get console data writeable
    pub fn data_mut(&self) -> RwLockWriteGuard<'_, String> {
        self.data.write().unwrap()
    }

    /// Send command to server
    pub fn cmd(&self, cmd: String) -> Result<(), Fail> {
        let mut conn = ConnBuilder::new(&self.addr, &self.aead)?.init()?;
        conn.write(cmd)
    }
}

/// Server manager
pub struct Manager<'a> {
    conn: Connection<'a>,
}

impl<'a> Manager<'a> {
    /// Connection to server
    pub fn conn(&mut self) -> &mut Connection<'a> {
        &mut self.conn
    }
}
