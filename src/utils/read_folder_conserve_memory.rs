use crate::utils::keep_line::keep_line;
use crate::utils::parse_line::parse_line;
use flate2::read::GzDecoder;
use std::collections::HashMap;
use std::fs;
use std::fs::metadata;
use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn read_folder_conserve_memory(file_path: String, isUnique: Option<bool>) {
    let mut paths: Vec<_> = fs::read_dir(file_path).unwrap().collect();
    paths.sort_by_key(|x| {
        metadata(x.as_ref().unwrap().path().to_str().unwrap())
            .unwrap()
            .modified()
            .unwrap()
    });
    let mut occurrences: HashMap<String, bool> = HashMap::new();
    for path in paths {
        let p: String = path.unwrap().path().to_str().unwrap().to_string();
        if p.contains(".gz") {
            let file = File::open(p).expect("Ooops.");
            let reader = BufReader::new(GzDecoder::new(file));
            for r in reader.lines() {
                match r {
                    Ok(line) => {
                        let ip: String =
                            line.clone().split(" ").collect::<Vec<&str>>()[0].to_string();

                        if line.chars().filter(|c| *c == '"').count() > 4
                            && line.len() > 20
                            && keep_line(&parse_line(&line), true)
                        {
                            if isUnique.is_some() && isUnique.unwrap() == true {
                                if !occurrences.contains_key(&ip) {
                                    println!("{}", line.clone() + "\n");
                                    occurrences.insert(ip, true);
                                }
                            } else {
                                println!("{}", line.clone() + "\n");
                            }
                        }
                    }
                    Err(error) => {
                        eprintln!("Error reading line: {}", error);
                    }
                }
            }
        } else {
            let file = File::open(p).expect("Ooops.");
            let reader = BufReader::new(file).lines();
            for line in reader.flatten() {
                let ip: String = line.clone().split(" ").collect::<Vec<&str>>()[0].to_string();
                if keep_line(&parse_line(&line), true) {
                    if isUnique.is_some() && isUnique.unwrap() == true {
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
    }
}
