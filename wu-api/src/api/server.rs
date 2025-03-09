//! Servers API

use crate::common::*;
use jzon::JsonValue;
use kern::http::server::HttpRequest;
use wu::{Fail, Result};

/// List server statistics handler
pub fn stats(req: HttpRequest, shared: &SharedData) -> Result<Vec<u8>> {
    // get values
    let headers = req.headers();
    let username = get_username(headers)?;
    let token = get_str(headers, "token")?;

    // verify login
    if shared.logins().valid(username, token) {
        // get statistics
        let mut stats = JsonValue::new_object();
        shared.statistics().iter().for_each(|(k, v)| {
            stats[k] = JsonValue::new_object();
            stats[k]["cpu"] = (*v.cpu()).into();

            let mem = v.mem();
            stats[k]["memused"] = mem.0.into();
            stats[k]["memtotal"] = mem.1.into();

            let disk = v.disk();
            stats[k]["diskused"] = disk.0.into();
            stats[k]["disktotal"] = disk.1.into();
        });

        // return servers list
        Ok(jsonify(object!(stats: stats)))
    } else {
        Fail::from("unauthenticated")
    }
}
