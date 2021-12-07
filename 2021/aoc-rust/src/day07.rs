use aoc_runner_derive::*;

type Input = Vec<i64>;
type Output = i64;

#[aoc_generator(day7)]
fn input_generator_arrays(raw_input: &str) -> Input {
    raw_input.split(',').map(|n| n.parse().unwrap()).collect()
}

fn iter_expanding_from(n: i64) -> impl Iterator<Item = (i64, i64)> {
    (1..).map(move |dist| (n.saturating_sub(dist), n.saturating_add(dist)))
}

fn fuel_efficiency(input: &Input, target: i64, increasing_cost: bool) -> i64 {
    input
        .iter()
        .map(|&location| {
            let cost = (location - target).abs();
            increasing_cost
                .then(|| (cost * (cost + 1)) / 2)
                .unwrap_or(cost)
        })
        .sum()
}

fn fuel_efficient_convergence(input: &Input, increasing_cost: bool) -> i64 {
    let average = input.iter().sum::<i64>() / input.len() as i64;
    let (mut left_min, mut right_min) = (
        fuel_efficiency(input, average, increasing_cost),
        fuel_efficiency(input, average, increasing_cost),
    );
    for (left_cost, right_cost) in iter_expanding_from(average).map(|(next_left, next_right)| {
        (
            fuel_efficiency(input, next_left, increasing_cost),
            fuel_efficiency(input, next_right, increasing_cost),
        )
    }) {
        flow_control::break_if!(left_cost > left_min && right_cost > right_min);
        (left_cost < left_min).then(|| left_min = left_cost);
        (right_cost < right_min).then(|| right_min = right_cost);
    }
    std::cmp::min(left_min, right_min)
}

#[aoc(day7, part1)]
fn solve_part1(input: &Input) -> Output {
    fuel_efficient_convergence(input, false)
}

#[aoc(day7, part2)]
fn solve_part2(input: &Input) -> Output {
    fuel_efficient_convergence(input, true)
}
