use clap::Parser;
use std::env;
use std::fs;
use std::thread;

#[path = "./structs/mod.rs"]
mod structs;
mod utils;
use crate::structs::Args::ArgParser;

fn main() {
    let args: crate::structs::Args::ArgParser = ArgParser::parse();
    thread::spawn(|| {
        let contents =
            fs::read_to_string(args.file).expect("Should have been able to read the file");

        for line in contents.split("\n") {
            if utils::keep_line::keep_line(line.to_string()) {
                println!("{}", line)
            }
        }
    });
}
