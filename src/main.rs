mod structs;
use clap::Parser;

fn main() {
    let args = structs::ArgParser::parse();
    println!("Hello, world2!");
    println!("{}", args.file)
}
