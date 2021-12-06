use aoc_runner_derive::*;

type Input = ();
type Output = u64;
const START_FISH: [u64; 9] = [0, 225, 20, 13, 20, 22, 0, 0, 0];

const fn spawn([t0, t1, t2, t3, t4, t5, t6, t7, t8]: [u64; 9]) -> [u64; 9] {
    [t1, t2, t3, t4, t5, t6, t7 + t0, t8, t0]
}

const fn count_fish([t0, t1, t2, t3, t4, t5, t6, t7, t8]: [u64; 9]) -> u64 {
    t0 + t1 + t2 + t3 + t4 + t5 + t6 + t7 + t8
}

macro_rules! const_spawn_count {
    ($fish:expr) => { count_fish($fish) };
    ($fish:expr, $($commas:tt)*) => { const_spawn_count!(spawn($fish) $($commas)*) };
}

#[aoc_generator(day6)]
fn input_generator_arrays(_: &str) -> () {}

#[aoc(day6, part1)]
fn solve_part1(_: &Input) -> Output {
    const_spawn_count!(START_FISH,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,)
}

#[aoc(day6, part2)]
fn solve_part2(_: &Input) -> Output {
    const_spawn_count!(START_FISH,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,,)
}
