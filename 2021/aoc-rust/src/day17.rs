use aoc_runner_derive::*;
use std::{cmp::Ordering, ops::RangeInclusive};
use text_io::scan;

type Input = (RangeInclusive<i64>, RangeInclusive<i64>);
type Output = i64;

#[derive(Debug, Copy, Clone)]
struct Probe {
    x_vel: i64,
    y_vel: i64,
    x_pos: i64,
    y_pos: i64,
    max_y_pos: i64,
}

impl Probe {
    fn step(self) -> Self {
        let mut p = self.go().apply_effects();
        if p.y_pos > self.y_pos {
            p.max_y_pos = p.y_pos;
        }
        p
    }

    fn go(self) -> Self {
        Self {
            x_pos: self.x_pos + self.x_vel,
            y_pos: self.y_pos + self.y_vel,
            ..self
        }
    }

    fn apply_effects(self) -> Self {
        Self {
            x_vel: self.x_vel.approach(0),
            y_vel: self.y_vel - 1,
            ..self
        }
    }
}

trait Approach {
    fn approach(self, target: Self) -> Self;
}

impl Approach for i64 {
    fn approach(self, target: Self) -> Self {
        match target.cmp(&self) {
            Ordering::Less => self - 1,
            Ordering::Equal => self,
            Ordering::Greater => self + 1,
        }
    }
}

#[aoc_generator(day17)]
fn input_generator(raw_input: &str) -> Input {
    let [x_min, x_max]: [i64; 2];
    let [y_min, y_max]: [i64; 2];
    scan!(raw_input.bytes() => "target area: x={}..{}, y={}..{}", x_min, x_max, y_min, y_max);
    (x_min..=x_max, y_min..=y_max)
}

fn get_max_ys((x_range, y_range): &Input) -> Vec<i64> {
    let mut ys = Vec::new();
    for x_vel in 22..281 {
        for y_vel in -73..73 {
            let mut probe = Probe {
                x_vel,
                y_vel,
                x_pos: 0,
                y_pos: 0,
                max_y_pos: 0,
            };
            while probe.x_pos < *x_range.end() && probe.y_pos > *y_range.start() {
                probe = probe.step();
                if x_range.contains(&probe.x_pos) && y_range.contains(&probe.y_pos) {
                    break;
                }
            }
            if x_range.contains(&probe.x_pos) && y_range.contains(&probe.y_pos) {
                ys.push(probe.max_y_pos);
            }
        }
    }
    ys
}

#[aoc(day17, part1)]
fn solve_part1(input: &Input) -> Output {
    get_max_ys(input).into_iter().max().unwrap()
}

#[aoc(day17, part2)]
fn solve_part2(input: &Input) -> Output {
    get_max_ys(input).into_iter().count() as i64
}
