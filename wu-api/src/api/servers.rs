//! Servers API

use crate::common::*;
use kern::http::server::HttpRequest;
use wu::{Fail, Result};

/// List servers handler
pub fn list(req: HttpRequest, shared: &SharedData) -> Result<Vec<u8>> {
    // get values
    let headers = req.headers();
    let username = get_username(headers)?;
    let token = get_str(headers, "token")?;

    // verify login
    if shared.logins().valid(username, token) {
        // get server names
        let servers = shared.servers();
        let server_names: Vec<&str> = servers.keys().map(|n| n.as_str()).collect();

        // return servers list
        Ok(jsonify(object!(servers: server_names)))
    } else {
        Fail::from("unauthenticated")
    }
}

/// Get server console data handler
pub fn data(req: HttpRequest, shared: &SharedData) -> Result<Vec<u8>> {
    // get values
    let headers = req.headers();
    let username = get_username(headers)?;
    let token = get_str(headers, "token")?;
    let name = get_str(headers, "name")?;
    let read_len = get(headers, "readlen").unwrap_or(0usize);

    // verify login
    if shared.logins().valid(username, token) {
        // get server names
        let servers = shared.servers();
        match servers.get(name) {
            Some(server) => {
                // return console data
                let data = server.data();
                let read_len = if data.len() >= read_len {
                    if data.len() - read_len >= 50000 {
                        data.len() - 50000
                    } else {
                        read_len
                    }
                } else {
                    0
                };
                Ok(jsonify(object!(data: &data[read_len..], len: data.len())))
                /*
                if read_len < data.len() {
                    Ok(jsonify(object!(data: &data[read_len..])))
                } else {
                    Ok(jsonify(object!(data: "")))
                }
                */
            }
            None => Fail::from("server does not exist"),
        }
    } else {
        Fail::from("unauthenticated")
    }
}

/// Execute server command handler
pub fn exec(req: HttpRequest, shared: &SharedData) -> Result<Vec<u8>> {
    // get values
    let headers = req.headers();
    let username = get_username(headers)?;
    let token = get_str(headers, "token")?;
    let name = get_str(headers, "name")?;
    let server_command = get(headers, "servercommand")?;

    // verify login
    if shared.logins().valid(username, token) {
        // get server names
        let servers = shared.servers();
        match servers.get(name) {
            Some(server) => {
                // send command to execute
                server.cmd(server_command)?;

                // return successs
                Ok(jsonify(object!(error: false)))
            }
            None => Fail::from("server does not exist"),
        }
    } else {
        Fail::from("unauthenticated")
    }
}
