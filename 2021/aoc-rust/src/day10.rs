use aoc_runner_derive::*;
use flow_control::return_if;
use itertools::Itertools;

type Input = Vec<Vec<u8>>;
type Output = usize;

#[aoc_generator(day10)]
fn input_generator(raw_input: &str) -> Input {
    raw_input
        .lines()
        .map(|line| line.bytes().collect())
        .collect()
}

fn is_opening(byte: u8) -> bool {
    matches!(byte, b'(' | b'[' | b'{' | b'<')
}

fn get_match(byte: u8) -> u8 {
    match byte {
        b'(' => b')',
        b'[' => b']',
        b'{' => b'}',
        b'<' => b'>',
        _ => panic!("Another one bytes the dust."),
    }
}

fn is_matching_pair(lhs: u8, rhs: Option<u8>) -> bool {
    rhs.map(|byte| lhs == get_match(byte)).unwrap_or(false)
}

fn point_value_part1(byte: u8) -> usize {
    match byte {
        b')' => 3,
        b']' => 57,
        b'}' => 1197,
        b'>' => 25137,
        _ => 0,
    }
}

fn point_value_part2(byte: u8) -> usize {
    match byte {
        b')' => 1,
        b']' => 2,
        b'}' => 3,
        b'>' => 4,
        _ => 0,
    }
}

fn next(input: &[u8]) -> (u8, &[u8]) {
    let (head, tail) = input.split_at(1);
    (head[0], tail)
}

fn parse(input: &[u8], mut stack: Vec<u8>) -> Result<Vec<u8>, u8> {
    return_if!(input.is_empty(), Ok(stack));
    let (byte, tail) = next(input);
    if is_opening(byte) {
        stack.push(byte);
        parse(tail, stack)
    } else if is_matching_pair(byte, stack.pop()) {
        parse(tail, stack)
    } else {
        Err(byte)
    }
}

fn eval(input: &Vec<u8>) -> Result<Vec<u8>, u8> {
    let stack = Vec::with_capacity(input.len());
    parse(input, stack)
}

#[aoc(day10, part1)]
fn solve_part1(input: &Input) -> Output {
    input
        .iter()
        .map(eval)
        .filter(Result::is_err)
        .map(Result::unwrap_err)
        .map(point_value_part1)
        .sum()
}

#[aoc(day10, part2)]
fn solve_part2(input: &Input) -> Output {
    let scores = input
        .iter()
        .filter_map(|line| eval(line).ok())
        .map(|stack| {
            stack
                .into_iter()
                .rev()
                .map(get_match)
                .fold(0, |value, byte| value * 5 + point_value_part2(byte))
        })
        .sorted()
        .collect::<Vec<_>>();
    scores[scores.len() / 2]
}
