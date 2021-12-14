use aoc_runner_derive::*;

const fn get_index(lhs: usize, rhs: usize) -> usize {
    lhs << 7 | rhs
}

const fn get_pair(index: usize) -> (usize, usize) {
    ((index >> 7) & 0b1111111, index & 0b1111111)
}

#[test]
fn test_index_roundtrips() {
    for lhs in b'A'..=b'Z' {
        let lhs = lhs as usize;
        for rhs in b'A'..=b'Z' {
            let rhs = rhs as usize;
            assert_eq!(get_pair(get_index(lhs, rhs)), (lhs, rhs));
        }
    }
}

const MIN_INDEX: usize = get_index('A' as usize, 'A' as usize);
const MAX_INDEX: usize = get_index('Z' as usize, 'Z' as usize);
type Input = [(usize, Option<usize>); MAX_INDEX + 1];
type Output = usize;

#[aoc_generator(day14)]
fn map_generator(raw_map: &str) -> Input {
    let mut map = [(0, None); MAX_INDEX + 1];
    let mut iter = raw_map.split("\n\n");
    let seed = iter
        .next()
        .unwrap()
        .bytes()
        .map(|byte| byte as usize)
        .collect::<Vec<_>>();
    for &byte in &seed {
        map[byte].0 += 1;
    }
    for w in seed.windows(2) {
        map[get_index(w[0], w[1])].0 += 1;
    }
    iter.next().unwrap().lines().for_each(|line| {
        let mut iter = line.split(" -> ");
        let pair = iter.next().unwrap();
        let c1 = pair.as_bytes().get(0).copied().unwrap() as usize;
        let c2 = pair.as_bytes().get(1).copied().unwrap() as usize;
        let c3 = iter.next().unwrap().chars().next().unwrap() as usize;
        let idx = get_index(c1, c2);
        map[idx].1 = Some(c3);
    });
    map
}

fn step(in_map: Input) -> Input {
    let mut out_map = in_map;
    for i in MIN_INDEX..=MAX_INDEX {
        if let (count, Some(insertion)) = in_map[i] {
            let (c1, c2) = get_pair(i);
            out_map[i].0 -= count;
            out_map[insertion].0 += count;
            out_map[get_index(c1, insertion)].0 += count;
            out_map[get_index(insertion, c2)].0 += count;
        }
    }
    out_map
}

fn max_minus_min(map: &Input) -> usize {
    let max = map['A' as usize..='Z' as usize]
        .iter()
        .map(|&(count, _)| count)
        .filter(|&count| count > 0)
        .max()
        .unwrap();
    let min = map['A' as usize..='Z' as usize]
        .iter()
        .map(|&(count, _)| count)
        .filter(|&count| count > 0)
        .min()
        .unwrap();
    max - min
}

#[aoc(day14, part1)]
fn solve_part1(&map: &Input) -> Output {
    max_minus_min(&(0..10).fold(map, |map, _| step(map)))
}

#[aoc(day14, part2)]
fn solve_part2(&map: &Input) -> Output {
    max_minus_min(&(0..40).fold(map, |map, _| step(map)))
}
