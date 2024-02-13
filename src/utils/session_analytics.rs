use crate::utils::parse_line::parse_line;
use crate::utils::{parse_line, sessionize::sessionize};
use std::collections::HashMap;
use std::path;

struct HostPath {
    path: String,
    count: i64,
}
struct SessionAnalysisStats {
    total_count: i64,
    host_paths: HashMap<String, HostPath>,
    average_request_count: usize,
    average_request_length: i64,
}
pub fn session_analytics(log_selection: Vec<crate::structs::LineParseResult::LineParseResult>) {
    let mut sessions = sessionize(log_selection);
    let mut stats: SessionAnalysisStats = SessionAnalysisStats {
        total_count: 0,
        host_paths: HashMap::new(),
        average_request_count: 0,
        average_request_length: 0,
    };

    stats.average_request_count =
        (stats.average_request_count as usize) / ((stats.total_count + 1) as usize);
    stats.average_request_length = stats.average_request_length / (stats.total_count + 1);

    let mut ips_text: String = "".to_string();
    let mut ip_index = 0;

    sessions.sort_by_key(|a| a.sessions.len());
    sessions.reverse();
    for s in sessions {
        stats.total_count += 1;
        let mut host_path: Vec<String> = [].to_vec();
        for l in s.lines {
            let a = parse_line(l.as_str()).host;
            if host_path.len() == 0 || host_path[host_path.len() - 1] != a {
                host_path.push(a);
            }
            /*
                   if str(host_path) not in stats["host_paths"]:
                stats["host_paths"][str(host_path)] = {
                    "path": str(host_path),
                    "count":1,
                }
            else:
                stats["host_paths"][str(host_path)]["count"] += 1
            */
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
    let a: Vec<&HostPath> = stats.host_paths.values().collect();
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
        stats_arc = stats.average_request_count,
        stats_asl = stats.average_request_length,
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
