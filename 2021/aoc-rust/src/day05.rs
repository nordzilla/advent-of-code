use aoc_runner_derive::*;
use itertools::Itertools;
use std::{cmp::Ordering::*, iter};

type Input = Vec<Line>;
type Output = usize;

#[derive(Copy, Clone)]
struct Line {
    x1: i16,
    y1: i16,
    x2: i16,
    y2: i16,
}

impl From<((i16, i16), (i16, i16))> for Line {
    fn from(((x1, y1), (x2, y2)): ((i16, i16), (i16, i16))) -> Self {
        Self { x1, y1, x2, y2 }
    }
}

impl Line {
    fn is_horizontal(self) -> bool {
        self.y1 == self.y2
    }

    fn is_vertical(self) -> bool {
        self.x1 == self.x2
    }

    #[rustfmt::skip]
    fn points(self) -> impl Iterator<Item = (i16, i16)> {
        iter::successors(Some((self.x1, self.y1)), move |&(x, y)| {
            match (self.x2.cmp(&x), self.y2.cmp(&y)) {
                (Equal,   Equal)   => None,
                (Equal,   Less)    => Some((x + 0, y - 1)),
                (Equal,   Greater) => Some((x + 0, y + 1)),
                (Less,    Equal)   => Some((x - 1, y + 0)),
                (Less,    Less)    => Some((x - 1, y - 1)),
                (Less,    Greater) => Some((x - 1, y + 1)),
                (Greater, Equal)   => Some((x + 1, y + 0)),
                (Greater, Less)    => Some((x + 1, y - 1)),
                (Greater, Greater) => Some((x + 1, y + 1)),
            }
        })
    }
}

#[aoc_generator(day5)]
fn input_generator(raw_input: &str) -> Input {
    raw_input
        .lines()
        .map(|line| {
            let mut split = line.split(" -> ");
            let mut start_point = split
                .next()
                .unwrap()
                .split(',')
                .map(|n| n.parse::<i16>().unwrap());
            let mut end_point = split
                .next()
                .unwrap()
                .split(',')
                .map(|n| n.parse::<i16>().unwrap());
            (
                (start_point.next().unwrap(), start_point.next().unwrap()),
                (end_point.next().unwrap(), end_point.next().unwrap()),
            )
                .into()
        })
        .collect::<Vec<_>>()
}

fn count_overlapping(input: &Input, predicate: impl FnMut(&&Line) -> bool) -> Output {
    input
        .iter()
        .filter(predicate)
        .flat_map(|line| line.points())
        .into_group_map_by(|&point| point)
        .values()
        .filter(|group| group.len() > 1)
        .count()
}

#[aoc(day5, part1)]
fn solve_part1(input: &Input) -> Output {
    count_overlapping(input, |line| line.is_vertical() || line.is_horizontal())
}

#[aoc(day5, part2)]
fn solve_part2(input: &Input) -> Output {
    count_overlapping(input, |_| true)
}
