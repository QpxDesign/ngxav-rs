use crate::structs::Args::ArgParser;
use crate::utils::parse_input_time::parse_input_time;
use crate::utils::parse_nginx_time_format::parse_nginx_time_format;
use crate::utils::parse_user_agent::parse_user_agent;
use regex::Regex;
#[path = "../structs/mod.rs"]
mod structs;
use crate::structs::LineParseResult::LineParseResult;
use crate::structs::UserAgentParseResult;
use clap::Parser;
use lazy_static::lazy_static;

use std::time::{SystemTime, UNIX_EPOCH};
lazy_static! {
    static ref ARGS: crate::structs::Args::ArgParser = ArgParser::parse();
    static ref SEARCH_REGEX: Regex = Regex::new(&ARGS.search.clone().unwrap().to_string()).unwrap();
}
pub fn keep_line(parsed_line: &LineParseResult, cm: bool) -> bool {
    if !ARGS.search.is_none() {
        if !ARGS.plain_text.is_none() && ARGS.plain_text == Some(true) {
            if !parsed_line
                .full_text
                .contains(&ARGS.search.as_ref().unwrap().to_string())
            {
                return false;
            }
        } else {
            if !SEARCH_REGEX.is_match(&parsed_line.full_text) {
                return false;
            }
        }
    }
    if !ARGS.host.is_none()
        && parsed_line.host.replace('"', "").as_str() != ARGS.host.as_ref().unwrap()
    {
        return false;
    }
    if !ARGS.request.is_none()
        && !parsed_line
            .request
            .contains(&ARGS.request.as_ref().unwrap().to_string())
    {
        return false;
    }
    if !ARGS.http_status.is_none() && parsed_line.status != ARGS.http_status.as_ref().unwrap() {
        return false;
    }
    if !ARGS.referer.is_none()
        && parsed_line.referer.replace('"', "").as_str() != ARGS.referer.as_ref().unwrap()
    {
        return false;
    }

    if cm == false {
        return true;
    }
    let tz = parsed_line.time.split(" ").collect::<Vec<_>>()[1];
    if !ARGS.start_date.is_none() && ARGS.end_date.is_none() {
        if parse_nginx_time_format(&parsed_line.time)
            < parse_input_time(&ARGS.start_date.as_ref().unwrap(), tz.to_string())
        {
            return false;
        }
    }
    if !ARGS.end_date.is_none() && ARGS.start_date.is_none() {
        if parse_nginx_time_format(&parsed_line.time)
            > parse_input_time(&ARGS.end_date.as_ref().unwrap(), tz.to_string())
        {
            return false;
        }
    }
    if !ARGS.start_date.is_none()
        && !ARGS.end_date.is_none()
        && (parse_nginx_time_format(&parsed_line.time)
            > parse_input_time(&ARGS.end_date.as_ref().unwrap(), tz.to_string())
            || parse_nginx_time_format(&parsed_line.time)
                < parse_input_time(&ARGS.start_date.as_ref().unwrap(), tz.to_string()))
    {
        return false;
    }
    let start = SystemTime::now();
    let since_the_epoch = start
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards");
    let mut epoch_seconds: u64 = since_the_epoch.as_secs();

    if !ARGS.last.is_none() {
        epoch_seconds = epoch_seconds - 60 * ARGS.last.unwrap();
        if parse_nginx_time_format(&parsed_line.time).timestamp() < (epoch_seconds as i64) {
            return false;
        }
    }

    let parsed_ua = parse_user_agent(parsed_line.user_agent);
    if ARGS.browser.is_some()
        && parsed_ua.browser.to_lowercase() != ARGS.browser.as_ref().expect("WOOP").to_lowercase()
    {
        return false;
    }
    if ARGS.os.is_some()
        && parsed_ua.operating_system.to_lowercase()
            != ARGS.os.to_owned().expect("WOOP").to_lowercase()
    {
        return false;
    }
    if ARGS.device_category.is_some()
        && ARGS.device_category.as_ref().unwrap().as_str() != parsed_ua.category
    {
        return false;
    }
    if ARGS.bot.is_some() && ARGS.bot.unwrap() == false && parsed_ua.isBot == true {
        return false;
    }
    if ARGS.bot.is_some() && ARGS.bot.unwrap() == true && parsed_ua.isBot == false {
        return false;
    }

    return true;
}
