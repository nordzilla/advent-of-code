use aoc_runner_derive::*;
use flow_control::return_if;
use std::{cell::RefCell, collections::HashMap};

type Input = Vec<i16>;
type Output = i64;

thread_local! {
    static FISHIES_CACHE: RefCell<HashMap<(i16, i16), i64>> = RefCell::new(HashMap::new());
}

#[aoc_generator(day6)]
fn input_generator(raw_input: &str) -> Input {
    raw_input.split(',').map(|n| n.parse().unwrap()).collect()
}

fn spawn_count(days_left: i16, timer: i16) -> i64 {
    return_if!(days_left - timer <= 0, 1);
    let count = FISHIES_CACHE
        .with(|cache| cache.borrow().get(&(days_left, timer)).copied())
        .unwrap_or_else(|| {
            1 + (0..days_left - timer)
                .rev()
                .step_by(7)
                .map(|days_left| spawn_count(days_left, 8))
                .sum::<i64>()
        });
    FISHIES_CACHE.with(|cache| {
        cache
            .borrow_mut()
            .entry((days_left, timer))
            .or_insert(count)
            .clone()
    })
}

#[aoc(day6, part1)]
fn solve_part1(input: &Input) -> Output {
    input.iter().map(|&timer| spawn_count(80, timer)).sum()
}

#[aoc(day6, part2)]
fn solve_part2(input: &Input) -> Output {
    input.iter().map(|&timer| spawn_count(256, timer)).sum()
}
