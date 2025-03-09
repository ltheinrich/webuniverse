//! User API handlers

use crate::SharedData;
use crate::common::*;
use kern::http::server::HttpRequest;
use wu::crypto::argon2_verify;
use wu::crypto::hash;
use wu::{Fail, Result};

/// Token validation handler
pub fn valid(req: HttpRequest, shared: &SharedData) -> Result<Vec<u8>> {
    // get values
    let headers = req.headers();
    let username = get_username(headers)?;
    let token = get_str(headers, "token")?;

    // validate
    Ok(jsonify(
        object!(valid: shared.logins().valid(username, token)),
    ))
}

/// Account logout handler
pub fn logout(req: HttpRequest, shared: &SharedData) -> Result<Vec<u8>> {
    // get values
    let headers = req.headers();
    let username = get_username(headers)?;
    let token = get_str(headers, "token")?;

    // verify login
    if shared.logins().valid(username, token) {
        // delete user token
        shared.logins_mut().remove(username, token);

        // successfully deleted
        Ok(jsonify(object!(error: false)))
    } else {
        // wrong login token
        Fail::from("unauthenticated")
    }
}

/// Account deletion handler
pub fn delete(req: HttpRequest, shared: &SharedData) -> Result<Vec<u8>> {
    // get values
    let headers = req.headers();
    let username = get_username(headers)?;
    let token = get_str(headers, "token")?;

    // verify login
    if shared.logins().valid(username, token) {
        // delete user
        let mut user_data = shared.users_mut();
        user_data.cache_mut().remove(username);
        user_data.write()?;
        shared.logins_mut().remove_user(username);

        // successfully deleted
        Ok(jsonify(object!(error: false)))
    } else {
        // wrong login token
        Fail::from("unauthenticated")
    }
}

/// Login handler
pub fn login(req: HttpRequest, shared: &SharedData) -> Result<Vec<u8>> {
    // get values
    let headers = req.headers();
    let username = get_username(headers)?;
    let password = get_str(headers, "password")?;

    // get password hash from db
    let user_data = shared.users();
    if let Some(password_hash) = user_data.cache().get(username) {
        // verify password hash
        if argon2_verify(password_hash, password) {
            // return login token
            return Ok(jsonify(object!(token: shared.logins_mut().add(username))));
        }
    }
    Fail::from("unauthenticated")
}

/// Update user handler
pub fn update(req: HttpRequest, shared: &SharedData) -> Result<Vec<u8>> {
    // get values
    let headers = req.headers();
    let username = get_username(headers)?;
    let token = get_str(headers, "token")?;
    let new_password = get_str(headers, "newpassword")?;
    let new_username = get_an(headers, "newusername");

    // verify login
    if shared.logins().valid(username, token) {
        let mut user_data = shared.users_mut();

        // change password
        if let Some(user_password) = user_data.cache_mut().get_mut(username) {
            // hash and change password
            *user_password = hash(new_password);
            user_data.write()?;
        } else {
            return Fail::from("internal error: user entry does not exist in cache");
        }

        // change username
        if let Ok(new_username) = new_username {
            // borrow users mutably
            let users = user_data.cache_mut();

            // check if user already exists
            if users.contains_key(new_username) {
                return Fail::from("new username already exists");
            }

            // rename user
            let password_hash = users
                .remove(username)
                .ok_or_else(|| Fail::new("internal error: user entry does not exist in cache"))?;
            users.insert(new_username.to_string(), password_hash);
            user_data.write()?;
            shared
                .logins_mut()
                .rename(username, new_username.to_string());
        }

        // return success
        Ok(jsonify(object!(error: false)))
    } else {
        Fail::from("unauthenticated")
    }
}
