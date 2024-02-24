use crate::utils::parse_line::parse_line;
use crate::utils::parse_nginx_time_format::parse_nginx_time_format;
use crate::utils::sessionize::sessionize;
use rayon::slice::ParallelSliceMut;

pub fn session_unique(log_selection: Vec<crate::structs::LineParseResult::LineParseResult>) {
    let sessions = sessionize(log_selection);
    let mut out: Vec<crate::structs::LineParseResult::LineParseResult> = Vec::new();
    for ip_session in sessions {
        for session in ip_session.sessions {
            if session.len() != 0 {
                out.push(parse_line(&session[0].clone()).clone());
            }
        }
    }
    out.par_sort_by_key(|a| parse_nginx_time_format(a.time.as_str()).timestamp());
    for o in out {
        println!("{}", o.full_text + "\n");
    }
}
