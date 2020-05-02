//! Common

pub use crate::utils::*;

use crate::api::logins::UserLogins;
use crate::client_api::server::Server;
use crate::data::StorageFile;
use std::collections::BTreeMap;
use std::sync::{Arc, RwLock, RwLockReadGuard, RwLockWriteGuard};

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
  --key        S       Path to TLS certificate key (DATA_DIR/key.pem)";

/// Cargo.toml
pub const CARGO_TOML: &str = include_str!("../Cargo.toml");

/// Data shared between handlers
pub struct SharedData {
  users: RwLock<StorageFile>,
  logins: RwLock<UserLogins>,
  data_dir: RwLock<String>,
  servers: Arc<RwLock<BTreeMap<String, Server>>>,
}

impl SharedData {
  /// Default SharedData
  pub fn new(users: StorageFile, data_dir: String) -> Self {
    // return default with provided user data
    Self {
      users: RwLock::new(users),
      logins: RwLock::new(UserLogins::new()),
      data_dir: RwLock::new(data_dir),
      servers: Arc::new(RwLock::new(BTreeMap::new())),
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
  pub fn servers(&self) -> RwLockReadGuard<'_, BTreeMap<String, Server>> {
    self.servers.read().unwrap()
  }

  /// Servers map writeable
  pub fn servers_mut(&self) -> RwLockWriteGuard<'_, BTreeMap<String, Server>> {
    self.servers.write().unwrap()
  }
}
