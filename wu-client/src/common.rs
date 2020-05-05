//! Common

pub use crate::utils::*;

/// Help output
pub const HELP: &str = "
Usage: wu-client [TYPE] [OPTIONS] [COMMAND]
String S, Integer I, Boolean B (+Length)

Type:
  add-server      Start server and add to API
  send-stats      Send server statistics to API

Options:
  --addr          S       Listener address ([::]:0)
  --api-port      I       API port (4499)
  --api-addr      S       API IP address ([::1])
  --api-key       S+32    API key (RANDOM)
  --name          S       Name for server or statistics (RANDOM)";

/// Cargo.toml
pub const CARGO_TOML: &str = include_str!("../Cargo.toml");
