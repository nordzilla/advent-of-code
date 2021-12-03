use aoc_runner_derive::*;

type Input = Vec<i64>;
type Output = i64;

#[aoc_generator(day3)]
fn input_generator(raw_input: &str) -> Input {
    raw_input
        .lines()
        .map(|line| i64::from_str_radix(line, 2).unwrap())
        .collect()
}

fn has_more_zeros(input: &Input, on_bit: i64) -> bool {
    input.iter().filter(|&&n| 0 == n & on_bit).count() > input.len() / 2
}

#[aoc(day3, part1)]
fn solve_part1(input: &Input) -> Output {
    let (gamma_rate, epsilon_rate) = (0..12)
        .rev()
        .map(|order| {
            let on_bit = 1 << order;
            has_more_zeros(input, on_bit)
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
    let retain_ones = |ratings: &mut Input, on_bit| ratings.retain(|n| 0 != n & on_bit);
    let retain_zeros = |ratings: &mut Input, on_bit| ratings.retain(|n| 0 == n & on_bit);

    for on_bit in (0..12).rev().map(|order| 1 << order) {
        if ratings.len() == 1 {
            break;
        }
        match preference {
            Preference::Majority => has_more_zeros(&ratings, on_bit)
                .then(|| retain_zeros(&mut ratings, on_bit))
                .unwrap_or_else(|| retain_ones(&mut ratings, on_bit)),
            Preference::Minority => has_more_zeros(&ratings, on_bit)
                .then(|| retain_ones(&mut ratings, on_bit))
                .unwrap_or_else(|| retain_zeros(&mut ratings, on_bit)),
        }
    }

    ratings[0]
}

#[aoc(day3, part2)]
fn solve_part2(input: &Input) -> Output {
    find_rating(input, Preference::Majority) * find_rating(input, Preference::Minority)
}
