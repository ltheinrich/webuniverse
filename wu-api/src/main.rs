//! Webuniverse API
#![cfg(target_os = "linux")]

#[macro_use]
extern crate jzon;

mod api;
mod client_api;
mod common;
mod data;
mod utils;

use client_api::listen_clients;
pub use common::*;
use data::StorageFile;
use kern::http::server::{HttpRequest, HttpServerBuilder};
use mysql::Pool;
use std::env::args;
use std::fs::create_dir;
use std::sync::OnceLock;
use wu::crypto::{argon2_hash, hash_password};
use wu::crypto::{random, random_an};
use wu::http::server::{HttpSettings, load_certificate_provider};
use wu::{
    CliBuilder, Result,
    meta::{init_name, init_version},
};

static SHARED: OnceLock<SharedData> = OnceLock::new();

pub fn get_share() -> &'static SharedData {
    SHARED.get().unwrap()
}

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
        return println!("{HELP}");
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
    let mut users = StorageFile::new(format!("{}/users.wdb", &data)).unwrap();

    // create admin:admin user if empty
    if users.cache().is_empty() {
        users.cache_mut().insert(
            "admin".to_string(),
            argon2_hash(hash_password("admin", "admin"), random(16)).unwrap(),
        );
    }

    // connect to MariaDB (old+new)
    /*let mysql_url = format!(
        "mysql://{}:{}@{}:{}/{}",
        mysql_user, mysql_pass, mysql_addr, mysql_port, mysql_db
    );*/
    let mysql_opts = mysql::OptsBuilder::new()
        .ip_or_hostname(Some(mysql_addr))
        .tcp_port(mysql_port)
        .db_name(Some(mysql_db))
        .user(Some(mysql_user))
        .pass(Some(mysql_pass));
    let mysql_pool = Pool::new(mysql_opts).unwrap();

    // shared data
    let shared = SharedData::new(users, data, mysql_pool);
    SHARED.set(shared).map_err(|_| 0).unwrap();

    // start HTTPS server
    let tls_config = load_certificate_provider(cert, key).unwrap();
    let settings = HttpSettings::new().threads_num(threads);
    HttpServerBuilder::new()
        .addr(format!("{addr}:{port}"))
        .settings(settings)
        .tls_on(tls_config)
        .handler(handle)
        .build()
        .unwrap();

    // print info message
    println!("HTTPS server available on {addr}:{port}");

    // client api
    listen_clients(&format!("{api_addr}:{api_port}"), &api_key).unwrap();
}

/// Assigning requests to handlers
fn handle(req: HttpRequest) -> Result<Vec<u8>> {
    // match url
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
    Ok(match handler(req, get_share()) {
        Ok(resp) => resp,
        Err(err) => json_error(err),
    })
}
