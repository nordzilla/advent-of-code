use aoc_runner_derive::*;

use itertools::Itertools;

type InputPart1 = String;
type InputPart2 = InputPart1;

type OutputPart1 = i64;
type OutputPart2 = OutputPart1;

#[aoc_generator(day2, part1)]
pub fn input_generator_part1(raw_input: &str) -> InputPart1 {
    raw_input.into()
}

#[aoc_generator(day2, part2)]
pub fn input_generator_part2(raw_input: &str) -> InputPart2 {
    input_generator_part1(raw_input)
}

#[aoc(day2, part1)]
pub fn solve_part1(input: &InputPart1) -> OutputPart1 {
    unreachable!()
}

#[aoc(day2, part2)]
pub fn solve_part2(input: &InputPart2) -> OutputPart2 {
    unreachable!()
}