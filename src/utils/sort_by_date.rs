use crate::utils::parse_line::parse_line;
use crate::utils::parse_nginx_time_format::parse_nginx_time_format;
use rayon::slice::ParallelSliceMut;
pub fn sort_by_date(log_selection: Vec<std::string::String>) -> Vec<std::string::String> {
    let mut ls = log_selection.clone();
    ls.par_sort_by_key(|a| parse_nginx_time_format(&parse_line(a.as_str()).time).timestamp());
    return ls;
}
