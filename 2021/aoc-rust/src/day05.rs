use aoc_runner_derive::*;

use itertools::Itertools;
use std::{cmp::Ordering, iter};
use text_io::scan;

type Input = Vec<Line>;
type Output = usize;

struct Line {
    x1: i16,
    y1: i16,
    x2: i16,
    y2: i16,
}

impl From<[i16; 4]> for Line {
    fn from([x1, y1, x2, y2]: [i16; 4]) -> Self {
        Self { x1, y1, x2, y2 }
    }
}

#[aoc_generator(day5)]
fn input_generator(raw_input: &str) -> Input {
    raw_input
        .lines()
        .map(|line| {
            let [x1, y1, x2, y2]: [i16; 4];
            scan!(line.bytes() => "{},{} -> {},{}", x1, y1, x2, y2);
            [x1, y1, x2, y2].into()
        })
        .collect()
}

trait Approach {
    fn approach(self, target: Self) -> Self;
}

impl Approach for i16 {
    fn approach(self, target: Self) -> Self {
        match target.cmp(&self) {
            Ordering::Less => self - 1,
            Ordering::Equal => self,
            Ordering::Greater => self + 1,
        }
    }
}

impl Line {
    fn is_not_diagonal(&self) -> bool {
        self.x1 == self.x2 || self.y1 == self.y2
    }

    fn points(&Self { x1, y1, x2, y2 }: &Self) -> impl Iterator<Item = (i16, i16)> + '_ {
        iter::successors(Some((x1, y1)), move |&(x, y)| {
            (x != x2 || y != y2).then(|| (x.approach(x2), y.approach(y2)))
        })
    }
}

fn count_intersections(input: &Input, predicate: impl FnMut(&&Line) -> bool) -> Output {
    input
        .iter()
        .filter(predicate)
        .flat_map(Line::points)
        .counts()
        .values()
        .filter(|&&count| count > 1)
        .count()
}

#[aoc(day5, part1)]
fn solve_part1(input: &Input) -> Output {
    count_intersections(input, |line| line.is_not_diagonal())
}

#[aoc(day5, part2)]
fn solve_part2(input: &Input) -> Output {
    count_intersections(input, |_| true)
}
