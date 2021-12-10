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

enum Direction {
    Forward,
    Up,
    Down,
}

struct Instruction {
    direction: Direction,
    value: i32,
}

type InputRow = Instruction;
type ParsedQuestionInput = Vec<InputRow>;

fn main() {
    let cli = Cli::parse();

    let content = fs::read_to_string(cli.input).expect("Something went wrong reading the file");

    let input = parse_input(content);

    let answer = match &cli.part {
        QuestionPart::Part1 => part1(input),
        QuestionPart::Part2 => part2(input),
    };

    println!("Answer: {}", answer);
}

fn parse_input(contents: String) -> ParsedQuestionInput {
    let split_lines = contents.lines().collect::<Vec<&str>>();
    let mut out = vec![];

    for line in split_lines {
        let values: Vec<&str> = line.split(" ").collect();

        let direction = match &values[0][..] {
            "forward" => Direction::Forward,
            "up" => Direction::Up,
            "down" => Direction::Down,
            _ => continue,
        };

        let value = match values[1].parse::<i32>() {
            Ok(num) => num,
            Err(_) => continue,
        };

        out.push(Instruction { direction, value });
    }

    out
}

fn part1(instructions: ParsedQuestionInput) -> i32 {
    let (mut x, mut y) = (0, 0);

    for instruction in instructions {
        match instruction.direction {
            Direction::Forward => x += instruction.value,
            Direction::Up => y -= instruction.value,
            Direction::Down => y += instruction.value,
        }
    }

    x * y
}

fn part2(instructions: ParsedQuestionInput) -> i32 {
    let (mut x, mut y, mut aim) = (0, 0, 0);

    for instruction in instructions {
        match instruction.direction {
            Direction::Forward => {
                x += instruction.value;
                y += instruction.value * aim;
            }
            Direction::Up => aim -= instruction.value, // TODO: can't go below 0
            Direction::Down => aim += instruction.value,
        }
    }

    x * y
}
