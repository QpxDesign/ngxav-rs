use crate::sort_by_date::sort_by_date;
use clap::Parser;
use rayon::prelude::*;
use structs::LineParseResult::LineParseResult;
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
fn main() {
    let args: crate::structs::Args::ArgParser = ArgParser::parse();
    if args.thread_count.is_some() {
        rayon::ThreadPoolBuilder::new()
            .num_threads(args.thread_count.unwrap().try_into().expect("WOMP WOMP"))
            .build_global()
            .unwrap();
    }
    let lines: Vec<String> = lines_from_file(args.file.clone()).expect("should read");
    let parsed_lines: Vec<crate::structs::LineParseResult::LineParseResult> =
        lines.par_iter().map(|l: &String| parse_line(l)).collect();
    let mut stdout = io::stdout().lock();
    let mut kel: Vec<LineParseResult> = parsed_lines
        .into_par_iter()
        .filter(|p: &crate::structs::LineParseResult::LineParseResult| {
            utils::keep_line::keep_line(p) == true
        })
        .collect();
    kel = sort_by_date(kel, &args.last, &args.start_date, &args.end_date);
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
        for line in kel {
            stdout.write_all(line.full_text.as_bytes());
            stdout.write_all(b"\n\n");
        }
    }
}
