//! Users API handling

use crate::common::*;
use crate::SharedData;
use kern::http::server::HttpRequest;
use std::sync::RwLockReadGuard;
use wu::{Fail, Result};

/// User deletion handler
pub fn delete(req: HttpRequest, shared: RwLockReadGuard<'_, SharedData>) -> Result<Vec<u8>> {
    // get values
    let headers = req.headers();
    let username = get_username(headers)?;
    let token = get_str(headers, "token")?;
    let user = get_an(headers, "user")?;

    // verify login
    if shared.logins().valid(username, token) {
        // delete user
        let mut user_data = shared.users_mut();
        user_data.cache_mut().remove(user);
        user_data.write()?;
        shared.logins_mut().remove_user(user);

        // successfully deleted
        Ok(jsonify(object!(error: false)))
    } else {
        // wrong login token
        Fail::from("unauthenticated")
    }
}

/// Account creation handler
pub fn create(req: HttpRequest, shared: RwLockReadGuard<'_, SharedData>) -> Result<Vec<u8>> {
    // get values
    let headers = req.headers();
    let username = get_username(headers)?;
    let token = get_str(headers, "token")?;
    let user = get_an(headers, "user")?;
    let password = get_str(headers, "password")?;

    // verify login
    if shared.logins().valid(username, token) {
        // cache mut
        let mut user_data = shared.users_mut();
        let users = user_data.cache_mut();

        // check if user already exists
        if users.contains_key(user) {
            return Fail::from("username already exists");
        }

        // create user
        users.insert(user.to_string(), password.to_string());
        user_data.write()?;

        // return success
        Ok(jsonify(object!(error: false)))
    } else {
        Fail::from("unauthenticated")
    }
}

/// Account list handler
pub fn list(req: HttpRequest, shared: RwLockReadGuard<'_, SharedData>) -> Result<Vec<u8>> {
    // get values
    let headers = req.headers();
    let username = get_username(headers)?;
    let token = get_str(headers, "token")?;

    // verify login
    if shared.logins().valid(username, token) {
        // get users list
        let user_data = shared.users();
        let users: Vec<&str> = user_data.cache().keys().map(|n| n.as_str()).collect();

        // return users
        Ok(jsonify(object!(users: users)))
    } else {
        Fail::from("unauthenticated")
    }
}

/// Change user handler
pub fn change(req: HttpRequest, shared: RwLockReadGuard<'_, SharedData>) -> Result<Vec<u8>> {
    // get values
    let headers = req.headers();
    let username = get_username(headers)?;
    let token = get_str(headers, "token")?;
    let user = get_str(headers, "user")?;
    let password = get_str(headers, "password")?;
    let new_username = get_an(headers, "newusername");

    // verify login
    if shared.logins().valid(username, token) {
        let mut user_data = shared.users_mut();

        // change password
        if let Some(user_password) = user_data.cache_mut().get_mut(user) {
            // hash and change password
            *user_password = password.to_string();
            user_data.write()?;
        } else {
            return Fail::from("user does not exist");
        }

        // change username
        match new_username {
            Ok(new_username) => {
                // borrow users mutably
                let users = user_data.cache_mut();

                // check if user already exists
                if users.contains_key(new_username) {
                    return Fail::from("new username already exists");
                }

                // rename user
                let password_hash = users
                    .remove(user)
                    .ok_or_else(|| Fail::new("user does not exist"))?;
                users.insert(new_username.to_string(), password_hash);
                user_data.write()?;
                shared.logins_mut().rename(user, new_username.to_string());
            }
            Err(err) => {
                if err.to_string() == "newusername is not alphanumeric" {
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
