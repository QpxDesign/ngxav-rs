use crate::utils::parse_nginx_time_format::parse_nginx_time_format;
use chrono::prelude::*;
use regex::Regex;
use std::collections::HashMap;

#[derive(Clone)]
pub struct SessionOccurrences {
    pub ip_address: String,
    pub lines: Vec<String>,
    pub times: Vec<DateTime<Utc>>,
    pub sessions: Vec<Vec<String>>,
}

pub fn sessionize(
    lines: Vec<crate::structs::LineParseResult::LineParseResult>,
    unique_by: Option<String>,
) -> Vec<SessionOccurrences> {
    let session_cutoff_min = 10;
    let mut occurrences: HashMap<String, SessionOccurrences> = HashMap::new();
    let mut r = Regex::new("").unwrap();
    if unique_by.is_some() {
        let u = &unique_by.clone().unwrap();
        r = Regex::new(&u).unwrap();
    }
    for parsed_line in lines {
        let mut key: String = "".to_string();
        if unique_by.is_some() {
            let m = r.find(parsed_line.full_text);
            if m.is_some() {
                key = m.unwrap().as_str().to_string();
            } else {
                continue;
            }
        } else {
            key = parsed_line.ip_address.to_string()
        }
        if &key != "-" {
            let time: DateTime<Utc> = parse_nginx_time_format(&parsed_line.time);
            if !occurrences.contains_key(&key) {
                let mut l = Vec::new();
                l.push(parsed_line.full_text.to_string());
                let mut t = Vec::new();
                t.push(time);
                occurrences.insert(
                    key.clone(),
                    SessionOccurrences {
                        ip_address: key.clone(),
                        lines: l,
                        times: t,
                        sessions: Vec::new(),
                    },
                );
            } else {
                occurrences
                    .get_mut(key.as_str())
                    .unwrap()
                    .lines
                    .push(parsed_line.full_text.to_string());
                occurrences.get_mut(key.as_str()).unwrap().times.push(time);
            }
        }
    }
    let mut o = occurrences.clone();
    for entry in o.values_mut() {
        let mut index = 0;
        let mut tmp: Vec<String> = Vec::new();

        for l in &entry.times {
            if index + 1 == entry.times.len() {
                tmp.push(entry.lines[0].clone());
                entry.sessions.push(tmp);
                tmp = Vec::new();
            } else if index == 0 {
                tmp.push(entry.lines[0].clone());
            } else if l.timestamp() - entry.times[index - 1].timestamp() < (session_cutoff_min * 60)
            {
                tmp.push(entry.lines[index].clone());
            } else {
                entry.sessions.push(tmp);
                tmp = Vec::new();
            }
            index += 1;
        }
    }
    return o.to_owned().into_values().collect();
}
