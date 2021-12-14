use aoc_runner_derive::*;
use flow_control::{continue_if, return_if};

const MAX_INDEX: usize = single_index(b'Z' as u16);
const MAX_PAIR_INDEX: usize = combined_index(b'Z' as u16, b'Z' as u16);
type CountsMap = [u64; MAX_INDEX + 1];
type PairsMap = [(u64, Option<u16>); MAX_PAIR_INDEX + 1];
type Input = (CountsMap, PairsMap);
type Output = u64;

const fn single_index(c: u16) -> usize {
    (c - (b'A' as u16)) as usize
}

const fn combined_index(c1: u16, c2: u16) -> usize {
    ((c1 - (b'A' as u16)) << 5 | (c2 - (b'A' as u16))) as usize
}

const fn pair_from(index: usize) -> (u16, u16) {
    (
        ((index as u16 >> 5) & 0b11111) + b'A' as u16,
        (index as u16 & 0b11111) + b'A' as u16,
    )
}

#[aoc_generator(day14, part1, nordzilla)]
#[aoc_generator(day14, part2, nordzilla)]
fn input_generator(raw_map: &str) -> Input {
    let mut counts = [0; MAX_INDEX + 1];
    let mut pairs = [(0, None); MAX_PAIR_INDEX + 1];
    let mut iter = raw_map.split("\n\n");
    let seed = iter
        .next()
        .unwrap()
        .bytes()
        .map(|byte| byte as u16)
        .collect::<Vec<_>>();
    for &byte in &seed {
        counts[single_index(byte)] += 1;
    }
    for w in seed.windows(2) {
        pairs[combined_index(w[0], w[1])].0 += 1;
    }
    iter.next().unwrap().lines().for_each(|line| {
        let mut iter = line.split(" -> ");
        let pair = iter.next().unwrap();
        let c1 = pair.as_bytes().get(0).copied().unwrap() as u16;
        let c2 = pair.as_bytes().get(1).copied().unwrap() as u16;
        let c3 = iter.next().unwrap().chars().next().unwrap() as u16;
        pairs[combined_index(c1, c2)].1 = Some(c3);
    });
    (counts, pairs)
}

fn step(counts: &mut CountsMap, in_pairs: &PairsMap) -> PairsMap {
    let mut out_pairs = *in_pairs;
    for index in 0..=MAX_PAIR_INDEX {
        if let (count, Some(c3)) = in_pairs[index] {
            continue_if!(count == 0);
            let (c1, c2) = pair_from(index);
            out_pairs[index].0 -= count;
            out_pairs[combined_index(c1, c3)].0 += count;
            out_pairs[combined_index(c3, c2)].0 += count;
            counts[single_index(c3)] += count;
        }
    }
    out_pairs
}

fn max_minus_min(counts: &CountsMap) -> u64 {
    let (min, max) = counts
        .iter()
        .fold((u64::MAX, u64::MIN), |(min, max), &value| {
            return_if!(value == 0, (min, max));
            (
                if value < min { value } else { min },
                if value > max { value } else { max },
            )
        });
    max - min
}

#[aoc(day14, part1, nordzilla)]
fn solve_part1(&(mut counts, pairs): &Input) -> Output {
    (0..10).fold(pairs, |pairs, _| step(&mut counts, &pairs));
    max_minus_min(&counts)
}

#[aoc(day14, part2, nordzilla)]
fn solve_part2(&(mut counts, pairs): &Input) -> Output {
    (0..40).fold(pairs, |pairs, _| step(&mut counts, &pairs));
    max_minus_min(&counts)
}
