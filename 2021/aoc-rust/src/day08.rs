use aoc_runner_derive::*;

type Input = Vec<(Vec<u8>, Vec<u8>)>;
type Output = usize;

const BIMAP_LEN: usize = 138;
type BiMap = [u8; BIMAP_LEN];

/// Values for collecting segment sequences into a single u8 representation.
#[rustfmt::skip]
static SEGMENT_BITS: [u8; 7] = [
    1 << 0,
    1 << 1,
    1 << 2,
    1 << 3,
    1 << 4,
    1 << 5,
    1 << 6,
];

/// Values guaranteed to not be a u8 sequence mapping. Since the seven-pin
/// mappings are contained within the first seven bits, all of these values
/// have the 8th bit set.
static SEQUENCE_KEY: [usize; 10] = [
    0 | 128,
    1 | 128,
    2 | 128,
    3 | 128,
    4 | 128,
    5 | 128,
    6 | 128,
    7 | 128,
    8 | 128,
    9 | 128,
];

fn parse_sequences<'a>(mut iter: impl Iterator<Item = &'a str>) -> Vec<u8> {
    iter.next()
        .unwrap()
        .split_whitespace()
        .map(|sequence| {
            sequence
                .bytes()
                .map(|byte| SEGMENT_BITS[(byte - b'a') as usize])
                .fold(0, |byte, bit| byte | bit)
        })
        .collect()
}

#[aoc_generator(day8)]
fn input_generator_arrays(raw_input: &str) -> Input {
    raw_input
        .lines()
        .map(|line| {
            let mut split = line.split(" | ");
            (parse_sequences(&mut split), parse_sequences(&mut split))
        })
        .collect()
}

/// Takes a digit and its expected pop count as a segment sequence.
fn decode_unique(digit: usize, pop_count: u32, sequences: &[u8], bi_map: &mut BiMap) {
    let sequence = sequences
        .iter()
        .copied()
        .find(|sequence| sequence.count_ones() == pop_count)
        .unwrap();
    bi_map[SEQUENCE_KEY[digit]] = sequence;
    bi_map[sequence as usize] = digit as u8;
}

fn xor_pop_count(lhs: u8, rhs: u8) -> u32 {
    (lhs ^ rhs).count_ones()
}

/// Takes a digit and its expected segment pop-count when xored with
/// 1, 4, 7, and 8 respectively as segment sequences.
///
/// Xoring the the segments sequences that have non-unique counts produces
/// a distinct mapping when xored with the 1, 4, 7, and 8's segment sequences.
fn decode_from_known(
    digit: usize,
    [xor_pop1, xor_pop4, xor_pop7, xor_pop8]: [u32; 4],
    sequences: &[u8],
    bi_map: &mut BiMap,
) {
    let sequence = sequences
        .iter()
        .copied()
        .find(|&sequence| {
            xor_pop_count(sequence, bi_map[SEQUENCE_KEY[1]]) == xor_pop1
                && xor_pop_count(sequence, bi_map[SEQUENCE_KEY[4]]) == xor_pop4
                && xor_pop_count(sequence, bi_map[SEQUENCE_KEY[7]]) == xor_pop7
                && xor_pop_count(sequence, bi_map[SEQUENCE_KEY[8]]) == xor_pop8
        })
        .unwrap();
    bi_map[SEQUENCE_KEY[digit]] = sequence;
    bi_map[sequence as usize] = digit as u8;
}

fn decode(sequences: &[u8]) -> BiMap {
    let mut bi_map = [0; BIMAP_LEN];

    // Get the numbers with unique pins.
    decode_unique(1, 2, sequences, &mut bi_map);
    decode_unique(4, 4, sequences, &mut bi_map);
    decode_unique(7, 3, sequences, &mut bi_map);
    decode_unique(8, 7, sequences, &mut bi_map);

    // Derive other numbers by xoring with the ones above.
    decode_from_known(0, [4, 4, 3, 1], sequences, &mut bi_map);
    decode_from_known(2, [5, 5, 4, 2], sequences, &mut bi_map);
    decode_from_known(3, [3, 3, 2, 2], sequences, &mut bi_map);
    decode_from_known(5, [5, 3, 4, 2], sequences, &mut bi_map);
    decode_from_known(6, [6, 4, 5, 1], sequences, &mut bi_map);
    decode_from_known(9, [4, 2, 3, 1], sequences, &mut bi_map);

    bi_map
}

#[aoc(day8, part1)]
fn solve_part1(input: &Input) -> Output {
    input
        .iter()
        .flat_map(|(notes, outputs)| {
            let bi_map = decode(notes);
            outputs
                .iter()
                .map(move |&sequence| bi_map[sequence as usize])
        })
        .filter(|&n| n == 1 || n == 4 || n == 7 || n == 8)
        .count()
}

#[aoc(day8, part2)]
fn solve_part2(input: &Input) -> Output {
    input
        .iter()
        .flat_map(|(notes, outputs)| {
            let bi_map = decode(notes);
            outputs
                .iter()
                .rev()
                .map(move |&sequence| bi_map[sequence as usize] as usize)
                .zip((0..).map(|n| 10_usize.pow(n)))
                .map(|(digit, place_value)| digit * place_value)
        })
        .sum()
}
