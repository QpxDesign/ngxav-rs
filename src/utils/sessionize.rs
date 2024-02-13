use crate::utils::parse_line::parse_line;
use crate::utils::parse_nginx_time_format::parse_nginx_time_format;
use chrono::prelude::*;
use std::collections::btree_map::Entry;
use std::collections::HashMap;

#[derive(Clone)]
pub struct SessionOccurrences {
    pub ip_address: String,
    pub lines: Vec<String>,
    pub times: Vec<DateTime<Utc>>,
    pub sessions: Vec<Vec<String>>,
}
pub fn sessionize(lines: Vec<std::string::String>) -> Vec<SessionOccurrences> {
    let session_cutoff_min = 10;
    let mut occurrences: HashMap<String, SessionOccurrences> = HashMap::new();
    for line in lines {}
    return occurrences.into_values().collect();
}
