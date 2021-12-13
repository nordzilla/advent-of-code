#![allow(unused)]

use aoc_runner_derive::*;
use flow_control::return_if;
use std::collections::{BTreeMap, BTreeSet};
use text_io::scan;

type Dots = BTreeSet<(i32, i32)>;
type Input = (Dots, Vec<Fold>);
type Output = usize;

#[derive(Debug, Copy, Clone)]
enum Fold {
    Up(i32),
    Left(i32),
}

#[aoc_generator(day13)]
fn input_generator(raw_input: &str) -> Input {
    let mut iter = raw_input.split("\n\n");
    let dots = iter
        .next()
        .unwrap()
        .lines()
        .map(|line| {
            let [x, y]: [i32; 2];
            scan!(line.bytes() => "{},{}", x, y);
            (x, y)
        })
        .collect();
    let folds = iter
        .next()
        .unwrap()
        .lines()
        .map(|line| {
            let direction: String;
            let location: i32;
            scan!(line.bytes() => "fold along {}={}", direction, location);
            if direction == "y" {
                Fold::Up(location)
            } else {
                Fold::Left(location)
            }
        })
        .collect();
    (dots, folds)
}

fn fold(dots: Dots, instruction: Fold) -> Dots {
    let (mut half1, mut half2): (Dots, Dots) = dots
        .into_iter()
        .filter(|&(x, y)| match instruction {
            Fold::Up(location) => y != location,
            Fold::Left(location) => x != location,
        })
        .partition(|&(x, y)| match instruction {
            Fold::Up(location) => y < location,
            Fold::Left(location) => x < location,
        });

    match instruction {
        Fold::Up(location) => {
            half2 = half2
                .into_iter()
                .map(|(x, y)| (x, y - 2 * (y - location)))
                .collect();
        }
        Fold::Left(location) => {
            half2 = half2
                .into_iter()
                .map(|(x, y)| (x - 2 * (x - location), y))
                .collect();
        }
    }
    half1.union(&half2).into_iter().copied().collect()
}

fn plot(dots: &Dots) {
    let min_x = dots.iter().min_by_key(|(x, _)| x).unwrap().0;
    let min_y = dots.iter().min_by_key(|(_, y)| y).unwrap().1;

    let max_x = dots.iter().max_by_key(|(x, _)| x).unwrap().0;
    let max_y = dots.iter().max_by_key(|(_, y)| y).unwrap().1;

    for y in min_y..=max_y {
        for x in min_x..=max_x {
            if dots.contains(&(x, y)) {
                print!("#");
            } else {
                print!(".")
            }
        }
        println!();
    }
}

#[aoc(day13, part1)]
fn solve_part1((dots, folds): &Input) -> Output {
    let mut dots = dots.clone();
    for &instruction in folds.iter().take(1) {
        dots = fold(dots, instruction);
    }
    dots.into_iter().count()
}

#[aoc(day13, part2)]
fn solve_part2((dots, folds): &Input) -> Output {
    let mut dots = dots.clone();
    for &instruction in folds.iter() {
        dots = fold(dots, instruction);
    }
    plot(&dots);
    dots.into_iter().count()
}
