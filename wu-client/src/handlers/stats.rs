//! Server stats handler

use crate::utils::cpu_mem_usage;
use std::time::Duration;
use wu::net::Connection;
use wu::Command;

pub fn send_stats(mut conn: Connection, _cmd: Command) {
    let (rx, _) = cpu_mem_usage(Duration::from_secs(5));

    let mut buf = Vec::with_capacity(40);
    while let Ok((cpu_usage, (mem_used, mem_total), (disk_used, disk_total))) =
        rx.recv_timeout(Duration::from_secs(10))
    {
        buf.extend_from_slice(&cpu_usage.to_be_bytes());
        buf.extend_from_slice(&mem_used.to_be_bytes());
        buf.extend_from_slice(&mem_total.to_be_bytes());
        buf.extend_from_slice(&disk_used.to_be_bytes());
        buf.extend_from_slice(&disk_total.to_be_bytes());
        conn.write(&buf).unwrap();
        buf.clear();
    }
}
