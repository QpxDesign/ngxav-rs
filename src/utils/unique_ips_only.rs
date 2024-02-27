#[path = "../structs/mod.rs"]
mod structs;
use crate::structs::LineParseResult::LineParseResult;
use std::collections::HashMap;

pub fn unique_ips_only(parsed_lines: Vec<LineParseResult>) -> Vec<LineParseResult> {
    let mut occurrences: HashMap<&str, LineParseResult> = HashMap::new();

    for line in parsed_lines {
        if occurrences.contains_key(&line.ip_address) == false {
            occurrences.insert(line.ip_address, line);
        }
    }
    return occurrences.into_iter().map(|(_, v)| v).collect();
}
