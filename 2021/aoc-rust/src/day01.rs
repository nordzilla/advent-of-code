use aoc_runner_derive::*;

type InputPart1 = Vec<i64>;
type InputPart2 = InputPart1;
type OutputPart1 = usize;
type OutputPart2 = OutputPart1;

#[aoc_generator(day1, part1)]
pub fn input_generator_part1(raw_input: &str) -> InputPart1 {
    raw_input
        .lines()
        .map(|line| line.parse().unwrap())
        .collect()
}

#[aoc_generator(day1, part2)]
pub fn input_generator_part2(raw_input: &str) -> InputPart2 {
    input_generator_part1(raw_input)
        .windows(3)
        .map(|window| window.iter().sum())
        .collect()
}

#[aoc(day1, part1)]
pub fn solve_part1(input: &InputPart1) -> OutputPart1 {
    input
        .iter()
        .zip(&input[1..])
        .filter(|(prev, next)| next > prev)
        .count()
}

#[aoc(day1, part2)]
pub fn solve_part2(input: &InputPart2) -> OutputPart2 {
    solve_part1(input)
}
