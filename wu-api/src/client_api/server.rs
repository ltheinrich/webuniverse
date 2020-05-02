//! Server management

use crossbeam_channel::{unbounded, Receiver, Sender};
use std::sync::{RwLock, RwLockReadGuard, RwLockWriteGuard};
use wu::net::Connection;
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
    pub fn build(self) -> (Server, Manager<'a>) {
        let (tx, rx) = unbounded();
        let server = Server {
            data: RwLock::new(String::new()),
            tx,
        };
        let manager = Manager {
            conn: self.conn,
            rx,
        };
        (server, manager)
    }
}

/// Server representation
pub struct Server {
    data: RwLock<String>,
    tx: Sender<String>,
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
        self.tx.send(cmd).or_else(Fail::from)
    }
}

/// Server manager
pub struct Manager<'a> {
    conn: Connection<'a>,
    rx: Receiver<String>,
}

impl<'a> Manager<'a> {
    /// Connection to server
    pub fn conn(&mut self) -> &mut Connection<'a> {
        &mut self.conn
    }

    /// Command receiver
    pub fn recv(&self) -> Result<String, Fail> {
        self.rx.recv().or_else(Fail::from)
    }
}
