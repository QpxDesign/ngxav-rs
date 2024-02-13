use crate::utils::parse_nginx_time_format::parse_nginx_time_format;
use rayon::slice::ParallelSliceMut;

pub fn sort_by_date(
    log_selection: Vec<crate::structs::LineParseResult::LineParseResult>,
) -> Vec<crate::structs::LineParseResult::LineParseResult> {
    let mut ls = log_selection;
    ls.par_sort_by_key(|a| parse_nginx_time_format(a.time.as_str()).timestamp());
    return ls;
}
