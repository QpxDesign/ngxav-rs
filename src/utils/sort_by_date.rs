use crate::utils::parse_input_time::parse_input_time;
use crate::utils::parse_nginx_time_format::parse_nginx_time_format;
use rayon::slice::ParallelSliceMut;
use std::time::{SystemTime, UNIX_EPOCH};

pub fn sort_by_date(
    log_selection: Vec<crate::structs::LineParseResult::LineParseResult>,
    last_min: &Option<u64>,
    start_date: &Option<String>,
    end_date: &Option<String>,
) -> Vec<crate::structs::LineParseResult::LineParseResult> {
    let tz = log_selection[0].time.split(" ").collect::<Vec<_>>()[1].to_string();
    let mut ls = log_selection.clone();
    if !last_min.is_none() {
        let start = SystemTime::now();
        let since_the_epoch = start
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards");
        let epoch_seconds_end: u64 = since_the_epoch.as_secs();
        let epoch_seconds_start = epoch_seconds_end - 60 * last_min.unwrap();
        ls = b_search(
            &log_selection,
            epoch_seconds_start.try_into().unwrap(),
            epoch_seconds_end.try_into().unwrap(),
        );
    } else if !start_date.is_none() && !end_date.is_none() {
        ls = b_search(
            &log_selection,
            parse_input_time(&start_date.as_ref().unwrap(), tz.clone()).timestamp(),
            parse_input_time(&end_date.as_ref().unwrap(), tz.clone()).timestamp(),
        );
    } else if !end_date.is_none() {
        ls = b_search(
            &log_selection,
            0,
            parse_input_time(&end_date.as_ref().unwrap(), tz).timestamp(),
        );
    } else if !start_date.is_none() {
        let start = SystemTime::now();
        let since_the_epoch = start
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards");

        ls = b_search(
            &log_selection,
            parse_input_time(&start_date.as_ref().unwrap(), tz).timestamp(),
            since_the_epoch.as_secs().try_into().unwrap(),
        );
    }
    return ls;
}

fn b_search(
    logs: &Vec<crate::structs::LineParseResult::LineParseResult>,
    start_time_range: i64,
    end_time_range: i64,
) -> Vec<crate::structs::LineParseResult::LineParseResult> {
    let st =
        logs.partition_point(|x| parse_nginx_time_format(&x.time).timestamp() < start_time_range);
    let en = logs[st..]
        .partition_point(|x| parse_nginx_time_format(&x.time).timestamp() < end_time_range)
        + st;

    return logs[st..en].to_vec();
}
