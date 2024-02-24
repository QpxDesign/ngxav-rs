use crate::utils::parse_line::parse_line;
use crate::utils::sessionize::sessionize;
use crate::utils::sort_by_date::sort_by_date;

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
    for o in sort_by_date(out, &None, &None, &None) {
        println!("{}", o.full_text + "\n");
    }
}
