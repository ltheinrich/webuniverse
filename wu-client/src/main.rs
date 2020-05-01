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
use wu::crypto::random_an;
use wu::meta::{init_name, init_version};
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
    let port = cmd.param("port", "4499");
    let addr = cmd.param("addr", "[::1]");
    let api_key = cmd.parameter("api-key", random_an(32));
    println!("executing 'ping srv.ltheinrich.de' ...");
    let mut process = Process::new("ping")
        .arg("srv.ltheinrich.de")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .unwrap();
    let stdout = process.stdout.as_mut().unwrap();
    let mut br = BufReader::new(stdout);
    let mut output = Vec::new();
    for i in 0..6 {
        let mut buf = Vec::new();
        br.read_until(b'\n', &mut buf).unwrap();
        if i > 0 {
            println!("Ping {}", i);
        }
        output.append(&mut buf);
    }
    process.kill().unwrap();
    println!("'ping srv.ltheinrich.de' executed");
    let mut stream = TcpStream::connect(format!("{}:{}", addr, port)).unwrap();
    use wu::aes_gcm::aead::{generic_array::GenericArray, Aead, NewAead};
    use wu::aes_gcm::Aes256Gcm;
    let key = GenericArray::clone_from_slice(api_key.as_bytes());
    let aead = Aes256Gcm::new(key);
    let nonce = GenericArray::from_slice(b"unique nonce");
    let ciphertext = aead
        .encrypt(nonce, String::from_utf8_lossy(&output).as_bytes())
        .expect("encryption failure!");
    stream.write_all(&ciphertext.len().to_be_bytes()).unwrap();
    stream.write_all(b"unique nonce").unwrap();
    stream.write_all(&ciphertext).unwrap();
}
