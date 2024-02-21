use crate::utils::parse_nginx_time_format::parse_nginx_time_format;
use chrono::prelude::*;
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
) -> Vec<SessionOccurrences> {
    let session_cutoff_min = 10;
    let mut occurrences: HashMap<String, SessionOccurrences> = HashMap::new();

    for parsed_line in lines {
        if parsed_line.ip_address != "-" {
            let time: DateTime<Utc> = parse_nginx_time_format(parsed_line.time.as_str());
            if !occurrences.contains_key(&parsed_line.ip_address) {
                let cl = parsed_line.ip_address.to_owned();
                let mut l = Vec::new();
                l.push(parsed_line.full_text);
                let mut t = Vec::new();
                t.push(time);
                occurrences.insert(
                    parsed_line.ip_address,
                    SessionOccurrences {
                        ip_address: cl,
                        lines: l,
                        times: t,
                        sessions: Vec::new(),
                    },
                );
            } else {
                occurrences
                    .get_mut(&parsed_line.ip_address)
                    .unwrap()
                    .lines
                    .push(parsed_line.full_text);
                occurrences
                    .get_mut(&parsed_line.ip_address)
                    .unwrap()
                    .times
                    .push(time);
            }
        }
    }

    for entry in occurrences.values_mut() {
        let mut index = 0;
        let mut tmp: Vec<String> = Vec::new();
        if &entry.times.len() != &entry.lines.len() {
            println!("{}", "CANARY");
        }
        for l in &entry.times {
            if index + 1 == entry.times.len() {
                tmp.push(entry.lines[0].clone());
                entry.sessions.push(tmp.clone());
                tmp = Vec::new();
            } else if index == 0 {
                tmp.push(entry.lines[0].clone());
            } else if l.timestamp() - entry.times[index - 1].timestamp() < (session_cutoff_min * 60)
            {
                tmp.push(entry.lines[index].clone());
            } else {
                entry.sessions.push(tmp.clone());
                tmp = Vec::new();
            }
            index += 1;
        }
    }
    return occurrences.into_values().collect();
}
