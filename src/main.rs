use crate::sort_by_date::sort_by_date;
use atty::Stream;
use clap::Parser;
use rayon::prelude::*;
use std::collections::HashMap;
use std::iter::FromIterator;
use utils::keep_line::keep_line;
use utils::parse_nginx_time_format::parse_nginx_time_format;
use utils::read_folder::read_folder;
use utils::sort_by_date;

#[path = "./structs/mod.rs"]
mod structs;
mod utils;
use crate::structs::Args::ArgParser;
use crate::utils::parse_line::parse_line;

use std::{
    fs::metadata,
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

fn load_stdin() -> Vec<String> {
    let stdin = io::stdin();
    return stdin
        .lock()
        .lines()
        .map(|line| line.expect("Failed to read line"))
        .collect();
}
fn main() {
    let args: crate::structs::Args::ArgParser = ArgParser::parse();
    if args.thread_count.is_some() {
        rayon::ThreadPoolBuilder::new()
            .num_threads(args.thread_count.unwrap().try_into().expect("WOMP WOMP"))
            .build_global()
            .unwrap();
    }
    if args.file.is_some()
        && !args.conserve_memory.is_none()
        && args.conserve_memory.unwrap() == true
        && metadata(args.file.clone().unwrap()).unwrap().is_file()
    {
        if let Ok(lines) = read_line_by_line(args.file.unwrap()) {
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
    let mut lines = Vec::new();
    let stdin = load_stdin();
    if args.file.is_none() && stdin.len() == 0 {
        panic!("error must either pipe-in log data or provide file with -f")
    }
    if args.file.is_some() && metadata(args.file.clone().unwrap()).unwrap().is_dir() {
        if args.conserve_memory.is_some() && args.conserve_memory.unwrap() == true {
            utils::read_folder_conserve_memory::read_folder_conserve_memory(
                args.file.unwrap(),
                args.unique,
            );
            return;
        }
    } else if args.file.is_none() && stdin.len() > 0 {
        lines = stdin.clone();
    } else {
        lines = lines_from_file(args.file.unwrap()).expect("should read");
    }
    let range = sort_by_date(&lines, &args.last, &args.start_date, &args.end_date);
    let mut kel: Vec<crate::structs::LineParseResult::LineParseResult> = lines[range.0..range.1]
        .par_iter()
        .map(|l: &String| parse_line(l))
        .collect();
    let mut stdout = io::stdout().lock();
    kel = kel
        .into_par_iter()
        .filter(|p: &crate::structs::LineParseResult::LineParseResult| {
            utils::keep_line::keep_line(p, false) == true
        })
        .collect();
    if (!args.unique.is_none() && args.unique == Some(true)) {
        kel = utils::unique_ips_only::unique_ips_only(kel, args.unique_by.clone());
    }
    if !args.analytics.is_none() && args.analytics == Some(true) {
        utils::generate_analytics::generate_analytical_output(kel);
    } else if !args.session_analytics.is_none() && args.session_analytics == Some(true) {
        utils::session_analytics::session_analytics(kel, args.unique_by.clone());
    } else if !args.large.is_none() {
        utils::sort_by_body_size::sort_by_body_size(kel, args.large.unwrap());
    } else if !args.ip_ses.is_none() {
        utils::sessions_from_ip::sessions_from_ip(kel, args.ip_ses, args.unique_by);
    } else if !args.session_unqiue.is_none() && args.session_unqiue == Some(true) {
        utils::session_unique::session_unique(kel, args.unique_by);
    } else {
        kel.par_sort_by_key(|a| parse_nginx_time_format(&a.time).timestamp());
        for line in kel {
            stdout.write_all(line.full_text.as_bytes());
            stdout.write_all(b"\n\n");
        }
    }
}
