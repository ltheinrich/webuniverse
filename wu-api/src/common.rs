//! Common

pub use crate::utils::*;

use crate::data::StorageFile;
use std::collections::BTreeMap;
use std::time::{Duration, SystemTime};
use wu::crypto::random_an;

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
#[derive(Debug)]
pub struct SharedData {
  pub user_data: StorageFile,
  pub user_logins: UserLogins,
  pub data_dir: String,
}

impl SharedData {
  /// Default SharedData
  pub fn new(user_data: StorageFile, data_dir: String) -> Self {
    // return default with provided user data
    Self {
      user_data,
      user_logins: UserLogins::new(),
      data_dir,
    }
  }
}

/// Seconds a login token is valid
const VALID_LOGIN_SECS: u64 = 3600;

/// User login/token management
#[derive(Clone, Debug, Default)]
pub struct UserLogins {
  user_logins: BTreeMap<String, Vec<(String, SystemTime)>>,
}

impl UserLogins {
  /// Create empty
  pub fn new() -> Self {
    Self {
      user_logins: BTreeMap::new(),
    }
  }

  /// Check if login token is valid and remove expired
  pub fn valid(&self, user: &str, token: &str) -> bool {
    // get logins
    match self.user_logins.get(user) {
      Some(logins) => {
        // check login
        logins
          .iter()
          .any(|login| login.0 == token && Self::check_unexpired(&login.1))
      }
      None => false,
    }
  }

  /// Generate login token for user
  pub fn add(&mut self, user: &str) -> &str {
    // generate random token and get logins
    let token = random_an(32);
    match self.user_logins.get_mut(user) {
      Some(logins) => {
        // remove expired logins and return logins
        Self::remove_expired(logins);
        logins.push((token, SystemTime::now()));
      }
      None => {
        // create new logins vector for user
        self
          .user_logins
          .insert(user.to_string(), [(token, SystemTime::now())].to_vec());
      }
    };

    // return token
    &self.user_logins[user].last().unwrap().0
  }

  /// Remove login token for user
  pub fn remove(&mut self, user: &str, token: &str) {
    // get logins
    if let Some(logins) = self.user_logins.get_mut(user) {
      // remove token
      logins.retain(|login| login.0 != token && Self::check_unexpired(&login.1));
    }
  }

  /// Remove all logins for user
  pub fn remove_user(&mut self, user: &str) {
    // remove user
    self.user_logins.remove(user);
  }

  /// Rename user entry
  pub fn rename(&mut self, user: &str, new_user: String) {
    if let Some(logins) = self.user_logins.remove(user) {
      self.user_logins.insert(new_user, logins);
    }
  }

  /// Remove expired logins
  fn remove_expired(logins: &mut Vec<(String, SystemTime)>) {
    (*logins).retain(|login| Self::check_unexpired(&login.1));
  }

  /// Check if login is expired
  fn check_unexpired(expiration: &SystemTime) -> bool {
    expiration
      .elapsed()
      .unwrap_or(Duration::from_secs(u64::max_value()))
      .as_secs()
      < VALID_LOGIN_SECS
  }
}
