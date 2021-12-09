use std::fs;
use clap::{Parser, Subcommand};

#[derive(Subcommand)]
enum QuestionPart {
    Part1,
    Part2,
}

#[derive(Parser)]
struct Cli {
    #[clap(short, long)]
    input: String,

    #[clap(subcommand)]
    part: QuestionPart,
}

fn main() {
    let cli = Cli::parse();

    let contents = fs::read_to_string(cli.input)
        .expect("Something went wrong reading the file");

    let lines = contents.lines()
        .map(|x| x.parse::<i32>().expect("parse error"))
        .collect::<Vec<i32>>();

    let answer = match &cli.part {
        QuestionPart::Part1 => { count_depth_increases(&lines[..]) }
        QuestionPart::Part2 => { count_depth_sum_increases(&lines[..]) }
    };

    println!("Answer: {}", answer);
}

fn count_depth_increases(depths: &[i32]) -> i32 {
    let mut current_depth = depths[0];
    let mut depth_increases = 0;

    for next_depth in &depths[1..] {
        if next_depth > &current_depth {
            depth_increases += 1;
        }

        current_depth = *next_depth;
    }

    depth_increases
}

fn count_depth_sum_increases(depths: &[i32]) -> i32 {
    if depths.len() < 4 { return 0; }
    
    let mut current_depth_window = [depths[0], depths[1], depths[2]];
    let mut depth_increases = 0;
    
    for i in 1..depths.len()-2 {
        let next_depth_window = [depths[i], depths[i+1], depths[i+2]];
        
        if next_depth_window.iter().sum::<i32>() > current_depth_window.iter().sum::<i32>() {
            depth_increases += 1;
        }
        
        current_depth_window = next_depth_window;
    }

    depth_increases
}