#[path = "../structs/mod.rs"]
mod structs;
use crate::utils::parse_line::parse_line;
use rayon::prelude::*;
use rayon::slice::ParallelSliceMut;
pub fn sort_by_body_size(log_selection: Vec<String>, n: usize) {
    let mut parsed_lines: Vec<crate::structs::LineParseResult::LineParseResult> = log_selection
        .par_iter()
        .map(|l: &String| parse_line(l))
        .collect();

    parsed_lines.par_sort_by_key(|a| a.body_bytes_sent.clone());
    if parsed_lines.len() < n {
        parsed_lines = parsed_lines[0..parsed_lines.len()].to_vec();
    } else {
        parsed_lines = parsed_lines[0..n].to_vec();
    }
    for line in parsed_lines {
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
