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

    let content = fs::read_to_string(cli.input)
        .expect("Something went wrong reading the file");
    
    let lines = content.lines().collect::<Vec<&str>>();
    
    let parsed_lines = parse_lines(lines);

    let answer = match &cli.part {
        QuestionPart::Part1 => { part1(parsed_lines) }
        QuestionPart::Part2 => { part2(parsed_lines) }
    };

    println!("Answer: {}", answer);
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

fn parse_lines(lines: Vec<&str>) -> Vec<Instruction> {
    let mut out = vec![];
    
    for line in lines {
        let values: Vec<&str> = line.split(" ").collect();
        
        let direction = match &values[0][..] {
            "forward" => Direction::Forward,
            "up" => Direction::Up,
            "down" => Direction::Down,
            _ => continue
        };
        
        let value = match values[1].parse::<i32>() {
            Ok(num) => num,
            Err(_) => continue
        };
        
        out.push(Instruction { direction, value });
    }
    
    out
}

fn part1(instructions: Vec<Instruction>) -> i32{
    let (mut x, mut y) = (0, 0);
    
    for instruction in instructions {
        match instruction.direction {
            Direction::Forward => x += instruction.value,
            Direction::Up => y -= instruction.value,
            Direction::Down => y += instruction.value
        }
    }
    
    x * y
}

fn part2(instructions: Vec<Instruction>) -> i32{
    let (mut x, mut y, mut aim) = (0, 0, 0);

    for instruction in instructions {
        match instruction.direction {
            Direction::Forward => {
                x += instruction.value;
                y += instruction.value * aim;
            }
            Direction::Up => aim -= instruction.value,   // TODO: can't go below 0
            Direction::Down => aim += instruction.value
        }
    }

    x * y
}