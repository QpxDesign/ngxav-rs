use clap::Parser;
use std::env;
use std::fs;
use std::sync::mpsc::channel;
use std::sync::{Arc, Mutex};
use std::thread;
use tokio; // 1.14.0

#[path = "./structs/mod.rs"]
mod structs;
mod utils;
use crate::structs::Args::ArgParser;
use crate::utils::generate_analytics::generate_analytical_output;
use kdam::tqdm;
use std::{
    fs::File,
    io::{self, BufRead, BufReader},
    path::Path,
};
use threadpool::ThreadPool;

fn lines_from_file(filename: impl AsRef<Path>) -> io::Result<Vec<String>> {
    BufReader::new(File::open(filename)?).lines().collect()
}
#[tokio::main]
async fn main() {
    let args: crate::structs::Args::ArgParser = ArgParser::parse();
    let mut lines: Vec<String> = lines_from_file(args.file).expect("should read");
    println!("{}", lines.len());
    let mut kept_lines: Arc<Mutex<Vec<String>>> = Arc::new(Mutex::new(vec![]));
    let pool: ThreadPool = ThreadPool::new(16);
    for line in lines.clone() {
        let kept_lines = Arc::clone(&kept_lines);
        let new_line = line.clone();
        pool.execute(move || {
            if utils::keep_line::keep_line(line) {
                let mut kept_lines = kept_lines.lock().unwrap();
                kept_lines.push(new_line);
            }
        })
    }
    let final_lines: std::sync::MutexGuard<'_, Vec<String>> = kept_lines.lock().unwrap();
    if !args.analytics.is_none() && args.analytics == Some(true) {
        println!("{}", "worked");
    } else {
        println!("{}", final_lines.len());
    }
}
