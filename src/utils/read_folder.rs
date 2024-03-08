extern crate flate2;
use flate2::read::GzDecoder;
use std::fs;
use std::io;
use std::io::prelude::*;

pub fn read_folder(file_path: String) -> Vec<String> {
    let paths = fs::read_dir(file_path).unwrap();
    let mut lines = Vec::new();
    for path in paths {
        let a = std::io::BufReader::new(
            fs::File::open(path.as_ref().unwrap().path().to_str().unwrap()).unwrap(),
        );
        if path.unwrap().path().to_str().unwrap().contains(".gz") {
            let d = GzDecoder::new(a);
            for line in io::BufReader::new(d).lines() {
                lines.push(line.unwrap().to_string());
            }
        } else {
            for line in io::BufReader::new(a).lines() {
                lines.push(line.unwrap().to_string());
            }
        }
    }
    return lines;
}
