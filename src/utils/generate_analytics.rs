#[path = "../structs/mod.rs"]
mod structs;
use crate::structs::AnalyticsResult::AnalyticsResult;
use crate::structs::AnalyticsResult::TopResult;
use crate::structs::LineParseResult;
use crate::utils::parse_line::parse_line;
use std::collections::HashMap;

pub fn generate_analytical_output(log_selection: Vec<&str>) {
    println!("{}", log_selection.len());
    let mut stats: structs::AnalyticsResult::AnalyticsResult =
        structs::AnalyticsResult::AnalyticsResult {
            request_count: 0,
            top_requests: HashMap::new(),
            top_hosts: HashMap::new(),
            top_ips: HashMap::new(),
        };
    for line in log_selection {
        println!("readline");
        let parsed_line = parse_line(line).clone();
        let request = parsed_line.request.clone();
        let host = parsed_line.host.clone();
        let ip = parsed_line.ip_address.clone();
        stats.request_count += 1;
        if stats.top_requests.contains_key(&parsed_line.request) == false {
            stats.top_requests.insert(
                parsed_line.request.to_string(),
                structs::AnalyticsResult::TopResult {
                    text: parsed_line.request,
                    count: 0,
                },
            );
        }
        stats.top_requests.get_mut(&request).unwrap().count += 1;

        if stats.top_hosts.contains_key(&parsed_line.host) == false {
            stats.top_hosts.insert(
                parsed_line.host.to_string(),
                structs::AnalyticsResult::TopResult {
                    text: parsed_line.host,
                    count: 0,
                },
            );
        }
        stats.top_hosts.get_mut(&host).unwrap().count += 1;

        if stats.top_ips.contains_key(&parsed_line.ip_address) == false {
            stats.top_ips.insert(
                parsed_line.ip_address,
                structs::AnalyticsResult::TopResult {
                    text: ip.clone(),
                    count: 0,
                },
            );
        }
        stats.top_ips.get_mut(&ip).unwrap().count += 1;

        let mut top_hosts: Vec<_> = stats.top_hosts.values().collect();
        let mut top_ips: Vec<_> = stats.top_ips.values().collect();
        let mut top_requests: Vec<_> = stats.top_requests.values().collect();
        top_hosts.sort_by(|a, b| {
            let first = a.count.cmp(&b.count);
            let second = a.count.cmp(&b.count);
            first.then(second)
        });
        top_ips.sort_by(|a, b| {
            let first = a.count.cmp(&b.count);
            let second = a.count.cmp(&b.count);
            first.then(second)
        });
        top_requests.sort_by(|a, b| {
            let first = a.count.cmp(&b.count);
            let second = a.count.cmp(&b.count);
            first.then(second)
        });
    }
}
