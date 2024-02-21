use crate::utils::parse_line::parse_line;
use crate::utils::{parse_line, parse_nginx_time_format, sessionize::sessionize};
use std::collections::HashMap;

struct HostPath {
    path: String,
    count: i64,
}
struct SessionAnalysisStats {
    total_count: i64,
    host_paths: HashMap<String, HostPath>,
    average_request_count: i64,
    average_request_length: i64,
    request_count_sum: i64,
    request_length_sum: i64,
}
pub fn session_analytics(log_selection: Vec<crate::structs::LineParseResult::LineParseResult>) {
    let mut sessions = sessionize(log_selection);
    let mut stats: SessionAnalysisStats = SessionAnalysisStats {
        total_count: 0,
        host_paths: HashMap::new(),
        average_request_count: 0,
        request_count_sum: 0,
        request_length_sum: 0,
        average_request_length: 0,
    };

    let mut ips_text: String = "".to_string();
    let mut ip_index = 0;

    sessions.sort_by_key(|a| a.sessions.len());
    sessions.reverse();
    for s in sessions {
        stats.request_count_sum += i64::try_from(s.lines.len()).unwrap();
        let mut host_path: Vec<String> = [].to_vec();
        for ses in s.sessions.clone() {
            stats.total_count += 1;
            if ses.len() > 1 {
                let a = (parse_nginx_time_format::parse_nginx_time_format(
                    &parse_line(ses[0].as_str()).time,
                )
                .timestamp()
                    - parse_nginx_time_format::parse_nginx_time_format(
                        &parse_line(ses[ses.len() - 1].as_str()).time,
                    )
                    .timestamp())
                .abs();

                if a < 60 * 60 {
                    stats.request_length_sum += (parse_nginx_time_format::parse_nginx_time_format(
                        &parse_line(ses[0].as_str()).time,
                    )
                    .timestamp()
                        - parse_nginx_time_format::parse_nginx_time_format(
                            &parse_line(ses[ses.len() - 1].as_str()).time,
                        )
                        .timestamp())
                    .abs();
                }
            }
        }
        for l in s.lines {
            let a = parse_line(l.as_str()).host;
            if host_path.len() == 0 || host_path[host_path.len() - 1] != a {
                host_path.push(a);
            }
        }
        let str_key = &StringVecToKey(host_path);
        if !stats.host_paths.contains_key(str_key) {
            stats.host_paths.insert(
                str_key.to_string(),
                HostPath {
                    path: str_key.to_string(),
                    count: 1,
                },
            );
        } else {
            stats.host_paths.get_mut(str_key).unwrap().count += 1;
        }

        if ip_index <= 10 {
            ips_text = ips_text
                + format!(
                    "- {ip} - {num}\n",
                    ip = s.ip_address,
                    num = s.sessions.len()
                )
                .as_str();
            ip_index += 1
        }
    }
    let mut host_text: String = "".to_string();
    let mut h_index = 0;
    let mut a: Vec<&HostPath> = stats.host_paths.values().collect();
    a.sort_by_key(|a| a.count);
    a.reverse();
    for path_entry in a {
        if h_index < 5 {
            host_text = host_text
                + format!(
                    "- {pe} [{cnt}]\n",
                    pe = path_entry
                        .path
                        .replace(",", " -->")
                        .replace("[", "")
                        .replace("]", ""),
                    cnt = path_entry.count
                )
                .as_str();
            h_index += 1;
        }
    }
    println!(
        "
SESSION STATS
==============
{stats_tc} Total Unique Sessions
{stats_arc} Avg Requests Per Session
{stats_asl}min Avg Session Length

MOST COMMON PATHS
===================
{h_text}
IPS WITH MOST SESSIONS
======================
{ips_txt}
    ",
        stats_tc = stats.total_count,
        stats_arc = stats.request_count_sum / stats.total_count,
        stats_asl = ((stats.request_length_sum / 60) / stats.total_count),
        h_text = host_text,
        ips_txt = ips_text
    )
}
fn StringVecToKey(sv: Vec<String>) -> String {
    let mut ans: String = "[".to_string();

    for item in sv {
        let s = format!("{item}, ", item = item.as_str());
        ans.push_str(&s);
    }
    return ans + "]";
}
