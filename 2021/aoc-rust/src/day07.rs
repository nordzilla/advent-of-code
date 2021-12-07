use aoc_runner_derive::*;
use itertools::Itertools;

type Input = Vec<i64>;
type Output = i64;

#[aoc_generator(day7)]
fn input_generator_arrays(raw_input: &str) -> Input {
    raw_input
        .split(',')
        .map(|n| n.parse().unwrap())
        .sorted()
        .collect()
}

#[derive(Clone, Copy)]
enum CostType {
    Constant,
    Increasing,
}

fn iter_expanding_from(n: i64) -> impl Iterator<Item = (i64, i64)> {
    (1..).map(move |dist| (n.saturating_sub(dist), n.saturating_add(dist)))
}

fn fuel_efficiency(input: &Input, target: i64, cost_type: CostType) -> i64 {
    input
        .iter()
        .map(|&location| {
            let cost = (location - target).abs();
            match cost_type {
                CostType::Constant => cost,
                CostType::Increasing => cost * (cost + 1) / 2,
            }
        })
        .sum()
}

fn fuel_efficient_convergence(input: &Input, start_point: i64, cost_type: CostType) -> i64 {
    let mut left_min = fuel_efficiency(input, start_point, cost_type);
    let mut right_min = left_min;
    for (left_cost, right_cost) in
        iter_expanding_from(start_point).map(|(next_left, next_right)| {
            (
                fuel_efficiency(input, next_left, cost_type),
                fuel_efficiency(input, next_right, cost_type),
            )
        })
    {
        flow_control::break_if!(left_cost > left_min && right_cost > right_min);
        (left_cost < left_min).then(|| left_min = left_cost);
        (right_cost < right_min).then(|| right_min = right_cost);
    }
    std::cmp::min(left_min, right_min)
}

#[aoc(day7, part1)]
fn solve_part1(input: &Input) -> Output {
    let median = input.iter().copied().nth(input.len() / 2).unwrap();
    fuel_efficient_convergence(input, median, CostType::Constant)
}

#[aoc(day7, part2)]
fn solve_part2(input: &Input) -> Output {
    let average = input.iter().sum::<i64>() / input.len() as i64;
    fuel_efficient_convergence(input, average, CostType::Increasing)
}
