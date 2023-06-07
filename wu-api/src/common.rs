//! Common

pub use crate::utils::*;

use crate::api::logins::UserLogins;
use crate::client_api::server::Server;
use crate::data::StorageFile;
use mysql::{Pool, PooledConn};
use std::collections::HashMap;
use std::sync::{Arc, RwLock, RwLockReadGuard, RwLockWriteGuard};
use wu::{Fail, Result};

/// Help output
pub const HELP: &str = "
Usage: wu-api [OPTIONS]
String S, Integer I, Boolean B (+Length)

Options:
  --port       I       Port (4490)
  --addr       S       IP address ([::])
  --api-port   I       API Port (PORT + 9)
  --api-addr   S       API IP address (ADDR)
  --api-key    S+32    API key (RANDOM)
  --threads    I       Number of threads to start (2)
  --data       S       Data directory (data)
  --cert       S       Path to TLS certificate (DATA_DIR/cert.pem)
  --key        S       Path to TLS certificate key (DATA_DIR/key.pem)
  --mysql-addr S       MySQL server address ([::1])
  --mysql-port I       MySQL server port (3306)
  --mysql-db   S       MySQL database name (webuniverse)
  --mysql-user S       MySQL username (webuniverse)
  --mysql-pass S       MySQL password (webuniverse)";

/// Cargo.toml
pub const CARGO_TOML: &str = include_str!("../Cargo.toml");

/// Data shared between handlers
pub struct SharedData {
    users: RwLock<StorageFile>,
    logins: RwLock<UserLogins>,
    data_dir: RwLock<String>,
    servers: Arc<RwLock<HashMap<String, Server>>>,
    statistics: RwLock<HashMap<String, Statistics>>,
    mysql_pool: Pool,
}

impl SharedData {
    /// Default SharedData
    pub fn new(users: StorageFile, data_dir: String, mysql_pool: Pool) -> Self {
        // return default with provided user data
        Self {
            users: RwLock::new(users),
            logins: RwLock::new(UserLogins::new()),
            data_dir: RwLock::new(data_dir),
            servers: Arc::new(RwLock::new(HashMap::new())),
            statistics: RwLock::new(HashMap::new()),
            mysql_pool,
        }
    }

    /// Users database read-only
    pub fn users(&self) -> RwLockReadGuard<'_, StorageFile> {
        self.users.read().unwrap()
    }

    /// Users database writeable
    pub fn users_mut(&self) -> RwLockWriteGuard<'_, StorageFile> {
        self.users.write().unwrap()
    }

    /// User logins read-only
    pub fn logins(&self) -> RwLockReadGuard<'_, UserLogins> {
        self.logins.read().unwrap()
    }

    /// User logins writeable
    pub fn logins_mut(&self) -> RwLockWriteGuard<'_, UserLogins> {
        self.logins.write().unwrap()
    }

    /// Data directory read-only
    pub fn data_dir(&self) -> RwLockReadGuard<'_, String> {
        self.data_dir.read().unwrap()
    }

    /// Servers map read-only
    pub fn servers(&self) -> RwLockReadGuard<'_, HashMap<String, Server>> {
        self.servers.read().unwrap()
    }

    /// Servers map writeable
    pub fn servers_mut(&self) -> RwLockWriteGuard<'_, HashMap<String, Server>> {
        self.servers.write().unwrap()
    }

    /// Statistics map read-only
    pub fn statistics(&self) -> RwLockReadGuard<'_, HashMap<String, Statistics>> {
        self.statistics.read().unwrap()
    }

    /// Statistics map writeable
    pub fn statistics_mut(&self) -> RwLockWriteGuard<'_, HashMap<String, Statistics>> {
        self.statistics.write().unwrap()
    }

    /// MySQL database connection read-only
    pub fn mysql_conn(&self) -> Result<PooledConn> {
        self.mysql_pool.get_conn().or_else(Fail::from)
    }
}

/// Server statistics
#[derive(Debug, Default)]
pub struct Statistics {
    /// CPU usage in percent
    cpu: RwLock<f64>,

    /// Memory used and total in kB
    mem: RwLock<(u64, u64)>,

    /// Disk space used and total in kB
    disk: RwLock<(u64, u64)>,
}

impl Statistics {
    /// Create new statistics
    pub fn new() -> Self {
        Self {
            cpu: RwLock::new(0.0),
            mem: RwLock::new((0, 0)),
            disk: RwLock::new((0, 0)),
        }
    }

    /// CPU usage in percent read-only
    pub fn cpu(&self) -> RwLockReadGuard<'_, f64> {
        self.cpu.read().unwrap()
    }

    /// CPU usage in percent writeable
    pub fn cpu_mut(&self) -> RwLockWriteGuard<'_, f64> {
        self.cpu.write().unwrap()
    }

    /// Memory usage in percent read-only
    pub fn mem(&self) -> RwLockReadGuard<'_, (u64, u64)> {
        self.mem.read().unwrap()
    }

    /// Memory usage in percent writeable
    pub fn mem_mut(&self) -> RwLockWriteGuard<'_, (u64, u64)> {
        self.mem.write().unwrap()
    }

    /// Disk space usage in percent read-only
    pub fn disk(&self) -> RwLockReadGuard<'_, (u64, u64)> {
        self.disk.read().unwrap()
    }

    /// Disk space usage in percent writeable
    pub fn disk_mut(&self) -> RwLockWriteGuard<'_, (u64, u64)> {
        self.disk.write().unwrap()
    }
}
