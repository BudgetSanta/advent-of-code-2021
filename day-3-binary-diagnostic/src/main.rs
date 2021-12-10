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

type InputRow<'a> = &'a str;
type ParsedQuestionInput<'a> = Vec<InputRow<'a>>;

fn main() {
    let cli = Cli::parse();

    let contents = fs::read_to_string(cli.input).expect("Something went wrong reading the file");

    let input = parse_input(&contents);

    let answer = match &cli.part {
        QuestionPart::Part1 => part1(input),
        QuestionPart::Part2 => part2(input),
    };

    println!("Answer: {}", answer);
}

fn parse_input<'a>(contents: &'a String) -> ParsedQuestionInput<'a> {
    contents.lines().collect()
}

fn part1(readings: ParsedQuestionInput) -> i32 {
    let mut counts = vec![(0, 0); readings[0].len()];

    let mut gamma = String::from("");
    let mut epsilon = String::from("");

    for reading in readings {
        for (i, digit) in reading.chars().enumerate() {
            match digit {
                '0' => counts[i].0 += 1,
                '1' => counts[i].1 += 1,
                _ => continue,
            }
        }
    }

    for count in counts {
        let greater = if count.0 > count.1 { '0' } else { '1' };
        let lesser = if greater == '0' { '1' } else { '0' };

        gamma.push(greater);
        epsilon.push(lesser);
    }

    binary_to_dec(gamma) * binary_to_dec(epsilon)
}

fn part2(readings: ParsedQuestionInput) -> i32 {
    0
}

fn binary_to_dec(binary: String) -> i32 {
    i32::from_str_radix(&binary, 2).expect("error parsing binary string into i32")
}
