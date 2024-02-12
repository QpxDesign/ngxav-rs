#[path = "../structs/mod.rs"]
mod structs;
use rayon::slice::ParallelSliceMut;

use crate::structs::LineParseResult::LineParseResult;

pub fn sort_by_body_size(log_selection: Vec<LineParseResult>, n: usize) {
    let mut ls = log_selection.clone();
    ls.par_sort_by_key(|a| a.body_bytes_sent.clone());
    if ls.len() < n {
        ls = ls[0..ls.len()].to_vec();
    } else {
        ls = ls[0..n].to_vec();
    }
    for line in ls {
        println!(
            "{size_mb} {ip} {host} {request}",
            size_mb = bytes_size_formatter(line.body_bytes_sent),
            ip = line.ip_address,
            host = line.host,
            request = line.request
        );
    }
}

fn bytes_size_formatter(b: i64) -> String {
    let f = b.abs();
    if f > 1024 * 1024 * 1024 {
        return format!("{num} GB", num = f / 1024 / 1024 / 1024);
    }
    if f > 1024 * 1024 {
        return format!("{num} MB", num = f / 1024 / 1024);
    }
    return format!("{num} KB", num = f / 1024);
}
