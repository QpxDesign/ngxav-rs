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
    io::{self, BufRead, BufReader},
    path::Path,
};

fn lines_from_file(filename: impl AsRef<Path>) -> io::Result<Vec<String>> {
    BufReader::new(File::open(filename)?).lines().collect()
}
fn main() {
    let args: crate::structs::Args::ArgParser = ArgParser::parse();
    let lines: Vec<String> = lines_from_file(args.file.clone()).expect("should read");
    let mut parsed_lines: Vec<crate::structs::LineParseResult::LineParseResult> =
        lines.par_iter().map(|l: &String| parse_line(l)).collect();

    let mut kel: Vec<LineParseResult> = parsed_lines
        .into_par_iter()
        .filter(|p: &crate::structs::LineParseResult::LineParseResult| {
            utils::keep_line::keep_line(p.clone(), &args) == true
        })
        .collect();
    if !args.unique.is_none() && args.unique == Some(true) {
        kel = utils::unique_ips_only::unique_ips_only(kel);
    }
    if !args.analytics.is_none() && args.analytics == Some(true) {
        utils::generate_analytics::generate_analytical_output(kel);
    } else if !args.session_analytics.is_none() && args.session_analytics == Some(true) {
        //  utils::session_analytics::session_analytics(kel);
    } else if !args.large.is_none() {
        utils::sort_by_body_size::sort_by_body_size(kel, args.large.unwrap());
    } else {
        for line in sort_by_date(kel) {
            println!("{}", line.full_text + "\n");
        }
    }
}
