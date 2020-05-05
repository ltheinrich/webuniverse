//! Add server handler

use std::io::prelude::*;
use std::io::BufReader;
use std::net::TcpListener;
use std::process::{Command as Process, Stdio};
use std::sync::{Arc, RwLock};
use std::thread;
use std::thread::sleep;
use std::time::Duration;
use wu::net::{ConnBuilder, Connection};
use wu::Command;
use wu::Fail;

pub fn add_server(mut conn: Connection, cmd: Command, addr: String) {
    // start process
    let mut process = Process::new(cmd.arg(1, ""))
        .args(match cmd.arguments().len() {
            len if len <= 2 => &[],
            _ => &cmd.arguments()[2..],
        })
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .unwrap();

    // init
    let stdin = Arc::new(RwLock::new(process.stdin.take().unwrap()));
    let stdout = process.stdout.take().unwrap();
    let aead = Arc::new(conn.crypter_aead().clone());

    // listen
    let listener = TcpListener::bind(addr).or_else(Fail::from).unwrap();
    conn.write(listener.local_addr().unwrap().port().to_be_bytes())
        .unwrap();

    // listener thread
    thread::spawn(move || {
        loop {
            // accept connections
            if let Ok((stream, _)) = listener.accept() {
                // clone
                let stdin = stdin.clone();
                let aead = aead.clone();

                thread::spawn(move || {
                    // accept connection and read
                    let mut conn = ConnBuilder::from(stream, &aead).accept().unwrap();
                    let read = conn.read().unwrap();

                    // write to process stdin
                    let mut stdin = stdin.write().unwrap();
                    stdin.write_all(&read).unwrap();
                    stdin.write_all(b"\n").unwrap();
                });
            }
        }
    });

    // buffered reader
    let mut br = BufReader::new(stdout);
    let mut buf = Vec::new();
    loop {
        // read into buf
        let read_len = br.read_until(b'\n', &mut buf).unwrap();

        // check if empty
        if read_len != 0 {
            // write buffer
            conn.write(&buf).unwrap();
        } else {
            // sleep
            sleep(Duration::from_millis(25));
        }

        // empty buffer
        buf.clear();
    }
}
