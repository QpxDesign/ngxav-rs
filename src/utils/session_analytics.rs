use std::collections::HashMap;

use crate::utils::parse_line::parse_line;
use crate::utils::sessionize::sessionize;

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
pub fn session_analytics(log_selection: Vec<std::string::String>) {
    let mut sessions = sessionize(log_selection);
    let mut stats: SessionAnalysisStats = SessionAnalysisStats {
        total_count: 0,
        host_paths: HashMap::new(),
        average_request_count: 0,
        average_request_length: 0,
    };

    println!(
        "
SESSION STATS
==============
{stats_tc} Total Unique Sessions
{stats_arc} Avg Requests Per Session
{stats_asl}min Avg Session Length


IPS WITH MOST SESSIONS
======================
{ips_txt}
    ",
        stats_tc = stats.total_count,
        stats_arc = stats.average_request_count,
        stats_asl = stats.average_request_length,
        ips_txt = "a"
    )
}

fn StringVecToKey(sv: Vec<String>) -> String {
    let mut ans: String = "[".to_string();

    for item in sv {
        let s = format!("{item}, ", item = item.as_str());
        ans.push_str(&s);
    }
    return ans;
}
