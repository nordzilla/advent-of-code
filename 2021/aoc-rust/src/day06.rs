use aoc_runner_derive::*;

type Input = [u64; 9];
type Output = u64;

#[aoc_generator(day6)]
fn input_generator_arrays(raw_input: &str) -> Input {
    raw_input
        .split(',')
        .map(|n| n.parse::<usize>().unwrap())
        .fold([0; 9], |mut arr, timer| {
            arr[timer] += 1;
            arr
        })
}

fn spawn_count(fish: [u64; 9], days_left: u64) -> u64 {
    (0..days_left)
        .fold(fish, |[t0, t1, t2, t3, t4, t5, t6, t7, t8], _| {
            [t1, t2, t3, t4, t5, t6, t7 + t0, t8, t0]
        })
        .into_iter()
        .sum()
}

#[aoc(day6, part1)]
fn solve_part1(&input: &Input) -> Output {
    spawn_count(input, 80)
}

#[aoc(day6, part2)]
fn solve_part2(&input: &Input) -> Output {
    spawn_count(input, 256)
}
