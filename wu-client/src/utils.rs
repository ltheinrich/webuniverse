//! Client utils

use std::fs::OpenOptions;
use std::io::prelude::*;
use std::process::Command;
use std::sync::mpsc::{channel, Receiver};
use std::thread::{sleep, spawn, JoinHandle};
use std::time::Duration;
use wu::Fail;

/// Resource usage receiver and thread
type ResourceUsageRx = (
    Receiver<(f64, (u64, u64), (u64, u64))>,
    JoinHandle<Result<(), Fail>>,
);

/// Get CPU and memory usage every duration except first
pub fn cpu_mem_usage(duration: Duration) -> ResourceUsageRx {
    // create channel
    let (tx, rc) = channel();

    // spawn sender thread
    let thread = spawn(move || {
        // read first cpu times and wait first second
        let (mut prev_idle, mut prev_total) = read_cpu_times()?;
        sleep(duration);

        // send cpu usage continously
        loop {
            // read cpu, memory and disk space and calculate difference for cpu
            let (idle_cpu, total_cpu) = read_cpu_times()?;
            let (used_mem, total_mem) = read_memory_usage()?;
            let (used_disk, total_disk) = read_disk_space()?;
            let dif_idle = (idle_cpu - prev_idle) as f64;
            let dif_total = (total_cpu - prev_total) as f64;

            // calculate cpu and memory usage and send
            let cpu_usage = ((1.0 - dif_idle / dif_total) * 10000.0).round() / 100.0;
            tx.send((cpu_usage, (used_mem, total_mem), (used_disk, total_disk)))
                .or_else(Fail::from)?;

            // set previous times and wait next second
            prev_idle = idle_cpu;
            prev_total = total_cpu;
            sleep(duration);
        }
    });

    // return receiver and thread
    (rc, thread)
}

/// Get CPU idle and total time from /proc/stat
fn read_cpu_times() -> Result<(u64, u64), Fail> {
    // open file
    let mut file = OpenOptions::new()
        .read(true)
        .open("/proc/stat")
        .or_else(Fail::from)?;

    // read file
    let mut buf = String::new();
    file.read_to_string(&mut buf).or_else(Fail::from)?;

    // only first line
    buf = {
        // split lines and get first
        let line = buf
            .split('\n')
            .next()
            .ok_or_else(|| Fail::new("broken /proc/stat"))?;

        // line should be at least 10 characters
        if line.len() < 10 {
            return Fail::from("broken /proc/stat");
        }

        // cut "cpu  "
        (&line[5..line.len()]).to_string()
    };

    // split by whitespace and get idle time
    let split: Vec<&str> = buf.split_ascii_whitespace().collect();
    let idle: u64 = split[3].parse().or_else(Fail::from)?;

    // calculate total from all times
    let mut total = 0u64;
    for s in split {
        total += s.parse::<u64>().or_else(Fail::from)?;
    }

    // return idle and total time
    Ok((idle, total))
}

/// Get memory used and total memory from /proc/meminfo
fn read_memory_usage() -> Result<(u64, u64), Fail> {
    // open file
    let mut file = OpenOptions::new()
        .read(true)
        .open("/proc/meminfo")
        .or_else(Fail::from)?;

    // read file
    let mut buf = String::new();
    file.read_to_string(&mut buf).or_else(Fail::from)?;

    // split by whitespace
    let split: Vec<&str> = buf.split_ascii_whitespace().collect();

    // parse total and used memory
    let mut total = 0u64;
    let mut used = 0u64;
    for s in split {
        if let Ok(u) = s.parse::<u64>() {
            if total == 0 {
                total = u;
            } else if used == 0 {
                used = 1;
            } else {
                if total < u {
                    return Fail::from("broken /proc/meminfo");
                }
                used = total - u;
                break;
            }
        }
    }

    // return used and total memory
    Ok((used, total))
}

/// Get used and total disk space from `df /`
fn read_disk_space() -> Result<(u64, u64), Fail> {
    // output of `df /`
    let out = Command::new("df").arg("/").output().or_else(Fail::from)?;
    let out = String::from_utf8(out.stdout).or_else(Fail::from)?;

    // split by whitespace
    let split: Vec<&str> = out
        .lines()
        .nth(1)
        .ok_or_else(|| Fail::new("broken `df /` output"))?
        .split_ascii_whitespace()
        .collect();

    // get used and total disk space
    let mut used = 0u64;
    let mut total = 0u64;
    for s in split {
        if let Ok(u) = s.parse::<u64>() {
            if total == 0 {
                total = u;
            } else {
                used = u;
                break;
            }
        }
    }

    // return used and total disk space
    Ok((used, total))
}
