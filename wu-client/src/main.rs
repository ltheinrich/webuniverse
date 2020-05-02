//! Webuniverse Linux client
#![cfg(target_os = "linux")]

mod common;
mod utils;

pub use common::*;

use std::env::args;
use std::io::prelude::*;
use std::io::BufReader;
use std::net::TcpStream;
use std::process::{Command as Process, Stdio};
use std::thread::sleep;
use std::time::Duration;
use wu::crypto::{init_aead, random_an};
use wu::meta::{init_name, init_version};
use wu::net::ConnBuilder;
use wu::Command;

fn main() {
    // print version
    println!(
        "{} {} (c) 2020 Lennart Heinrich",
        init_name(CARGO_TOML),
        init_version(CARGO_TOML)
    );

    // read cli
    let args: Vec<String> = args().collect();
    let cmd = Command::from(&args, &["help"]);
    if cmd.option("help") {
        return println!("{}", HELP);
    }

    // configuration
    let _addr = cmd.param("addr", "[::]:0");
    let api_port = cmd.param("api-port", "4499");
    let api_addr = cmd.param("api-addr", "[::1]");
    let api_key = cmd.parameter("api-key", random_an(32));

    let mut process = Process::new(cmd.arg(0, ""))
        .args(match cmd.arguments().len() {
            1 => &[],
            _ => &cmd.arguments()[1..],
        })
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .unwrap();

    let stream = TcpStream::connect(format!("{}:{}", api_addr, api_port)).unwrap();
    let aead = init_aead(api_key);
    let mut conn = ConnBuilder::from(stream, &aead).init().unwrap();
    conn.write(b"lennart").unwrap();

    let stdout = process.stdout.as_mut().unwrap();
    let mut br = BufReader::new(stdout);
    loop {
        let mut buf = Vec::new();
        let read_len = br.read_until(b'\n', &mut buf).unwrap();
        if read_len != 0 {
            conn.write(buf).unwrap();
        } else {
            sleep(Duration::from_millis(25));
        }
    }
}
