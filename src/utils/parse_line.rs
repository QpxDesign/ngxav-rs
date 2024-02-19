#[path = "../structs/mod.rs"]
mod structs;
use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    static ref R: Regex = Regex::new("\"(.*?)\"").unwrap();
}
pub fn parse_line(line: &str) -> crate::structs::LineParseResult::LineParseResult {
    let matches: Vec<&str> = R.find_iter(line).map(|mat| mat.as_str()).collect();
    let fields = line.split(" ").collect::<Vec<_>>();

    return crate::structs::LineParseResult::LineParseResult {
        ip_address: fields[0].to_string(),
        time: fields[3].replace("[", "") + " " + &fields[4].replace("]", ""),
        host: fields[5].replace('"', ""),
        referer: fields[11].replace('"', ""),
        request: matches[1].to_string(),
        status: fields[9].to_string(),
        body_bytes_sent: if !fields[10].to_string().parse::<i64>().is_err() {
            !fields[10].to_string().parse::<i64>().unwrap()
        } else {
            0
        },
        request_time: "0".to_string(),
        user_agent: matches[3].to_string(),
        full_text: line.to_string(),
    };
}
