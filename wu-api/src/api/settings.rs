//! Settings API

use crate::common::*;
use json::JsonValue;
use lhi::server::HttpRequest;
use mysql::prelude::*;
use std::sync::RwLockReadGuard;
use wu::crypto::hex_decode;
use wu::Fail;

/// Get all settings
pub fn all(req: HttpRequest, shared: RwLockReadGuard<'_, SharedData>) -> Result<Vec<u8>, Fail> {
    // get values
    let headers = req.headers();
    let username = get_username(headers)?;
    let token = get_str(headers, "token")?;

    // verify login
    if shared.logins().valid(username, token) {
        let mut conn = shared.mysql_conn()?;
        let mut settings = JsonValue::new_object();
        conn.query_map(
            "SELECT `key`, `value` FROM settings",
            |(key, value): (String, String)| {
                settings[key] = JsonValue::String(value);
            },
        )
        .or_else(Fail::from)?;

        // return servers list
        Ok(jsonify(object!(settings: settings)))
    } else {
        Fail::from("unauthenticated")
    }
}

/// Set setting
pub fn set(req: HttpRequest, shared: RwLockReadGuard<'_, SharedData>) -> Result<Vec<u8>, Fail> {
    // get values
    let headers = req.headers();
    let username = get_username(headers)?;
    let token = get_str(headers, "token")?;
    let setting_key = get_str(headers, "settingkey")?;
    let setting_value = get_str(headers, "settingvalue")?;

    // verify login
    if shared.logins().valid(username, token) {
        // decode value
        let setting_value = hex_decode(setting_value).or_else(Fail::from)?;
        let setting_value = String::from_utf8(setting_value).or_else(Fail::from)?;

        // update value
        let mut conn = shared.mysql_conn()?;
        conn.exec_drop(
            r"UPDATE settings SET `value` = ? WHERE `key` = ?",
            (setting_value, setting_key),
        )
        .or_else(Fail::from)?;

        // return servers list
        Ok(jsonify(object!(error: false)))
    } else {
        Fail::from("unauthenticated")
    }
}
