//! Users API handling

use crate::utils::*;
use crate::SharedData;
use lhi::server::HttpRequest;
use std::sync::{Arc, RwLock};
use wu::crypto::hash;
use wu::Fail;

/// User deletion handler
pub fn delete(req: HttpRequest, shared: Arc<RwLock<SharedData>>) -> Result<Vec<u8>, Fail> {
    // get values
    let headers = req.headers();
    let username = get_username(headers)?;
    let token = get_str(headers, "token")?;
    let user = get_an(headers, "user")?;

    // get shared
    let mut shared = shared.write().unwrap();

    // verify login
    if shared.user_logins.valid(username, token) {
        // delete user
        let users = shared.user_data.cache_mut();
        users.remove(user);
        shared.user_data.write()?;
        shared.user_logins.remove_user(user);

        // successfully deleted
        Ok(jsonify(object!(error: false)))
    } else {
        // wrong login token
        Fail::from("unauthenticated")
    }
}

/// Account creation handler
pub fn create(req: HttpRequest, shared: Arc<RwLock<SharedData>>) -> Result<Vec<u8>, Fail> {
    // get values
    let headers = req.headers();
    let username = get_username(headers)?;
    let token = get_str(headers, "token")?;
    let user = get_an(headers, "user")?;
    let password = get_str(headers, "password")?;

    // get shared
    let mut shared = shared.write().unwrap();

    // verify login
    if shared.user_logins.valid(username, token) {
        // cache mut
        let users = shared.user_data.cache_mut();

        // check if user already exists
        if users.contains_key(user) {
            return Fail::from("username already exists");
        }

        // create user
        users.insert(user.to_string(), hash(password));
        shared.user_data.write()?;

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
    let shared = shared.read().unwrap();
    let users = shared.user_data.cache();

    // verify login
    if shared.user_logins.valid(username, token) {
        // return success
        let users: Vec<&str> = users.keys().map(|n| n.as_str()).collect();
        Ok(jsonify(object!(users: users)))
    } else {
        Fail::from("unauthenticated")
    }
}

/// Change user handler
pub fn change(req: HttpRequest, shared: Arc<RwLock<SharedData>>) -> Result<Vec<u8>, Fail> {
    // get values
    let headers = req.headers();
    let username = get_username(headers)?;
    let token = get_str(headers, "token")?;
    let user = get_str(headers, "user")?;
    let password = get_str(headers, "password")?;
    let new_username = get_an(headers, "new_username");

    // get shared
    let mut shared = shared.write().unwrap();

    // verify login
    if shared.user_logins.valid(username, token) {
        // change password
        if let Some(user_password) = shared.user_data.cache_mut().get_mut(user) {
            // hash and change password
            *user_password = hash(password);
            shared.user_data.write()?;
        } else {
            return Fail::from("user does not exist");
        }

        // change username
        match new_username {
            Ok(new_username) => {
                // borrow users mutably
                let users = shared.user_data.cache_mut();

                // check if user already exists
                if users.contains_key(new_username) {
                    return Fail::from("new username already exists");
                }

                // rename user
                let password_hash = users
                    .remove(user)
                    .ok_or_else(|| Fail::new("user does not exist"))?;
                users.insert(new_username.to_string(), password_hash);
                shared.user_data.write()?;
                shared.user_logins.rename(user, new_username.to_string());
            }
            Err(err) => {
                if err.err_msg() == "new_username is not alphanumeric" {
                    return Err(err);
                }
            }
        }

        // return success
        Ok(jsonify(object!(error: false)))
    } else {
        Fail::from("unauthenticated")
    }
}
