use clap::Parser;
use rayon::prelude::*;

#[path = "./structs/mod.rs"]
mod structs;
mod utils;
use crate::structs::Args::ArgParser;

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
    let lines: Vec<String> = lines_from_file(args.file).expect("should read");

    let kel: Vec<_> = lines
        .into_par_iter()
        .filter(|p| utils::keep_line::keep_line(p.to_string()) == true)
        .collect();

    if !args.analytics.is_none() && args.analytics == Some(true) {
        utils::generate_analytics::generate_analytical_output(kel.clone());
    } else {
        for line in kel.clone() {
            println!("{}", line);
        }
    }
}
