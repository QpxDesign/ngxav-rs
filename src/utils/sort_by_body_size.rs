#[path = "../structs/mod.rs"]
mod structs;
use crate::structs::LineParseResult::LineParseResult;
use rayon::slice::ParallelSliceMut;
use std::collections::HashMap;

pub fn sort_by_body_size(log_selection: Vec<LineParseResult>, n: usize) {
    let mut parsed_lines = log_selection.clone();
    let mut occurrences: HashMap<String, LineParseResult> = HashMap::new();
    for line in parsed_lines {
        if occurrences.contains_key(&line.request) == false {
            occurrences.insert(line.request.clone(), line);
        }
    }
    let mut final_lines: Vec<_> = occurrences.values().cloned().collect();
    final_lines.par_sort_by_key(|a: &LineParseResult| a.body_bytes_sent.clone());

    final_lines.par_sort_by_key(|a| a.body_bytes_sent.clone());
    if final_lines.len() < n {
        final_lines = final_lines[0..final_lines.len()].to_vec();
    } else {
        final_lines = final_lines[0..n].to_vec();
    }

    for line in final_lines {
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
