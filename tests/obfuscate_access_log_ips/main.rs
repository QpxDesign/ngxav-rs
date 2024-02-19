use rand::Rng;
use std::collections::HashMap;
use std::env;
use std::fs; // 0.8.5 // 0.8.5 // 0.8.5 // 0.8.5

pub fn main() {
    let args: Vec<_> = env::args().collect();
    let contents =
        fs::read_to_string(args[1].clone()).expect("Should have been able to read the file");
    let binding = contents.clone();
    let lines: Vec<_> = binding.split("\n").collect();
    let ip_conversion_table: HashMap<String, String> = HashMap::new();
    for line in lines {
        let ip = line.split("").collect::<Vec<_>>()[0];
        if !ip_conversion_table.contains_key(ip) {
            let num = rand::thread_rng().gen_range(0..100);

            //ip_conversion_table.insert(ip, r_ip.to_string());
        }
        let new_line = line.clone();
    }
}
