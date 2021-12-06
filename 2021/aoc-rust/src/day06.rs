use std::{cell::RefCell, collections::HashMap};

use aoc_runner_derive::*;
use flow_control::return_if;

type Input = Vec<i64>;
type Output = i64;

thread_local! {
    static SPAWN_CACHE: RefCell<HashMap<(i64, i64), i64>> = RefCell::new(HashMap::new());
}

fn spawn_count(days_left: i64, timer: i64) -> i64 {
    return_if!(days_left - timer <= 0, 1);
    if let Some(count) = SPAWN_CACHE.with(|cache| cache.borrow().get(&(days_left, timer)).copied())
    {
        return count;
    }
    let count = 1
        + (0..days_left - timer)
            .rev()
            .step_by(7)
            .map(|days_left| spawn_count(days_left, 8))
            .sum::<i64>();
    SPAWN_CACHE.with(|cache| {
        cache
            .borrow_mut()
            .entry((days_left, timer))
            .or_insert(count)
            .clone()
    })
}

#[aoc_generator(day6)]
fn input_generator(raw_input: &str) -> Input {
    raw_input
        .split(',')
        .map(|n| n.parse::<i64>().unwrap())
        .collect()
}

#[aoc(day6, part1)]
fn solve_part1(input: &Input) -> Output {
    input.iter().map(|&timer| spawn_count(80, timer)).sum()
}

#[aoc(day6, part2)]
fn solve_part2(input: &Input) -> Output {
    input.iter().map(|&timer| spawn_count(256, timer)).sum()
}
