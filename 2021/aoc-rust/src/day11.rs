use aoc_runner_derive::*;
use std::iter::repeat_with;

type Input = Vec<Vec<u8>>;
type Output = usize;

#[aoc_generator(day11)]
fn input_generator(raw_input: &str) -> Input {
    raw_input
        .lines()
        .map(|line| line.bytes().map(|byte| byte - b'0').collect())
        .collect()
}

fn grid_points_iter(
    (start_row, start_col): (usize, usize),
    rows: usize,
    cols: usize,
) -> impl Iterator<Item = (usize, usize)> {
    (0..rows).flat_map(move |row| (0..cols).map(move |col| (start_row + row, start_col + col)))
}

fn flash(grid: &mut Input) -> usize {
    grid_points_iter((0, 0), grid.len(), grid[0].len())
        .flat_map(|(row, col)| {
            (grid[row][col] > 9).then(|| {
                grid_points_iter((row - 1, col - 1), 3, 3).for_each(|(row, col)| {
                    if let Some(n) = grid.get_mut(row).and_then(|row| row.get_mut(col)) {
                        (*n != 0).then(|| *n += 1);
                    }
                });
                grid[row][col] = 0;
                1
            })
        })
        .sum()
}

fn step(grid: &mut Input) -> usize {
    grid_points_iter((0, 0), grid.len(), grid[0].len()).for_each(|(row, col)| grid[row][col] += 1);
    repeat_with(|| flash(grid))
        .take_while(|&flashes| flashes > 0)
        .sum()
}

#[aoc(day11, part1)]
fn solve_part1(input: &Input) -> Output {
    let mut grid = input.clone();
    (0..100).map(|_| step(&mut grid)).sum()
}

#[aoc(day11, part2)]
fn solve_part2(input: &Input) -> Output {
    let mut grid = input.clone();
    (1..)
        .map(|n| (n, step(&mut grid)))
        .find(|&(_, flashes)| flashes == (input.len() * input[0].len()))
        .unwrap()
        .0
}
