use crate::utils::parse_input_time::parse_input_time;
use crate::utils::parse_nginx_time_format::parse_nginx_time_format;
use std::time::{SystemTime, UNIX_EPOCH};

pub fn sort_by_date(
    log_selection: &Vec<String>,
    last_min: &Option<u64>,
    start_date: &Option<String>,
    end_date: &Option<String>,
) -> (usize, usize) {
    if log_selection.len() == 0 {
        return (0, 0);
    }
    let tz = log_selection[0].split(" ").collect::<Vec<_>>()[4]
        .to_string()
        .replace("]", "");

    if !last_min.is_none() {
        let start = SystemTime::now();
        let since_the_epoch = start
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards");
        let epoch_seconds_end: u64 = since_the_epoch.as_secs();
        let epoch_seconds_start = epoch_seconds_end - 60 * last_min.unwrap();
        return b_search(
            &log_selection,
            epoch_seconds_start.try_into().unwrap(),
            epoch_seconds_end.try_into().unwrap(),
        );
    }
    if !start_date.is_none() && !end_date.is_none() {
        return b_search(
            &log_selection,
            parse_input_time(&start_date.as_ref().unwrap(), tz.clone()).timestamp(),
            parse_input_time(&end_date.as_ref().unwrap(), tz.clone()).timestamp(),
        );
    }
    if !end_date.is_none() && start_date.is_none() {
        return b_search(
            &log_selection,
            0,
            parse_input_time(&end_date.as_ref().unwrap(), tz).timestamp(),
        );
    }
    if !start_date.is_none() && end_date.is_none() {
        let start = SystemTime::now();
        let since_the_epoch = start
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards");
        return b_search(
            &log_selection,
            parse_input_time(&start_date.as_ref().unwrap(), tz).timestamp(),
            since_the_epoch.as_secs().try_into().unwrap(),
        );
    }
    return (0, log_selection.len() - 1);
}

fn b_search(logs: &Vec<String>, start_time_range: i64, end_time_range: i64) -> (usize, usize) {
    let st = logs.partition_point(|x| {
        let fields = x.split(" ").collect::<Vec<_>>();
        let t = fields[3].replace("[", "") + " " + &fields[4].replace("]", "");
        parse_nginx_time_format(&t).timestamp() < start_time_range
    });
    let en = logs[st..].partition_point(|x| {
        let fields = x.split(" ").collect::<Vec<_>>();
        let t = fields[3].replace("[", "") + " " + &fields[4].replace("]", "");
        return parse_nginx_time_format(&t).timestamp() < end_time_range;
    });

    return (st, en + st);
}
