use aoc_runner_derive::*;
use std::cmp::Ordering;
use text_io::scan;

type Dots = Vec<(i16, i16)>;
type Input = (Dots, Vec<Fold>);
type Output = usize;

#[derive(Debug, Copy, Clone)]
enum Fold {
    Up(i16),
    Left(i16),
}

#[aoc_generator(day13)]
fn input_generator(raw_input: &str) -> Input {
    let mut iter = raw_input.split("\n\n");
    let dots = iter
        .next()
        .unwrap()
        .lines()
        .map(|line| {
            let [x, y]: [i16; 2];
            scan!(line.bytes() => "{},{}", x, y);
            (x, y)
        })
        .collect();
    let folds = iter
        .next()
        .unwrap()
        .lines()
        .map(|line| {
            let direction: char;
            let location: i16;
            scan!(line.bytes() => "fold along {}={}", direction, location);
            if direction == 'y' {
                Fold::Up(location)
            } else {
                Fold::Left(location)
            }
        })
        .collect();
    (dots, folds)
}

impl Fold {
    fn apply_to(self, (x, y): (i16, i16)) -> Option<(i16, i16)> {
        match self {
            Fold::Up(location) => match y.cmp(&location) {
                Ordering::Equal => None,
                Ordering::Less => Some((x, y)),
                Ordering::Greater => Some((x, -y + 2 * location)),
            },
            Fold::Left(location) => match x.cmp(&location) {
                Ordering::Equal => None,
                Ordering::Less => Some((x, y)),
                Ordering::Greater => Some((-x + 2 * location, y)),
            },
        }
    }
}

fn apply_folds(dots: Dots, fold: Fold) -> Dots {
    dots.into_iter()
        .flat_map(|dot| fold.apply_to(dot))
        .collect()
}

#[allow(unused)]
fn plot(dots: &Dots) {
    let min_x = dots.iter().min_by_key(|(x, _)| x).unwrap().0;
    let max_x = dots.iter().max_by_key(|(x, _)| x).unwrap().0;
    let min_y = dots.iter().min_by_key(|(_, y)| y).unwrap().1;
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
        dots = apply_folds(dots, instruction);
    }
    dots.sort();
    dots.dedup();
    dots.len()
}

#[aoc(day13, part2)]
fn solve_part2((dots, folds): &Input) -> Output {
    let mut dots = dots.clone();
    for &instruction in folds.iter() {
        dots = apply_folds(dots, instruction);
    }
    //plot(&dots);
    867_5309
}
