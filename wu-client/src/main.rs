//! Webuniverse Linux client
#![cfg(target_os = "linux")]

pub mod common;

mod handlers;
mod utils;

use common::*;
use std::env::args;
use std::net::TcpStream;
use wu::crypto::init_aead;
use wu::crypto::random_an;
use wu::meta::{init_name, init_version};
use wu::net::ConnBuilder;
use wu::CliBuilder;

fn main() {
    // print version
    println!(
        "{} {} (c) 2020 Lennart Heinrich",
        init_name(CARGO_TOML),
        init_version(CARGO_TOML)
    );

    // read cli
    let args: Vec<String> = args().collect();
    let cmd = CliBuilder::new().options(&["help"]).build(&args);
    if cmd.option("help") {
        return println!("{}", HELP);
    }

    // configuration
    let addr = cmd.parameter("addr", "[::]:0".to_string());
    let api_port = cmd.parameter("api-port", 4499u16);
    let api_addr = cmd.parameter("api-addr", "[::1]".to_string());
    let api_key = cmd.parameter("api-key", random_an(32));
    let name = cmd.parameter("name", random_an(12));
    let htype = cmd.arg(0, "");

    // connect
    let stream = TcpStream::connect(format!("{}:{}", api_addr, api_port)).unwrap();
    let aead = init_aead(api_key);
    let mut conn = ConnBuilder::from(stream, &aead).init().unwrap();

    // init connection
    conn.write(htype.as_bytes()).unwrap();
    conn.write(name.as_bytes()).unwrap();

    // handle
    match cmd.arg(0, "") {
        "add-server" => handlers::add_server(conn, cmd, addr),
        "send-stats" => handlers::send_stats(conn, cmd),
        _ => println!("{}", HELP),
    }
}
