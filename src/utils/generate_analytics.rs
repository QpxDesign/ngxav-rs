#[path = "../structs/mod.rs"]
mod structs;
use crate::utils::parse_line::parse_line;
use std::collections::HashMap;

pub fn generate_analytical_output(log_selection: Vec<std::string::String>) {
    let mut stats: structs::AnalyticsResult::AnalyticsResult =
        structs::AnalyticsResult::AnalyticsResult {
            request_count: 0,
            top_requests: HashMap::new(),
            top_hosts: HashMap::new(),
            top_ips: HashMap::new(),
        };
    for line in log_selection {
        let parsed_line = parse_line(line.as_str()).clone();
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
    }
    let mut top_hosts: Vec<_> = stats.top_hosts.values().collect();
    let mut top_ips: Vec<_> = stats.top_ips.values().collect();
    let mut top_requests: Vec<_> = stats.top_requests.values().collect();
    top_hosts.sort_by_key(|a| a.count);
    top_hosts.reverse();

    top_requests.sort_by_key(|a| a.count);
    top_requests.reverse();

    top_ips.sort_by_key(|a| a.count);
    top_ips.reverse();

    println!(
        "
    ===~ LOG SELECTION STATS ~===
Total Requests: {total_requests}

Top 5 Requests:
{top5requests}
Top 5 Hosts:
{top5hosts}
Top 5 IP Addresses:
{top5ips}",
        total_requests = stats.request_count,
        top5requests = TopResultToString(top_requests, 5),
        top5hosts = TopResultToString(top_hosts, 5),
        top5ips = TopResultToString(top_ips, 5)
    )
}

fn TopResultToString(data: Vec<&structs::AnalyticsResult::TopResult>, n: usize) -> String {
    let mut rel_data: Vec<&structs::AnalyticsResult::TopResult> = [].to_vec();
    if data.len() < n {
        rel_data = data[0..data.len()].to_vec();
    } else {
        rel_data = data[0..n].to_vec();
    }
    let mut ans: String = "".to_string();

    for line in rel_data {
        ans += &format!("- {t} ~ {c} \n", t = line.text, c = line.count).to_string();
    }
    return ans;
}
