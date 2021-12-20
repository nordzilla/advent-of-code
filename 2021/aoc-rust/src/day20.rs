#![allow(unused)]

use aoc_runner_derive::*;
use flow_control::return_if;
use hashbrown::{HashMap, HashSet};
use itertools::Itertools;
use text_io::scan;

type Input = (Vec<i64>, HashMap<(i64, i64), i64>);
type Output = usize;

#[aoc_generator(day20, part1, nordzilla)]
#[aoc_generator(day20, part2, nordzilla)]
fn input_generator(raw_input: &str) -> Input {
    let mut lines = raw_input.lines();
    let line1 = lines.next().unwrap();
    let enhanced = line1
        .bytes()
        .map(|byte| if byte == b'.' { 0 } else { 1 })
        .collect();
    assert!(line1.len() == 512);

    let mut image = HashMap::new();

    lines.skip(1).enumerate().for_each(|(row, line)| {
        line.bytes().enumerate().for_each(|(col, byte)| {
            if byte == b'.' {
                image.insert((row as i64, col as i64), 0);
            } else {
                image.insert((row as i64, col as i64), 1);
            }
        });
    });
    (enhanced, image)
}

fn rectangle(
    (start_row, start_col): (i64, i64),
    rows: i64,
    cols: i64,
) -> impl Iterator<Item = (i64, i64)> {
    (0..rows).flat_map(move |row| (0..cols).map(move |col| (start_row + row, start_col + col)))
}

fn build_index(row: i64, col: i64, image: &HashMap<(i64, i64), i64>) -> usize {
    (0..9).rev().zip(rectangle((row - 1, col - 1), 3, 3)).fold(
        0,
        |index, (place_value, (row, col))| {
            let c = image.get(&(row, col)).unwrap_or(&0);
            index | ((*c as usize) << place_value)
        },
    )
}

fn enhance(pixels: &Vec<i64>, image: &HashMap<(i64, i64), i64>) -> HashMap<(i64, i64), i64> {
    let min_row = image.keys().min_by_key(|(row, _)| row).unwrap().0 - 2;
    let max_row = image.keys().max_by_key(|(row, _)| row).unwrap().0 + 2;
    let min_col = image.keys().min_by_key(|(_, col)| col).unwrap().1 - 2;
    let max_col = image.keys().max_by_key(|(_, col)| col).unwrap().1 + 2;
    let mut enhanced =
        HashMap::with_capacity((max_row - min_row) as usize * (max_col - min_col) as usize);
    for row in min_row..=max_row {
        for col in min_col..=max_col {
            enhanced.insert((row, col), pixels[build_index(row, col, image)]);
        }
    }
    enhanced
}

#[allow(unused)]
fn plot(image: &HashMap<(i64, i64), i64>) {
    let min_row = image.keys().min_by_key(|(row, _)| row).unwrap().0;
    let max_row = image.keys().max_by_key(|(row, _)| row).unwrap().0;
    let min_col = image.keys().min_by_key(|(_, col)| col).unwrap().1;
    let max_col = image.keys().max_by_key(|(_, col)| col).unwrap().1;
    for row in min_row..=max_row {
        for col in min_col..=max_col {
            match image.get(&(row, col)) {
                Some(0) => print!("."),
                Some(1) => print!("#"),
                _ => unreachable!(),
            }
        }
        println!();
    }
}

#[aoc(day20, part1, nordzilla)]
fn solve_part1((pixels, image): &Input) -> Output {
    let e1 = enhance(pixels, image);
    let e2 = enhance(pixels, &e1);
    plot(image);
    println!();
    plot(&e1);
    println!();
    plot(&e2);
    e2.values().filter(|&&value| value == 1).count()
}

#[aoc(day20, part2, nordzilla)]
fn solve_part2(input: &Input) -> Output {
    2
}

#[cfg(test)]
mod test {
    use super::*;
}
