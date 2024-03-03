#[path = "../structs/mod.rs"]
mod structs;
use crate::structs::LineParseResult::LineParseResult;
use std::collections::HashMap;

pub fn generate_analytical_output(log_selection: Vec<LineParseResult>) {
    let mut stats: structs::AnalyticsResult::AnalyticsResult =
        structs::AnalyticsResult::AnalyticsResult {
            request_count: 0,
            total_bytes_sent: 0,
            top_requests: HashMap::new(),
            top_hosts: HashMap::new(),
            top_ips: HashMap::new(),
        };
    for parsed_line in log_selection {
        let request = parsed_line.request.clone();
        let host = parsed_line.host.clone();
        let ip = parsed_line.ip_address.clone();
        stats.request_count += 1;
        stats.total_bytes_sent += parsed_line.body_bytes_sent.abs();
        if stats.top_requests.contains_key(&parsed_line.request) == false {
            stats.top_requests.insert(
                parsed_line.request,
                structs::AnalyticsResult::TopResult {
                    text: parsed_line.request,
                    count: 0,
                },
            );
        }
        stats.top_requests.get_mut(&request).unwrap().count += 1;

        if stats.top_hosts.contains_key(&parsed_line.host) == false {
            stats.top_hosts.insert(
                parsed_line.host,
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
Total Data Sent: {td}

Top 5 Requests:
{top5requests}
Top 5 Hosts:
{top5hosts}
Top 5 IP Addresses:
{top5ips}",
        total_requests = stats.request_count,
        td = bytes_size_formatter(stats.total_bytes_sent),
        top5requests = top_result_to_string(top_requests, 5),
        top5hosts = top_result_to_string(top_hosts, 5),
        top5ips = top_result_to_string(top_ips, 5)
    )
}

fn top_result_to_string(data: Vec<&structs::AnalyticsResult::TopResult>, n: usize) -> String {
    let rel_data: Vec<&structs::AnalyticsResult::TopResult>;
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
