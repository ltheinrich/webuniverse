//! Common

pub use crate::utils::*;

/// Help output
pub const HELP: &str = "
Usage: wu-client [OPTIONS] COMMAND
String S, Integer I, Boolean B (+Length)

Options:
  --addr          S       Listener address ([::]:0)
  --api-port       I       API port (4499)
  --api-addr       S       API IP address ([::1])
  --api-key       S+32    API key (RANDOM)";

/// Cargo.toml
pub const CARGO_TOML: &str = include_str!("../Cargo.toml");
