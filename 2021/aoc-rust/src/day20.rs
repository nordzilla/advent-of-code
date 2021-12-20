use aoc_runner_derive::*;
use hashbrown::HashMap;

type Input = (i64, Vec<i64>, HashMap<(i64, i64), i64>);
type Output = usize;

#[aoc_generator(day20, part1, nordzilla)]
#[aoc_generator(day20, part2, nordzilla)]
fn input_generator(raw_input: &str) -> Input {
    let mut lines = raw_input.lines();
    let line1 = lines.next().unwrap();
    let enhanced_pixels = line1
        .bytes()
        .map(|byte| if byte == b'.' { 0 } else { 1 })
        .collect::<Vec<_>>();
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
    (enhanced_pixels[0] + 1, enhanced_pixels, image)
}

fn rectangle(
    (start_row, start_col): (i64, i64),
    rows: i64,
    cols: i64,
) -> impl Iterator<Item = (i64, i64)> {
    (0..rows).flat_map(move |row| (0..cols).map(move |col| (start_row + row, start_col + col)))
}

fn build_index(row: i64, col: i64, image: &HashMap<(i64, i64), i64>, oob: i64) -> usize {
    (0..9).rev().zip(rectangle((row - 1, col - 1), 3, 3)).fold(
        0,
        |index, (place_value, (row, col))| {
            let c = image.get(&(row, col)).unwrap_or(&oob);
            index | ((*c as usize) << place_value)
        },
    )
}

fn enhance(
    pixels: &Vec<i64>,
    image: &HashMap<(i64, i64), i64>,
    oob: i64,
) -> HashMap<(i64, i64), i64> {
    let min_row = image.keys().min_by_key(|(row, _)| row).unwrap().0 - 1;
    let min_col = image.keys().min_by_key(|(_, col)| col).unwrap().1 - 1;
    let max_row = image.keys().max_by_key(|(row, _)| row).unwrap().0 + 1;
    let max_col = image.keys().max_by_key(|(_, col)| col).unwrap().1 + 1;
    let mut enhanced =
        HashMap::with_capacity((max_row - min_row) as usize * (max_col - min_col) as usize);
    for row in min_row..=max_row {
        for col in min_col..=max_col {
            enhanced.insert((row, col), pixels[build_index(row, col, image, oob)]);
        }
    }
    enhanced
}

#[aoc(day20, part1, nordzilla)]
fn solve_part1((divisor, pixels, image): &Input) -> Output {
    (0..2)
        .fold(image.clone(), |image, i| {
            enhance(pixels, &image, i % divisor)
        })
        .values()
        .filter(|&&value| value == 1)
        .count()
}

#[aoc(day20, part2, nordzilla)]
fn solve_part2((divisor, pixels, image): &Input) -> Output {
    (0..50)
        .fold(image.clone(), |image, i| {
            enhance(pixels, &image, i % divisor)
        })
        .values()
        .filter(|&&value| value == 1)
        .count()
}
