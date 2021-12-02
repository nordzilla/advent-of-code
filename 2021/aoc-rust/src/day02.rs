use aoc_runner_derive::*;
use std::str::FromStr;

const UP: &str = "up ";
const DOWN: &str = "down ";
const FORWARD: &str = "forward ";

type Input = Vec<Instruction>;
type Output = i64;

#[derive(Copy, Clone)]
enum Instruction {
    Up(i64),
    Down(i64),
    Forward(i64),
}

impl FromStr for Instruction {
    type Err = <i64 as FromStr>::Err;
    fn from_str(value: &str) -> Result<Self, Self::Err> {
        value
            .trim_start_matches(UP)
            .parse()
            .map(Instruction::Up)
            .or_else(|_| {
                value
                    .trim_start_matches(DOWN)
                    .parse()
                    .map(Instruction::Down)
            })
            .or_else(|_| {
                value
                    .trim_start_matches(FORWARD)
                    .parse()
                    .map(Instruction::Forward)
            })
    }
}

#[derive(Copy, Clone, Default)]
struct Submarine {
    aim: i64,
    depth: i64,
    horizontal: i64,
}

impl Submarine {
    fn go(mut self, instruction: Instruction) -> Self {
        match instruction {
            Instruction::Up(n) => self.depth -= n,
            Instruction::Down(n) => self.depth += n,
            Instruction::Forward(n) => self.horizontal += n,
        }
        self
    }

    fn go_with_aim(mut self, instruction: Instruction) -> Self {
        match instruction {
            Instruction::Up(n) => self.aim -= n,
            Instruction::Down(n) => self.aim += n,
            Instruction::Forward(n) => {
                self.horizontal += n;
                self.depth += self.aim * n;
            }
        }
        self
    }
}

#[aoc_generator(day2)]
fn input_generator(raw_input: &str) -> Input {
    raw_input
        .lines()
        .map(|line| line.parse().unwrap())
        .collect()
}

fn gogo_submarine(input: &Input, gogo: impl Fn(Submarine, Instruction) -> Submarine) -> Output {
    let submarine = input
        .iter()
        .fold(Submarine::default(), |submarine, &instruction| {
            gogo(submarine, instruction)
        });
    submarine.horizontal * submarine.depth
}

#[aoc(day2, part1)]
fn solve_part1(input: &Input) -> Output {
    gogo_submarine(input, Submarine::go)
}

#[aoc(day2, part2)]
fn solve_part2(input: &Input) -> Output {
    gogo_submarine(input, Submarine::go_with_aim)
}
