//! Webuniverse API

#[macro_use]
extern crate json;

mod api;
mod common;
mod data;
mod utils;

pub use common::*;
use data::StorageFile;
use lhi::server::{listen, load_certificate, HttpRequest, HttpSettings};
use std::env::args;
use std::fs::create_dir;
use std::sync::{Arc, RwLock};
use utils::json_error;
use wu::{
    meta::{init_name, init_version},
    Command, Fail,
};

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
    let port = cmd.param("port", "4490");
    let addr = cmd.param("addr", "[::]");
    let threads = cmd.parameter("threads", 2);
    let data = cmd.parameter("data", "data".to_string());
    let cert = cmd.parameter("cert", format!("{}/cert.pem", &data));
    let key = cmd.parameter("key", format!("{}/key.pem", &data));

    // open users database
    create_dir(&data).ok();
    let user_data = StorageFile::new(&format!("{}/user_data.wdb", &data)).unwrap();

    // start HTTPS server
    let tls_config = load_certificate(&cert, &key).unwrap();
    let listeners = listen(
        &format!("{}:{}", addr, port),
        threads,
        HttpSettings::new(),
        tls_config,
        handle,
        Arc::new(RwLock::new(SharedData::new(user_data, data))),
    )
    .unwrap();

    // print info message and join threads
    println!("HTTPS server available on {}:{}", addr, port);
    for listener in listeners {
        listener.join().expect("listener thread crashed");
    }
}

/// Assigning requests to handlers
fn handle(
    req: Result<HttpRequest, Fail>,
    shared: Arc<RwLock<SharedData>>,
) -> Result<Vec<u8>, Fail> {
    // unwrap and match url
    let req: HttpRequest = req?;
    let handler = match req.url() {
        "/user/create" => api::user::create,
        "/user/login" => api::user::login,
        "/user/delete" => api::user::delete,
        "/user/logout" => api::user::logout,
        "/user/valid" => api::user::valid,
        "/user/update" => api::user::update,
        "/user/list" => api::user::list,
        "/user/delete_user" => api::user::delete_user,
        _ => return Ok(json_error("handler not found")),
    };

    // handle request
    Ok(match handler(req, shared) {
        Ok(resp) => resp,
        Err(err) => json_error(err),
    })
}
