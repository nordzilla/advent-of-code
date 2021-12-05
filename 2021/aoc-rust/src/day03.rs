use aoc_runner_derive::*;
use flow_control::break_if;

type Input = Vec<i64>;
type Output = i64;

#[aoc_generator(day3)]
fn input_generator(raw_input: &str) -> Input {
    raw_input
        .lines()
        .map(|line| i64::from_str_radix(line, 2).unwrap())
        .collect()
}

fn has_more_zeros_at(on_bit: i64, input: &Input) -> bool {
    input.iter().filter(|&&n| 0 == n & on_bit).count() > input.len() / 2
}

fn on_bits_iter() -> impl Iterator<Item = i64> {
    (0..12).rev().map(|place_value| 1 << place_value)
}

#[aoc(day3, part1)]
fn solve_part1(input: &Input) -> Output {
    let (gamma_rate, epsilon_rate) = on_bits_iter()
        .map(|on_bit| {
            has_more_zeros_at(on_bit, input)
                .then(|| (0, on_bit))
                .unwrap_or((on_bit, 0))
        })
        .fold(
            (0, 0),
            |(gamma_rate, epsilon_rate), (gamma_bit, epsilon_bit)| {
                (gamma_rate | gamma_bit, epsilon_rate | epsilon_bit)
            },
        );
    gamma_rate * epsilon_rate
}

enum Preference {
    Majority,
    Minority,
}

fn find_rating(input: &Input, preference: Preference) -> i64 {
    let mut ratings = input.clone();
    let retain_ones_at = |on_bit, ratings: &mut Input| ratings.retain(|n| 0 != n & on_bit);
    let retain_zeros_at = |on_bit, ratings: &mut Input| ratings.retain(|n| 0 == n & on_bit);

    for on_bit in on_bits_iter() {
        break_if!(ratings.len() == 1);
        match preference {
            Preference::Majority => has_more_zeros_at(on_bit, &ratings)
                .then(|| retain_zeros_at(on_bit, &mut ratings))
                .unwrap_or_else(|| retain_ones_at(on_bit, &mut ratings)),
            Preference::Minority => has_more_zeros_at(on_bit, &ratings)
                .then(|| retain_ones_at(on_bit, &mut ratings))
                .unwrap_or_else(|| retain_zeros_at(on_bit, &mut ratings)),
        }
    }

    ratings[0]
}

#[aoc(day3, part2)]
fn solve_part2(input: &Input) -> Output {
    find_rating(input, Preference::Majority) * find_rating(input, Preference::Minority)
}
