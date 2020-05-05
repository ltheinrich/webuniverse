//! Server statistics handler

use crate::common::*;
use std::convert::TryInto;
use std::sync::RwLockReadGuard;
use wu::net::Connection;

pub fn send_stats(mut conn: Connection, shared: RwLockReadGuard<'_, SharedData>, name: String) {
    {
        // add statistics to map
        let mut stats = shared.statistics_mut();
        stats.insert(name.clone(), Statistics::new());
        // drop write-access
    }

    // read from client
    while let Ok(data) = conn.read() {
        if data.len() == 40 {
            // get statistics
            let stats = shared.statistics();
            let mut cpu = stats.get(&name).unwrap().cpu_mut();
            let mut mem = stats.get(&name).unwrap().mem_mut();
            let mut disk = stats.get(&name).unwrap().disk_mut();

            // update statistics
            *cpu = f64::from_be_bytes((&data[..8]).try_into().unwrap());
            mem.0 = u64::from_be_bytes((&data[8..16]).try_into().unwrap());
            mem.1 = u64::from_be_bytes((&data[16..24]).try_into().unwrap());
            disk.0 = u64::from_be_bytes((&data[24..32]).try_into().unwrap());
            disk.1 = u64::from_be_bytes((&data[32..]).try_into().unwrap());
        }
    }

    // remove server
    shared.statistics_mut().remove(&name);
}
