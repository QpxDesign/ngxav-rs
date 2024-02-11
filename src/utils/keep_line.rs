use clap::Parser;
use regex::Regex;

use crate::structs::Args::ArgParser;
use crate::utils::parse_nginx_time_format::parse_nginx_time_format;

use std::time::{SystemTime, UNIX_EPOCH};

pub fn keep_line(line: String) -> bool {
    let args: crate::structs::Args::ArgParser = ArgParser::parse();
    let parsed_line = crate::utils::parse_line::parse_line(&line);
    if !args.search.is_none() {
        let re = Regex::new(&args.search.clone().unwrap().to_string()).unwrap();
        if !re.is_match(&line) {
            println!("EXIT1");
            return false;
        }
    }
    if !args.start_date.is_none() && args.end_date.is_none() {
        if parse_nginx_time_format(&parsed_line.time)
            < parse_nginx_time_format(&args.start_date.clone().unwrap())
        {
            println!("EXIT2");
            return false;
        }
    }
    if !args.end_date.is_none() && args.start_date.is_none() {
        if parse_nginx_time_format(&parsed_line.time)
            > parse_nginx_time_format(&args.end_date.clone().unwrap())
        {
            println!("EXIT3");
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
        println!("EXIT4");
        return false;
    }
    if !args.host.is_none() && parsed_line.host != args.host.unwrap() {
        println!("EXIT5");
        return false;
    }
    if !args.request.is_none() && !parsed_line.request.contains(&args.request.unwrap()) {
        println!("EXIT6");
        return false;
    }
    if !args.http_status.is_none() && parsed_line.status != args.http_status.unwrap() {
        println!("EXIT7");
        return false;
    }
    if !args.referer.is_none() && parsed_line.referer != args.referer.unwrap() {
        println!("EXIT8");
        return false;
    }
    let start = SystemTime::now();
    let since_the_epoch = start
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards");
    let epoch_seconds = since_the_epoch.as_millis() * 1000;
    if !args.last.is_none() {
        if parse_nginx_time_format(&parsed_line.time).timestamp() < epoch_seconds as i64 {
            println!("EXIT9");
            return false;
        }
    }
    return true;
}
