use std::fs;
use clap::Parser;

#[derive(Parser)]
#[clap()]
struct Cli {
    #[clap()]
    input: String,
}

fn main() {
    let cli = Cli::parse();

    let contents = fs::read_to_string(cli.input)
        .expect("Something went wrong reading the file");

    for content in contents.lines() {
        println!(".{}", content)
    }
}