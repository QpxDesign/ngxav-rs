use crate::utils::parse_line::parse_line;
use crate::utils::parse_nginx_time_format::parse_nginx_time_format;
use crate::utils::sessionize::sessionize;
use rayon::slice::ParallelSliceMut;

pub fn session_unique(
    log_selection: Vec<crate::structs::LineParseResult::LineParseResult>,
    unique_by: Option<String>,
) {
    let sessions = sessionize(log_selection, unique_by);
    let mut out: Vec<String> = Vec::new();
    for ip_session in sessions {
        for session in ip_session.sessions {
            if session.len() != 0 {
                out.push(session[0].clone());
            }
        }
    }
    out.par_sort_by_key(|a| parse_nginx_time_format(&parse_line(&a).time).timestamp());
    for o in out {
        println!("{}", parse_line(&o).full_text.to_owned() + "\n");
    }
}
