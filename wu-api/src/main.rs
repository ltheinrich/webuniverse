//! Webuniverse API
#![cfg(target_os = "linux")]

#[macro_use]
extern crate json;

mod api;
mod client_api;
mod common;
mod data;
mod utils;

use client_api::listen_clients;
pub use common::*;
use data::StorageFile;
use lhi::server::{listen, load_certificate, HttpRequest, HttpSettings};
use mysql::Pool;
use std::env::args;
use std::fs::create_dir;
use std::sync::{Arc, RwLock};
use utils::json_error;
use wu::crypto::{argon2_hash, hash_password};
use wu::crypto::{random, random_an};
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
    let port = cmd.parameter("port", 4490);
    let addr = cmd.param("addr", "[::]");
    let api_port = cmd.parameter("api-port", port + 9);
    let api_addr = cmd.param("api-addr", addr);
    let api_key = cmd.parameter("api-key", random_an(32));
    let threads = cmd.parameter("threads", 2);
    let data = cmd.parameter("data", "data".to_string());
    let cert = cmd.parameter("cert", format!("{}/cert.pem", &data));
    let key = cmd.parameter("key", format!("{}/key.pem", &data));
    let mysql_addr = cmd.param("mysql-addr", "localhost");
    let mysql_port = cmd.parameter("mysql-port", 3306);
    let mysql_db = cmd.param("mysql-db", "webuniverse");
    let mysql_user = cmd.param("mysql-user", "webuniverse");
    let mysql_pass = cmd.param("mysql-pass", "webuniverse");

    // open users database
    create_dir(&data).ok();
    let mut users = StorageFile::new(&format!("{}/users.wdb", &data)).unwrap();

    // create admin:admin user if empty
    if users.cache().is_empty() {
        users.cache_mut().insert(
            "admin".to_string(),
            argon2_hash(hash_password("admin", "admin"), random(16)).unwrap(),
        );
    }

    // connect to MariaDB
    let mysql_url = format!(
        "mysql://{}:{}@{}:{}/{}",
        mysql_user, mysql_pass, mysql_addr, mysql_port, mysql_db
    );
    let mysql_pool = Pool::new(mysql_url).unwrap();

    // shared data
    let shared = Arc::new(RwLock::new(SharedData::new(users, data, mysql_pool)));

    // start HTTPS server
    let tls_config = load_certificate(&cert, &key).unwrap();
    let _listeners = listen(
        &format!("{}:{}", addr, port),
        threads,
        HttpSettings::new(),
        tls_config,
        handle,
        shared.clone(),
    )
    .unwrap();

    // print info message
    println!("HTTPS server available on {}:{}", addr, port);

    // client api
    listen_clients(&format!("{}:{}", api_addr, api_port), &api_key, shared).unwrap();
}

/// Assigning requests to handlers
fn handle(
    req: Result<HttpRequest, Fail>,
    shared: Arc<RwLock<SharedData>>,
) -> Result<Vec<u8>, Fail> {
    // unwrap and match url
    let req: HttpRequest = req?;
    let handler = match req.url() {
        // user
        "/user/login" => api::user::login,
        "/user/delete" => api::user::delete,
        "/user/logout" => api::user::logout,
        "/user/valid" => api::user::valid,
        "/user/update" => api::user::update,
        // users
        "/users/create" => api::users::create,
        "/users/list" => api::users::list,
        "/users/delete" => api::users::delete,
        "/users/change" => api::users::change,
        // servers
        "/servers/list" => api::servers::list,
        "/servers/data" => api::servers::data,
        "/servers/exec" => api::servers::exec,
        // server
        "/server/stats" => api::server::stats,
        // settings
        "/settings/all" => api::settings::all,
        "/settings/set" => api::settings::set,
        _ => return Ok(json_error("handler not found")),
    };

    // handle request
    Ok(match handler(req, shared.read().unwrap()) {
        Ok(resp) => resp,
        Err(err) => json_error(err),
    })
}
