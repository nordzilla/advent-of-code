use aoc_runner_derive::*;

use itertools::Itertools;
use std::collections::HashSet;

type InputPart1 = Vec<i64>;
type InputPart2 = InputPart1;

type OutputPart1 = i64;
type OutputPart2 = OutputPart1;

#[aoc_generator(day1, part1)]
pub fn input_generator_part1(raw_input: &str) -> InputPart1 {
    raw_input
        .lines()
        .map(|n| n.parse().unwrap())
        .collect()
}

#[aoc_generator(day1, part2)]
pub fn input_generator_part2(raw_input: &str) -> InputPart2 {
    input_generator_part1(raw_input)
}

#[aoc(day1, part1, Nested)]
pub fn solve_part1_nested(input: &InputPart1) -> OutputPart1 {
    for entry1 in input {
        for entry2 in input {
            if entry1 + entry2 == 2020 {
                return entry1 * entry2;
            }
        }
    }
    unreachable!()
}

#[aoc(day1, part1, Hash)]
pub fn solve_part1_hash(input: &InputPart1) -> OutputPart1 {
    let set = input.iter().copied().collect::<HashSet<_>>();
    for &entry in &set {
        let entry2 = 2020 - entry;
        if set.contains(&entry2) {
            return entry * entry2;
        }
    }
    unreachable!()
}

#[aoc(day1, part1, Itertools)]
pub fn solve_part1_itertools(input: &InputPart1) -> OutputPart1 {
    for entry in input.iter().combinations(2) {
        if entry[0] + entry[1] == 2020 {
            return entry[0] * entry[1];
        }
    }
    unreachable!()
}

#[aoc(day1, part2, Nested)]
pub fn solve_part2_nested(input: &InputPart2) -> OutputPart2 {
    for entry1 in input {
        for entry2 in input {
            for entry3 in input {
                if entry1 + entry2 + entry3 == 2020 {
                    return entry1 * entry2 * entry3;
                }
            }
        }
    }
    unreachable!()
}

#[aoc(day1, part2, Itertools)]
pub fn solve_part2_itertools(input: &InputPart2) -> OutputPart2 {
    for entry in input.iter().combinations(3) {
        if entry[0] + entry[1] + entry[2] == 2020 {
            return entry[0] * entry[1] * entry[2];
        }
    }
    unreachable!()
}