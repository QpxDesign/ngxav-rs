#[path = "../structs/mod.rs"]
mod structs;
use crate::structs::LineParseResult;
use regex::Regex;

pub fn parse_line(line: &str) -> structs::LineParseResult::LineParseResultStruct {
    let fields = line.split(" ").collect::<Vec<_>>();
    let re = Regex::new(r####"(.*?)"####).unwrap();
    return structs::LineParseResult::LineParseResultStruct {
        ip_address: fields[0].to_string(),
        time: fields[3].replace("[", ""),
        host: fields[5].replace('"', ""),
        referer: fields[11].replace('"', ""),
        request: fields[6].to_owned() + " " + fields[7],
        status: fields[9].to_string(),
        body_bytes_sent: fields[10].to_string(),
        request_time: "0".to_string(),
    };
}
