use crate::sort_by_date::sort_by_date;
use clap::Parser;
use rayon::prelude::*;
use std::collections::HashMap;
use utils::keep_line::keep_line;
use utils::parse_nginx_time_format::parse_nginx_time_format;
use utils::sort_by_date;

#[path = "./structs/mod.rs"]
mod structs;
mod utils;
use crate::structs::Args::ArgParser;
use crate::utils::parse_line::parse_line;

use std::{
    fs::File,
    io::{self, BufRead, BufReader, Write},
    path::Path,
};

fn lines_from_file(filename: impl AsRef<Path>) -> io::Result<Vec<String>> {
    BufReader::new(File::open(filename)?).lines().collect()
}
fn read_line_by_line(filename: impl AsRef<Path>) -> io::Result<io::Lines<io::BufReader<File>>> {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn main() {
    let args: crate::structs::Args::ArgParser = ArgParser::parse();
    if args.thread_count.is_some() {
        rayon::ThreadPoolBuilder::new()
            .num_threads(args.thread_count.unwrap().try_into().expect("WOMP WOMP"))
            .build_global()
            .unwrap();
    }
    if !args.conserve_memory.is_none() && args.conserve_memory.unwrap() == true {
        if let Ok(lines) = read_line_by_line(args.file) {
            let mut occurrences: HashMap<String, bool> = HashMap::new();
            for line in lines.flatten() {
                let ip: String = line.clone().split(" ").collect::<Vec<&str>>()[0].to_string();
                if keep_line(&parse_line(&line), true) {
                    if args.unique.is_some() && args.unique.unwrap() == true {
                        if !occurrences.contains_key(&ip) {
                            println!("{}", line.clone() + "\n");
                            occurrences.insert(ip, true);
                        }
                    } else {
                        println!("{}", line.clone() + "\n");
                    }
                }
            }
        }
        return;
    }
    let lines = lines_from_file(args.file).expect("should read");
    let mut kel: Vec<crate::structs::LineParseResult::LineParseResult> =
        lines.par_iter().map(|l: &String| parse_line(l)).collect();
    let mut stdout = io::stdout().lock();
    kel = kel
        .into_par_iter()
        .filter(|p: &crate::structs::LineParseResult::LineParseResult| {
            utils::keep_line::keep_line(p, false) == true
        })
        .collect();
    kel = sort_by_date(&kel, &args.last, &args.start_date, &args.end_date);
    if !args.unique.is_none() && args.unique == Some(true) {
        kel = utils::unique_ips_only::unique_ips_only(kel);
    }
    if !args.analytics.is_none() && args.analytics == Some(true) {
        utils::generate_analytics::generate_analytical_output(kel);
    } else if !args.session_analytics.is_none() && args.session_analytics == Some(true) {
        utils::session_analytics::session_analytics(kel);
    } else if !args.large.is_none() {
        utils::sort_by_body_size::sort_by_body_size(kel, args.large.unwrap());
    } else if !args.ip_ses.is_none() {
        utils::sessions_from_ip::sessions_from_ip(kel, args.ip_ses.unwrap());
    } else if !args.session_unqiue.is_none() && args.session_unqiue == Some(true) {
        utils::session_unique::session_unique(kel);
    } else {
        kel.par_sort_by_key(|a| parse_nginx_time_format(&a.time).timestamp());
        for line in kel {
            stdout.write_all(line.full_text.as_bytes());
            stdout.write_all(b"\n\n");
        }
    }
}
