use clap::Parser;
use regex::Regex;

use crate::structs::Args::ArgParser;
use crate::utils::parse_nginx_time_format::parse_nginx_time_format;

use std::time::{SystemTime, UNIX_EPOCH};

pub fn keep_line(line: String) -> bool {
    let args: crate::structs::Args::ArgParser = ArgParser::parse();
    let parsed_line = crate::utils::parse_line::parse_line(&line);

    let re = Regex::new(&args.search.clone().unwrap().to_string()).unwrap();
    if !args.search.is_none() && !re.is_match(&line) {
        return false;
    }
    if !args.start_date.is_none() && args.end_date.is_none() {
        if parse_nginx_time_format(&parsed_line.time)
            < parse_nginx_time_format(&args.start_date.clone().unwrap())
        {
            return false;
        }
    }
    if !args.end_date.is_none() && args.start_date.is_none() {
        if parse_nginx_time_format(&parsed_line.time)
            > parse_nginx_time_format(&args.end_date.clone().unwrap())
        {
            return false;
        }
    }
    if !args.start_date.is_none()
        && !args.end_date.is_none()
        && (parse_nginx_time_format(&parsed_line.time)
            > parse_nginx_time_format(&args.end_date.unwrap())
            || parse_nginx_time_format(&parsed_line.time)
                < parse_nginx_time_format(&args.start_date.unwrap()))
    {
        return false;
    }
    if !args.host.is_none() && parsed_line.host != args.host.unwrap() {
        return false;
    }
    if !args.request.is_none() && !parsed_line.request.contains(&args.request.unwrap()) {
        return false;
    }
    if !args.http_status.is_none() && parsed_line.status != args.http_status.unwrap() {
        return false;
    }
    if !args.referer.is_none() && parsed_line.referer != args.referer.unwrap() {
        return false;
    }
    let start = SystemTime::now();
    let since_the_epoch = start
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards");
    let epoch_seconds = since_the_epoch.as_millis() * 1000;
    if !args.last.is_none() {
        if parse_nginx_time_format(&parsed_line.time).timestamp() < epoch_seconds as i64 {
            return false;
        }
    }
    return true;
}
/*
parsed_line = parse_line(line)
        if args.last is not None and parse_nginx_time_format(parsed_line["time"]).timestamp() < (time.time()- float(args.last)*60):
            return False
        return True
 */
