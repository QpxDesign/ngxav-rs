use rayon::slice::ParallelSliceMut;
use crate::utils::parse_line::parse_line;

pub fn sort_by_body_size(log_selection: Vec<std::string::String>) -> Vec<std::string::String> {
    println!("STARTED SORT");
    let mut ls = log_selection.clone();
    ls.par_sort_by_key(|a| parse_line(a.as_str()).body_bytes_sent);
    ls.reverse();
    return ls;
}
