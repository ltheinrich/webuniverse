//! API utils

use json::JsonValue;
use kern::http::server::{respond, ResponseData};
use std::collections::BTreeMap;
use std::fmt::Display;
use std::str::FromStr;
use wu::{Fail, Result};

/// Get value as string or fail
pub fn get_str<'a>(data: &BTreeMap<String, &'a str>, key: &str) -> Result<&'a str> {
    Ok(*data
        .get(key)
        .ok_or_else(|| Fail::new(format!("{key} required")))?)
}

/// Get value or fail
pub fn get<T: FromStr>(data: &BTreeMap<String, &str>, key: &str) -> Result<T> {
    get_str(data, key)?
        .parse()
        .or_else(|_| Fail::from(format!("{key} is not correct type")))
}

/// Get alphanumeric value as string or fail
pub fn get_an<'a>(data: &BTreeMap<String, &'a str>, key: &str) -> Result<&'a str> {
    // get string
    let an = get_str(data, key)?;

    // check if alphanumeric
    if !an.chars().all(char::is_alphanumeric) {
        return Fail::from(format!("{key} is not alphanumeric"));
    }

    // return string
    Ok(an)
}

/// Get username string and check if alphanumeric
pub fn get_username<'a>(data: &BTreeMap<String, &'a str>) -> Result<&'a str> {
    get_an(data, "username")
}

/// Respond plain
pub fn respond_plain(plain: impl AsRef<[u8]>) -> Vec<u8> {
    respond(plain, "application/json", cors_headers())
}

/// Convert JsonValue to response
pub fn jsonify(value: JsonValue) -> Vec<u8> {
    respond(value.to_string(), "application/json", cors_headers())
}

/// Convert error message into json format error
pub fn json_error<E: Display>(err: E) -> Vec<u8> {
    jsonify(object!(error: err.to_string()))
}

pub fn cors_headers() -> Option<ResponseData<'static>> {
    let mut resp_data = ResponseData::new();
    resp_data.headers.insert("access-control-allow-origin", "*");
    resp_data
        .headers
        .insert("access-control-allow-headers", "*");
    resp_data
        .headers
        .insert("access-control-allow-methods", "*");
    Some(resp_data)
}
