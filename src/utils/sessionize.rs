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

    for line in lines {
        let parsed_line = parse_line(&line);

        if parsed_line.ip_address != "-" {
            let time: DateTime<Utc> = parse_nginx_time_format(parsed_line.time.as_str());
            if !occurrences.contains_key(&parsed_line.ip_address) {
                let cl = parsed_line.ip_address.clone();
                let mut l = Vec::new();
                l.push(line);
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
                    .push(line);
                occurrences
                    .get_mut(&parsed_line.ip_address)
                    .unwrap()
                    .times
                    .push(time);
                let entry: Option<&SessionOccurrences> = occurrences.get(&parsed_line.ip_address);
                if entry.is_some() {
                    let mut sessions: Vec<Vec<String>> = Vec::new();
                    let mut index = 0;
                    let mut tmp: Vec<String> = Vec::new();
                    for l in &entry.unwrap().times {
                        if index == 0 {
                            tmp.push("a".to_string());
                        } else if l.timestamp() - entry.unwrap().times[index - 1].timestamp()
                            < session_cutoff_min * 60
                        {
                            tmp.push("a".to_string());
                        } else {
                            sessions.push(tmp);

                            tmp = Vec::new();
                        } //34.46s user 1.15s system 122% cpu 29.166 total
                        index += 1;
                    }
                    occurrences
                        .get_mut(&parsed_line.ip_address)
                        .unwrap()
                        .sessions = sessions;
                }
            }
        }
    }
    return occurrences.into_values().collect();
}
