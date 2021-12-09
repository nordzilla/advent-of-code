use aoc_runner_derive::*;
use itertools::Itertools;
use std::iter;

type Input = Vec<Vec<u8>>;
type Output = i64;

#[aoc_generator(day9)]
fn input_generator(raw_input: &str) -> Input {
    raw_input
        .lines()
        .map(|line| line.bytes().map(|byte| byte - b'0').collect::<Vec<_>>())
        .collect()
}

#[derive(Debug, Copy, Clone, Default, Hash, PartialEq, Eq)]
struct Point {
    x: i64,
    y: i64,
}

impl Point {
    fn new((x, y): (i64, i64)) -> Self {
        Self { x, y }
    }

    fn up(mut self) -> Self {
        self.y -= 1;
        self
    }

    fn down(mut self) -> Self {
        self.y += 1;
        self
    }

    fn left(mut self) -> Self {
        self.x -= 1;
        self
    }

    fn right(mut self) -> Self {
        self.x += 1;
        self
    }

    fn value(self, grid: &Input) -> Option<u8> {
        grid.get(self.y as usize)
            .and_then(|row| row.get(self.x as usize))
            .copied()
    }

    fn is_lower_than(self, other: Self, grid: &Input) -> bool {
        self.value(grid)
            .map(|lhs| other.value(grid).map(|rhs| lhs < rhs).unwrap_or(true))
            .unwrap_or(false)
    }

    fn is_low_point(self, grid: &Input) -> bool {
        self.adjacent_points()
            .all(|point| self.is_lower_than(point, grid))
    }

    fn adjacent_points(self) -> impl Iterator<Item = Point> {
        [self.up(), self.right(), self.down(), self.left()].into_iter()
    }

    fn basin(self, grid: &Input) -> Box<dyn Iterator<Item = Point> + '_> {
        Box::new(
            std::iter::once(self)
                .chain(
                    self.adjacent_points()
                        .filter_map(move |point| {
                            (self.is_lower_than(point, grid)
                                && point.value(grid).filter(|&value| value < 9).is_some())
                            .then(move || point.basin(grid))
                        })
                        .flatten(),
                )
                .unique(),
        )
    }
}

fn low_points(grid: &Input) -> impl Iterator<Item = Point> + '_ {
    (0..grid[0].len() as i64)
        .flat_map(|x| iter::repeat(x).zip(0..grid.len() as i64))
        .map(Point::new)
        .filter(|point| point.is_low_point(grid))
}

#[aoc(day9, part1)]
fn solve_part1(input: &Input) -> Output {
    low_points(input)
        .filter_map(|point| point.value(input))
        .map(|value| value as i64 + 1)
        .sum()
}

#[aoc(day9, part2)]
fn solve_part2(input: &Input) -> Output {
    low_points(input)
        .map(|point| point.basin(input).count() as i64)
        .sorted()
        .rev()
        .take(3)
        .product()
}
