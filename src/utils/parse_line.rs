#[path = "../structs/mod.rs"]
mod structs;
use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    static ref R: Regex = Regex::new("\"(.*?)\"").unwrap();
}
pub fn parse_line(line: &str) -> crate::structs::LineParseResult::LineParseResult {
    let matches: Vec<&str> = R.find_iter(line).map(|mat| mat.as_str()).collect();
    let mut fields = line.split(" ").collect::<Vec<_>>();
    return crate::structs::LineParseResult::LineParseResult {
        ip_address: fields[0],
        time: fields[3].replace("[", "") + " " + &fields[4].replace("]", ""),
        host: fields[5],
        referer: fields[11],
        request: matches[1],
        status: fields[9],
        body_bytes_sent: if !fields[10].to_string().parse::<i64>().is_err() {
            !fields[10].to_string().parse::<i64>().unwrap()
        } else {
            0
        },
        request_time: 0,
        user_agent: matches[3],
        full_text: line,
    };
}
