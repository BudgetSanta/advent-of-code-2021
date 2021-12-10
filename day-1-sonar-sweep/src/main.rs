use clap::{Parser, Subcommand};
use std::fs;

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

type InputRow = i32;
type ParsedQuestionInput = Vec<InputRow>;

fn main() {
    let cli = Cli::parse();

    let contents = fs::read_to_string(cli.input).expect("Something went wrong reading the file");

    let input = parse_input(contents);

    let answer = match &cli.part {
        QuestionPart::Part1 => part1(input),
        QuestionPart::Part2 => part2(input),
    };

    println!("Answer: {}", answer);
}

fn parse_input(contents: String) -> ParsedQuestionInput {
    contents
        .lines()
        .map(|x| x.parse::<InputRow>().expect("parse error"))
        .collect()
}

fn part1(depths: ParsedQuestionInput) -> i32 {
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

fn part2(depths: ParsedQuestionInput) -> i32 {
    if depths.len() < 4 {
        return 0;
    }

    let mut current_depth_window = [depths[0], depths[1], depths[2]];
    let mut depth_increases = 0;

    for i in 1..depths.len() - 2 {
        let next_depth_window = [depths[i], depths[i + 1], depths[i + 2]];

        if next_depth_window.iter().sum::<i32>() > current_depth_window.iter().sum::<i32>() {
            depth_increases += 1;
        }

        current_depth_window = next_depth_window;
    }

    depth_increases
}
