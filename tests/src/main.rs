use async_process::Command;
use serde_derive::Deserialize;
use std::collections::HashMap;
use std::fs;
extern crate tokio;
use colored::Colorize;
use toml::from_str;
extern crate md5;

#[derive(Deserialize, Debug)]
struct TestCase {
    name: String,
    run_type: String,
    cmd: String,
    expect: String,
}

#[tokio::main]
async fn main() {
    let contents =
        fs::read_to_string("./test_cases.toml").expect("Should have been able to read the file");
    let items_table: HashMap<String, Vec<TestCase>> = from_str(contents.clone().as_str()).unwrap();
    let TestCases: &[TestCase] = &items_table["test_cases"];
    let mut TestIndex = 1;
    for case in TestCases {
        let cmd_parts: Vec<_> = case.cmd.split(" ").collect::<Vec<_>>();
        let output = Command::new(cmd_parts[0])
            .args(&cmd_parts[1..])
            .output()
            .await
            .expect("Failed to execute command");

        if (case.run_type == "output"
            && String::from_utf8_lossy(&output.stdout).contains(&case.expect))
            || (case.run_type == "checksum"
                && case.expect == format!("{:x}", md5::compute(&output.stdout)))
            || (case.run_type == "lines_count"
                && case.expect
                    == String::from_utf8_lossy(&output.stdout)
                        .split("\n")
                        .collect::<Vec<_>>()
                        .len()
                        .to_string())
        {
            println!(
                "Test #{i} - {n} - {r}",
                i = TestIndex,
                n = case.name.bold(),
                r = "Passed! ✓".green().bold()
            );
        } else {
            println!(
                "Test #{i} - {n} - {r}",
                i = TestIndex,
                n = case.name.bold(),
                r = "Failed! 𝕏".red().bold()
            );
            println!("{}", String::from_utf8_lossy(&output.stdout));
            println!("{}", String::from_utf8_lossy(&output.stderr));
            println!(
                "{}",
                String::from_utf8_lossy(&output.stdout)
                    .split("\n")
                    .collect::<Vec<_>>()
                    .len()
                    .to_string()
            );

            return;
        }
        TestIndex += 1;
    }
}
