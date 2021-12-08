use aoc_runner_derive::*;
use flow_control::return_if;

type Input = Vec<Vec<u8>>;
type Output = usize;

// The counts of how many segments are turned on for the seven-segment
// representations of the digits.
const ON0: u32 = 6;
const ON1: u32 = 2;
const ON2: u32 = 5;
const ON3: u32 = 5;
const ON4: u32 = 4;
const ON5: u32 = 5;
const ON6: u32 = 6;
const ON7: u32 = 3;
const ON8: u32 = 7;
const ON9: u32 = 6;

// The products of the counts of segments that are turned on
// for each digit that includes the segment. These products
// end up mapping to distinct values for each segment.
const A: u32 = ON0 * ON2 * ON3 * ON5 * ON6 * ON7 * ON8 * ON9;
const B: u32 = ON0 * ON4 * ON5 * ON6 * ON8 * ON9;
const C: u32 = ON0 * ON1 * ON2 * ON3 * ON4 * ON7 * ON8 * ON9;
const D: u32 = ON2 * ON3 * ON4 * ON5 * ON6 * ON8 * ON9;
const E: u32 = ON0 * ON2 * ON6 * ON8;
const F: u32 = ON0 * ON1 * ON3 * ON4 * ON5 * ON6 * ON7 * ON8 * ON9;
const G: u32 = ON0 * ON2 * ON3 * ON5 * ON6 * ON8 * ON9;

// The sums of the above weighted products for each segment that is
// required to light up each digit's sven-segment representation.
// These sums end up mapping to distinct values for each seven-segment digit.
const DIGIT0: u32 = A + B + C + E + F + G;
const DIGIT1: u32 = C + F;
const DIGIT2: u32 = A + C + D + E + G;
const DIGIT3: u32 = A + C + D + F + G;
const DIGIT4: u32 = B + C + D + F;
const DIGIT5: u32 = A + B + D + F + G;
const DIGIT6: u32 = A + B + D + E + F + G;
const DIGIT7: u32 = A + C + F;
const DIGIT8: u32 = A + B + C + D + E + F + G;
const DIGIT9: u32 = A + B + C + D + F + G;

// We create an array to accumulate the products for each turned-on segment
// multiplied by the length of the sequence it was contained in.
//
// In the end we will end up with some permutation of the [A, B, C, D, E, F, G]
// constants above that is automatically mapped to the correct indices when treating
// (b'a' as 0), (b'b' as 1), (b'c' as 2), and so on for indexing.
//
// Once we have this mapping, we need to just sum the values at the appropriate
// indices for each segment that is present in the observed output sequence.
//
// This will produce one of the DIGIT0, DIGIT1, DIGIT2... constants above.
fn parse_segment_map<'a>(mut iter: impl Iterator<Item = &'a str>) -> [u32; 7] {
    let mut segment_map = [1; 7];
    iter.next()
        .unwrap()
        .split_whitespace()
        .for_each(|sequence| {
            sequence.bytes().for_each(|byte| {
                segment_map[(byte - b'a') as usize] *= sequence.len() as u32;
            });
        });
    segment_map
}

// Once we have a weighted sum that maps to one of the above DIGIT0, DIGIT1... constants,
// we can map that directly to its integer value.
fn to_digit(weighted_segment_sum: u32) -> u8 {
    return_if!(weighted_segment_sum == DIGIT0, 0);
    return_if!(weighted_segment_sum == DIGIT1, 1);
    return_if!(weighted_segment_sum == DIGIT2, 2);
    return_if!(weighted_segment_sum == DIGIT3, 3);
    return_if!(weighted_segment_sum == DIGIT4, 4);
    return_if!(weighted_segment_sum == DIGIT5, 5);
    return_if!(weighted_segment_sum == DIGIT6, 6);
    return_if!(weighted_segment_sum == DIGIT7, 7);
    return_if!(weighted_segment_sum == DIGIT8, 8);
    return_if!(weighted_segment_sum == DIGIT9, 9);
    unreachable!("invalid segment sum");
}

#[aoc_generator(day8)]
fn input_generator(raw_input: &str) -> Input {
    raw_input
        .lines()
        .map(|line| {
            let mut split = line.split(" | ");
            let segment_map = parse_segment_map(&mut split);
            split
                .next()
                .unwrap()
                .split_whitespace()
                .map(move |sequence| {
                    sequence
                        .bytes()
                        .map(|byte| segment_map[(byte - b'a') as usize])
                        .sum::<u32>()
                })
                .map(to_digit)
                .collect()
        })
        .collect()
}

#[aoc(day8, part1)]
fn solve_part1(input: &Input) -> Output {
    input
        .iter()
        .flatten()
        .filter(|&&n| n == 1 || n == 4 || n == 7 || n == 8)
        .count()
}

#[aoc(day8, part2)]
fn solve_part2(input: &Input) -> Output {
    input
        .iter()
        .flat_map(|outputs| {
            outputs
                .iter()
                .rev()
                .zip((0..).map(|n| 10_usize.pow(n)))
                .map(|(&digit, place_value)| digit as usize * place_value)
        })
        .sum()
}
