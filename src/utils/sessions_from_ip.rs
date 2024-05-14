use crate::utils::parse_line::parse_line;
use crate::utils::sessionize::sessionize;
use regex::Regex;

pub fn sessions_from_ip(
    log_selection: Vec<crate::structs::LineParseResult::LineParseResult>,
    ip: Option<String>,
    unique_by: Option<String>,
) {
    let sessions = sessionize(log_selection, unique_by.clone());
    let mut host_paths: Vec<Vec<String>> = Vec::new();
    let mut session_start_times: Vec<String> = Vec::new();
    let mut session_end_times: Vec<String> = Vec::new();
    for session_entry in sessions {
        let mut key: String = "".to_string();
        if ip.clone().is_some() {
            key = ip.clone().unwrap();
        } else if unique_by.clone().is_some() {
            let u = unique_by.clone().unwrap();
            let r = Regex::new(&regex::escape(&u)).unwrap();
            let m = r.find(&session_entry.lines[0]);
            if m.is_some() {
                key = m.unwrap().clone().as_str().to_string();
            } else {
                continue;
            }
        }
        if session_entry.ip_address == key {
            for session in session_entry.sessions {
                let mut host_path: Vec<String> = Vec::new();
                if session.len() != 0 && session.len() != 1 {
                    session_start_times.push(parse_line(&session[0]).time);
                    session_end_times.push((parse_line(&session[session.len() - 1]).time).clone());
                    for line in session {
                        let h = parse_line(&line).host;
                        //  if len(host_path) == 0 or host_path[-1] != parse_line(line)["host"]:
                        if host_path.len() == 0 || host_path[host_path.len() - 1] != h {
                            host_path.push(h.to_string());
                        }
                    }
                    host_paths.push(host_path);
                }
            }
        }
    }
    let mut index = 0;
    for path in host_paths {
        println!("------------------------------");
        println!("======= {s}", s = session_start_times[index]);
        println!(
            "{a}",
            a = StringVecToKey(path)
                .replace(",", " -->")
                .replace("[", "")
                .replace("]", "")
        );
        println!("======= {a}", a = session_end_times[index]);
        println!("------------------------------");
        index += 1;
    }
}

fn StringVecToKey(sv: Vec<String>) -> String {
    let mut ans: String = "[".to_string();

    for item in sv {
        let s = format!("{item}, ", item = item);
        ans.push_str(&s);
    }
    return ans + "]";
}
