use clap::Parser;
use std::sync::mpsc::channel;

use std::sync::{Arc, Mutex};
use std::thread;
use tokio; // 1.14.0

#[path = "./structs/mod.rs"]
mod structs;
mod utils;
use crate::structs::Args::ArgParser;
use crate::utils::generate_analytics::generate_analytical_output;

use std::{
    fs::File,
    io::{self, BufRead, BufReader},
    path::Path,
};

fn lines_from_file(filename: impl AsRef<Path>) -> io::Result<Vec<String>> {
    BufReader::new(File::open(filename)?).lines().collect()
}
#[tokio::main]
async fn main() {
    let args: crate::structs::Args::ArgParser = ArgParser::parse();
    let mut lines: Vec<String> = lines_from_file(args.file).expect("should read");
    println!("{}", lines.len());
    let (sender, receiver) = channel();
    thread::spawn(move || {
        sender.send(gen_keep_lines(lines)).unwrap();
    });

    let final_lines = receiver.recv().unwrap();
    if !args.analytics.is_none() && args.analytics == Some(true) {
        utils::generate_analytics::generate_analytical_output(
            final_lines.await.lock().unwrap().to_vec(),
        );
    } else {
        println!("{}", final_lines.await.lock().unwrap().len());
    }
}

async fn gen_keep_lines(lines: Vec<String>) -> Arc<Mutex<Vec<String>>> {
    let mut kept_lines: Arc<Mutex<Vec<String>>> = Arc::new(Mutex::new(vec![]));
    for line in lines.clone().into_iter() {
        let kept_lines = Arc::clone(&kept_lines);
        let new_line = line.clone();
        if utils::keep_line::keep_line(line) {
            let mut kept_lines = kept_lines.lock().unwrap();
            kept_lines.push(new_line);
        } else {
            println!("FAIL");
        }
    }
    return kept_lines;
}
