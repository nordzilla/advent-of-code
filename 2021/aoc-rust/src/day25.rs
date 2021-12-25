use aoc_runner_derive::*;

type Input = Vec<Vec<u8>>;
type Output = usize;

#[aoc_generator(day25, part1, nordzilla)]
#[aoc_generator(day25, part2, nordzilla)]
fn input_generator(raw_input: &str) -> Input {
    raw_input
        .trim()
        .lines()
        .map(|line| line.trim().bytes().collect())
        .collect()
}

fn step_right(grid: &mut Vec<Vec<u8>>) -> usize {
    let mut moved = 0;
    for row in grid {
        let width = row.len();
        for col in 0..width {
            if b'>' == row[col] {
                if b'.' == row[(col + 1) % width] {
                    moved += 1;
                    row[col] = b'x';
                    row[(col + 1) % width] = b'<';
                }
            }
        }
    }
    moved
}

fn step_down(grid: &mut Vec<Vec<u8>>) -> usize {
    let mut moved = 0;
    let height = grid.len();
    for row in 0..height {
        for col in 0..grid[row].len() {
            if b'v' == grid[row][col] {
                if b'.' == grid[(row + 1) % height][col] {
                    moved += 1;
                    grid[row][col] = b'x';
                    grid[(row + 1) % height][col] = b'^';
                }
            }
        }
    }
    moved
}

fn swippy_swappy(grid: &mut Vec<Vec<u8>>) -> usize {
    for row in grid {
        for cell in row {
            match cell {
                b'<' => *cell = b'>',
                b'^' => *cell = b'v',
                b'x' => *cell = b'.',
                _ => (),
            }
        }
    }
    0
}

fn step(grid: &mut Vec<Vec<u8>>) -> usize {
    step_right(grid) + swippy_swappy(grid) + step_down(grid) + swippy_swappy(grid)
}

#[aoc(day25, part1, nordzilla)]
fn solve_part1(input: &Input) -> Output {
    let mut input = input.clone();
    std::iter::repeat_with(move || step(&mut input))
        .zip(1..)
        .find_map(|(moved, n)| (moved == 0).then(|| n))
        .unwrap()
}

#[aoc(day25, part2, nordzilla)]
fn solve_part2(_: &Input) -> Output {
    12_25_2021
}
