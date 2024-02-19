use rand::Rng;
use std::collections::HashMap;
use std::env;
use std::fs;

fn main() {
    let args: Vec<_> = env::args().collect();
    let contents =
        fs::read_to_string(args[1].clone()).expect("Should have been able to read the file");
    let binding = contents.clone();
    let lines: Vec<_> = binding.split("\n").collect();
    let mut ip_conversion_table: HashMap<String, String> = HashMap::new();
    let mut ans: String = "".to_string();
    for line in lines {
        let ip = line.split(" ").collect::<Vec<_>>()[0];
        if ip != "-" {
            if !ip_conversion_table.contains_key(ip) {
                let r_ip = rand::thread_rng().gen_range(0..1000000);
                ip_conversion_table.insert(ip.to_string(), r_ip.to_string());
            }
            let mut new_line: String = line.clone().to_string();
            new_line = str::replace(&new_line, ip, ip_conversion_table.get(ip).expect("WOOP"));
            new_line.push_str("\n");
            if new_line.len() > 20 {
                ans.push_str(&new_line.as_str());
            }
        }
    }
    fs::write(args[1].clone(), ans);
}
