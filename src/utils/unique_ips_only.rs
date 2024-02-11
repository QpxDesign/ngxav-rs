use std::collections::HashMap;

pub fn unique_ips_only(lines: Vec<std::string::String>) -> Vec<std::string::String> {
    let mut occurrences: HashMap<String, String> = HashMap::new();

    for line in lines {
        let unique_key = line.split(" ").collect::<Vec<&str>>()[0];
        if occurrences.contains_key(unique_key) == false {
            occurrences.insert(unique_key.to_string(), line.to_string());
        }
    }
    let a = occurrences.values().cloned().collect();
    return a;
}
