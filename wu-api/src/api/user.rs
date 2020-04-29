//! User API handlers

use crate::utils::*;
use crate::SharedData;
use lhi::server::HttpRequest;
use std::sync::{Arc, RwLock};
use wu::crypto::hash;
use wu::Fail;

/// Token validation handler
pub fn valid(req: HttpRequest, shared: Arc<RwLock<SharedData>>) -> Result<Vec<u8>, Fail> {
    // get values
    let headers = req.headers();
    let username = get_username(headers)?;
    let token = get_str(headers, "token")?;

    // get shared and validate
    let shared = shared.read().unwrap();
    Ok(jsonify(
        object!(valid: shared.user_logins.valid(username, token)),
    ))
}

/// Account logout handler
pub fn logout(req: HttpRequest, shared: Arc<RwLock<SharedData>>) -> Result<Vec<u8>, Fail> {
    // get values
    let headers = req.headers();
    let username = get_username(headers)?;
    let token = get_str(headers, "token")?;

    // get shared
    let mut shared = shared.write().unwrap();

    // verify login
    if shared.user_logins.valid(username, token) {
        // delete user token
        shared.user_logins.remove(username, token);

        // successfully deleted
        Ok(jsonify(object!(error: false)))
    } else {
        // wrong login token
        Fail::from("unauthenticated")
    }
}

/// Account deletion handler
pub fn delete(req: HttpRequest, shared: Arc<RwLock<SharedData>>) -> Result<Vec<u8>, Fail> {
    // get values
    let headers = req.headers();
    let username = get_username(headers)?;
    let token = get_str(headers, "token")?;

    // get shared
    let mut shared = shared.write().unwrap();
    let mut users = shared.user_data.parse()?;

    // verify login
    if shared.user_logins.valid(username, token) {
        // delete user
        users.remove(username);
        shared.user_data.serialize(&users)?;
        shared.user_logins.remove_user(username);

        // successfully deleted
        Ok(jsonify(object!(error: false)))
    } else {
        // wrong login token
        Fail::from("unauthenticated")
    }
}

/// User deletion handler
pub fn delete_user(req: HttpRequest, shared: Arc<RwLock<SharedData>>) -> Result<Vec<u8>, Fail> {
    // get values
    let headers = req.headers();
    let username = get_username(headers)?;
    let token = get_str(headers, "token")?;
    let user_username = get_an(headers, "user_username")?;

    // get shared
    let mut shared = shared.write().unwrap();
    let mut users = shared.user_data.parse()?;

    // verify login
    if shared.user_logins.valid(username, token) {
        // delete user
        users.remove(user_username);
        shared.user_data.serialize(&users)?;
        shared.user_logins.remove_user(user_username);

        // successfully deleted
        Ok(jsonify(object!(error: false)))
    } else {
        // wrong login token
        Fail::from("unauthenticated")
    }
}

/// Login handler
pub fn login(req: HttpRequest, shared: Arc<RwLock<SharedData>>) -> Result<Vec<u8>, Fail> {
    // get values
    let headers = req.headers();
    let username = get_username(headers)?;
    let password = get_str(headers, "password")?;

    // get shared
    let mut shared = shared.write().unwrap();
    let users = shared.user_data.parse()?;

    // get password hash from db
    match users.get(username) {
        Some(password_hash) => {
            // verify password hash
            if password_hash != &hash(password) {
                return Fail::from("unauthenticated");
            }

            // return login token
            Ok(jsonify(object!(token: shared.user_logins.add(username))))
        }
        None => Fail::from("unauthenticated"),
    }
}

/// Account creation handler
pub fn create(req: HttpRequest, shared: Arc<RwLock<SharedData>>) -> Result<Vec<u8>, Fail> {
    // get values
    let headers = req.headers();
    let username = get_username(headers)?;
    let token = get_str(headers, "token")?;
    let user_username = get_an(headers, "user_username")?;
    let user_password = get_str(headers, "user_password")?;

    // get shared
    let mut shared = shared.write().unwrap();
    let mut users = shared.user_data.parse()?;

    // verify login
    if shared.user_logins.valid(username, token) {
        // check if user already exists
        if users.contains_key(user_username) {
            return Fail::from("username already exists");
        }

        // create user
        users.insert(user_username.to_string(), hash(user_password));
        shared.user_data.serialize(&users)?;

        // return success
        Ok(jsonify(object!(error: false)))
    } else {
        Fail::from("unauthenticated")
    }
}

/// Update user handler
pub fn update(req: HttpRequest, shared: Arc<RwLock<SharedData>>) -> Result<Vec<u8>, Fail> {
    // get values
    let headers = req.headers();
    let username = get_username(headers)?;
    let token = get_str(headers, "token")?;
    let new_password = get_str(headers, "new_password")?;
    let new_username = get_an(headers, "new_username");

    // get shared
    let mut shared = shared.write().unwrap();
    let mut users = shared.user_data.parse()?;

    // verify login
    if shared.user_logins.valid(username, token) {
        // change password
        if let Some(user_password) = users.get_mut(username) {
            // hash and change password
            *user_password = hash(new_password);
            shared.user_data.serialize(&users)?;
        } else {
            return Fail::from("internal error: user entry does not exist in cache");
        }

        // change username
        if let Ok(new_username) = new_username {
            // check if user already exists
            if users.contains_key(new_username) {
                return Fail::from("new username already exists");
            }

            // rename user
            let password_hash = users
                .remove(username)
                .ok_or_else(|| Fail::new("internal error: user entry does not exist in cache"))?;
            users.insert(new_username.to_string(), password_hash);
            shared.user_data.serialize(&users)?;
            shared
                .user_logins
                .rename(username, new_username.to_string());
        }

        // return success
        Ok(jsonify(object!(error: false)))
    } else {
        Fail::from("unauthenticated")
    }
}

/// Account list handler
pub fn list(req: HttpRequest, shared: Arc<RwLock<SharedData>>) -> Result<Vec<u8>, Fail> {
    // get values
    let headers = req.headers();
    let username = get_username(headers)?;
    let token = get_str(headers, "token")?;

    // get shared
    let mut shared = shared.write().unwrap();
    let users = shared.user_data.parse()?;

    // verify login
    if shared.user_logins.valid(username, token) {
        // drop shared borrow
        drop(shared);

        // return success
        let users: Vec<&str> = users.keys().map(|n| n.as_str()).collect();
        Ok(jsonify(object!(users: users)))
    } else {
        Fail::from("unauthenticated")
    }
}
