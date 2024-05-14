#[path = "../structs/mod.rs"]
mod structs;
use crate::structs::LineParseResult::LineParseResult;
use lazy_static::lazy_static;
use regex::Regex;
use std::collections::HashMap;

pub fn unique_ips_only(
    parsed_lines: Vec<LineParseResult>,
    unique_by: Option<String>,
) -> Vec<LineParseResult> {
    let mut occurrences: HashMap<&str, LineParseResult> = HashMap::new();
    let mut r = Regex::new("").unwrap();
    if unique_by.is_some() {
        r = Regex::new(&unique_by.clone().unwrap()).unwrap();
    }
    for line in parsed_lines {
        let mut u: String = "".to_string();
        if unique_by.is_some() {
            let m = r.find(line.full_text);
            if m.is_some() {
                occurrences.insert(m.unwrap().as_str(), line);
            }
        } else {
            if occurrences.contains_key(&line.ip_address) == false {
                occurrences.insert(line.ip_address, line);
            }
        }
    }
    return occurrences.into_iter().map(|(_, v)| v).collect();
}
