use aoc_runner_derive::*;

type Input = Vec<i64>;
type Output = usize;

// ========================================
// Original solution
// ========================================

#[aoc_generator(day1, part1, Original)]
pub fn input_generator_part1(raw_input: &str) -> Input {
    raw_input
        .lines()
        .map(|line| line.parse().unwrap())
        .collect()
}

#[aoc_generator(day1, part2, Original)]
pub fn input_generator_part2(raw_input: &str) -> Input {
    input_generator_part1(raw_input)
        .windows(3)
        .map(|window| window.iter().sum())
        .collect()
}

#[aoc(day1, part1, Original)]
pub fn solve_part1(input: &Input) -> Output {
    input
        .iter()
        .zip(&input[1..])
        .filter(|(prev, next)| next > prev)
        .count()
}

#[aoc(day1, part2, Original)]
pub fn solve_part2(input: &Input) -> Output {
    solve_part1(input)
}

// ========================================
// Clever solution
// ========================================
//
// There is no need to sum the windows, because the two windows in question will
// share the same middle numbers. We need to compare only the first number to the
// last number.

#[aoc_generator(day1, Clever)]
pub fn input_generator_clever(raw_input: &str) -> Input {
    input_generator_part1(raw_input)
}

fn solve(input: &Input, window_size: usize) -> Output {
    input
        .windows(window_size)
        .filter(|window| window.last() > window.first())
        .count()
}

#[aoc(day1, part1, Clever)]
pub fn solve_part1_clever(input: &Input) -> Output {
    solve(input, 2)
}

#[aoc(day1, part2, Clever)]
pub fn solve_part2_clever(input: &Input) -> Output {
    solve(input, 4)
}
