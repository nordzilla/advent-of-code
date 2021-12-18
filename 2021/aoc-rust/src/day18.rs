use aoc_runner_derive::*;
use flow_control::return_if;
use itertools::Itertools;
use std::fmt;
use std::iter::Peekable;

type Input = Vec<SfNumber>;
type Output = i64;

#[derive(Debug, Clone)]
pub enum SfNumber {
    Single {
        height: i64,
        value: i64,
    },
    Pair {
        height: i64,
        lhs: Box<SfNumber>,
        rhs: Box<SfNumber>,
    },
}

impl SfNumber {
    fn new_single(value: i64) -> Self {
        SfNumber::Single { height: 0, value }
    }

    fn new_pair(lhs: SfNumber, rhs: SfNumber) -> Self {
        SfNumber::Pair {
            height: std::cmp::max(lhs.height(), rhs.height()) + 1,
            lhs: Box::new(lhs),
            rhs: Box::new(rhs),
        }
    }

    fn is_single(&self) -> bool {
        matches!(self, SfNumber::Single { .. })
    }

    fn height(&self) -> i64 {
        match self {
            SfNumber::Single { height: depth, .. } => *depth,
            SfNumber::Pair { height: depth, .. } => *depth,
        }
    }

    #[cfg(test)]
    fn to_string(&self) -> String {
        match self {
            SfNumber::Single { value, .. } => value.to_string(),
            SfNumber::Pair { lhs, rhs, .. } => {
                format!("[{},{}]", lhs.to_string(), rhs.to_string())
            }
        }
    }
}

//==================================================================================
// Parser
//==================================================================================

type ParseOption<'a, T> = Option<(T, &'a [u8])>;

fn next(bytes: &[u8]) -> ParseOption<u8> {
    (!bytes.is_empty())
        .then(|| bytes.split_at(1))
        .map(|(next, bytes)| (next[0], bytes))
}

fn take_while(predicate: impl Fn(u8) -> bool, bytes: &[u8]) -> ParseOption<&[u8]> {
    bytes
        .iter()
        .position(|&byte| !predicate(byte))
        .and_then(|i| (i > 0).then(|| bytes.split_at(i)))
}

fn require(predicate: impl Fn(u8) -> bool, bytes: &[u8]) -> ParseOption<u8> {
    next(bytes).filter(|&(b, _)| predicate(b))
}

fn parse_match(byte: u8, bytes: &[u8]) -> ParseOption<u8> {
    require(|b| b == byte, bytes)
}

fn parse_value(bytes: &[u8]) -> ParseOption<i64> {
    take_while(|byte| (b'0'..=b'9').contains(&byte), bytes).and_then(|(digits, bytes)| {
        i64::from_str_radix(unsafe { std::str::from_utf8_unchecked(digits) }, 10)
            .ok()
            .map(|value| (value, bytes))
    })
}

fn parse_sf_number(bytes: &[u8]) -> ParseOption<SfNumber> {
    parse_single(bytes).or_else(|| parse_pair(bytes))
}

fn parse_single(bytes: &[u8]) -> ParseOption<SfNumber> {
    parse_value(bytes).map(|(value, bytes)| (SfNumber::new_single(value), bytes))
}

fn parse_pair(bytes: &[u8]) -> ParseOption<SfNumber> {
    parse_match(b'[', bytes).and_then(|(_, bytes)| {
        parse_sf_number(bytes).and_then(|(lhs, bytes)| {
            parse_match(b',', bytes).and_then(|(_, bytes)| {
                parse_sf_number(bytes).and_then(|(rhs, bytes)| {
                    parse_match(b']', bytes).map(|(_, bytes)| (SfNumber::new_pair(lhs, rhs), bytes))
                })
            })
        })
    })
}

#[aoc_generator(day18, part1, nordzilla)]
#[aoc_generator(day18, part2, nordzilla)]
fn input_generator(raw_input: &str) -> Input {
    raw_input
        .lines()
        .map(|line| parse_sf_number(line.as_bytes()).unwrap().0)
        .collect()
}

//==================================================================================
// Arithmetic
//==================================================================================

impl SfNumber {
    fn magnitude(&self) -> i64 {
        match self {
            SfNumber::Single { value, .. } => *value,
            SfNumber::Pair { lhs, rhs, .. } => 3 * lhs.magnitude() + 2 * rhs.magnitude(),
        }
    }

    fn add(self, rhs: SfNumber) -> Self {
        SfNumber::new_pair(self, rhs).reduce()
    }

    fn reduce(mut self) -> Self {
        let mut exploded_values = None;
        self = self.explode(5, &mut exploded_values);
        if exploded_values.is_some() || self.split() {
            return self.reduce();
        }
        self
    }

    fn split(&mut self) -> bool {
        match self {
            SfNumber::Single { value, .. } => {
                if *value > 9 {
                    *self = SfNumber::new_pair(
                        SfNumber::new_single(*value / 2),
                        SfNumber::new_single(*value / 2 + *value % 2),
                    );
                    return true;
                }
                false
            }
            SfNumber::Pair { height, lhs, rhs } => {
                let split = lhs.split() || rhs.split();
                if split {
                    *height += 1;
                }
                split
            }
        }
    }

    fn accept_value_right(&mut self, n: &mut i64) {
        return_if!(*n == 0);
        match self {
            SfNumber::Single { value, .. } => {
                *value += *n;
                *n = 0;
            }
            SfNumber::Pair { rhs, .. } => {
                rhs.accept_value_right(n);
            }
        }
    }

    fn accept_value_left(&mut self, n: &mut i64) {
        return_if!(*n == 0);
        match self {
            SfNumber::Single { value, .. } => {
                *value += *n;
                *n = 0;
            }
            SfNumber::Pair { lhs, .. } => {
                lhs.accept_value_left(n);
            }
        }
    }

    fn explode(self, threshold: i64, values: &mut Option<(i64, i64)>) -> Self {
        return_if!(self.height() < threshold, self);
        match self {
            this @ SfNumber::Single { .. } => this,
            SfNumber::Pair {
                height,
                lhs,
                mut rhs,
            } => {
                if lhs.is_single() && rhs.is_single() {
                    *values = Some((lhs.magnitude(), rhs.magnitude()));
                    SfNumber::new_single(0)
                } else {
                    let mut lhs = lhs.explode(height - 1, values);
                    return_if!(values.is_some(), {
                        let (_, rv) = values.as_mut().unwrap();
                        rhs.accept_value_left(rv);
                        SfNumber::new_pair(lhs, *rhs)
                    });

                    let rhs = rhs.explode(height - 1, values);
                    return_if!(values.is_some(), {
                        let (lv, _) = values.as_mut().unwrap();
                        lhs.accept_value_right(lv);
                        SfNumber::new_pair(lhs, rhs)
                    });

                    SfNumber::new_pair(lhs, rhs)
                }
            }
        }
    }
}

#[aoc(day18, part1, nordzilla)]
fn solve_part1(input: &Input) -> Output {
    input
        .iter()
        .cloned()
        .reduce(|lhs, rhs| lhs.add(rhs))
        .unwrap()
        .magnitude()
}

#[aoc(day18, part2, nordzilla)]
fn solve_part2(input: &Input) -> Output {
    input
        .iter()
        .cloned()
        .combinations(2)
        .map(|combo| {
            std::cmp::max(
                combo[0].clone().add(combo[1].clone()).magnitude(),
                combo[1].clone().add(combo[0].clone()).magnitude(),
            )
        })
        .max()
        .unwrap()
}

#[cfg(test)]
mod test {
    use super::*;

    fn test_roundtrip(input: &str) {
        assert_eq!(
            parse_sf_number(input.as_bytes()).unwrap().0.to_string(),
            input,
            "{}",
            input
        );
    }

    #[test]
    fn test_to_string() {
        test_roundtrip("[1,2]");
        test_roundtrip("[[1,2],3]");
        test_roundtrip("[9,[8,7]]");
        test_roundtrip("[[1,9],[8,5]]");
        test_roundtrip("[[[[1,2],[3,4]],[[5,6],[7,8]]],9]");
        test_roundtrip("[[[9,[3,8]],[[0,9],6]],[[[3,7],[4,9]],3]]");
        test_roundtrip("[[[[1,3],[5,3]],[[1,3],[8,7]]],[[[4,9],[6,9]],[[8,2],[7,3]]]]");
        test_roundtrip("[[[[0,7],4],[15,[0,13]]],[1,1]]");
    }

    fn test_explode(before: &str, after: &str) {
        let mut values = None;
        let number = parse_sf_number(before.as_bytes()).unwrap().0;
        let exploded = number.explode(5, &mut values);
        assert!(dbg!(values).is_some());
        assert_eq!(exploded.to_string(), after);
    }

    #[test]
    fn test_explodes() {
        test_explode("[[[[[9,8],1],2],3],4]", "[[[[0,9],2],3],4]");
        test_explode("[7,[6,[5,[4,[3,2]]]]]", "[7,[6,[5,[7,0]]]]");
        test_explode("[[6,[5,[4,[3,2]]]],1]", "[[6,[5,[7,0]]],3]");
        test_explode(
            "[[3,[2,[1,[7,3]]]],[6,[5,[4,[3,2]]]]]",
            "[[3,[2,[8,0]]],[9,[5,[4,[3,2]]]]]",
        );
        test_explode(
            "[[3,[2,[8,0]]],[9,[5,[4,[3,2]]]]]",
            "[[3,[2,[8,0]]],[9,[5,[7,0]]]]",
        );
        test_explode(
            "[[[[[4,3],4],4],[7,[[8,4],9]]],[1,1]]",
            "[[[[0,7],4],[7,[[8,4],9]]],[1,1]]",
        );
        test_explode(
            "[[[[0,7],4],[7,[[8,4],9]]],[1,1]]",
            "[[[[0,7],4],[15,[0,13]]],[1,1]]",
        );
        test_explode(
            "[[[[0,7],4],[[7,8],[0,[6,7]]]],[1,1]]",
            "[[[[0,7],4],[[7,8],[6,0]]],[8,1]]",
        );
    }

    fn test_split(before: &str, after: &str) {
        let mut number = parse_sf_number(before.as_bytes()).unwrap().0;
        assert!(number.split());
        assert_eq!(number.to_string(), after, "\nbefore: {}", before,);
    }

    #[test]
    fn test_splits() {
        test_split(
            "[[[[0,7],4],[15,[0,13]]],[1,1]]",
            "[[[[0,7],4],[[7,8],[0,13]]],[1,1]]",
        );
        test_split(
            "[[[[0,7],4],[[7,8],[0,13]]],[1,1]]",
            "[[[[0,7],4],[[7,8],[0,[6,7]]]],[1,1]]",
        );
    }

    fn test_reduce(before: &str, after: &str) {
        assert_eq!(
            parse_sf_number(before.as_bytes())
                .unwrap()
                .0
                .reduce()
                .to_string(),
            after,
            "\nbefore: {}",
            before
        );
    }

    #[test]
    fn test_reduction() {
        test_reduce(
            "[[[[[4,3],4],4],[7,[[8,4],9]]],[1,1]]",
            "[[[[0,7],4],[[7,8],[6,0]]],[8,1]]",
        );
    }

    macro_rules! test_add {
        ($lhs:tt $(+ $rhs:tt)* = $output:expr) => {
            let actual = [
                $lhs,
                $($rhs),*
            ]
            .into_iter()
            .map(|input| parse_sf_number(input.as_bytes()).unwrap().0)
            .fold1(|lhs, rhs| lhs.add(rhs))
            .unwrap()
            .to_string();
            let expected = $output;
            assert_eq!(actual, expected);
        };
    }

    #[test]
    #[rustfmt::skip]
    fn test_addition() {
        test_add!(
            "[1,1]"
          + "[2,2]"
          + "[3,3]"
          + "[4,4]"
          = "[[[[1,1],[2,2]],[3,3]],[4,4]]"
        );
        test_add!(
            "[1,1]"
          + "[2,2]"
          + "[3,3]"
          + "[4,4]"
          + "[5,5]"
          = "[[[[3,0],[5,3]],[4,4]],[5,5]]"
        );
        test_add!(
            "[1,1]"
          + "[2,2]"
          + "[3,3]"
          + "[4,4]"
          + "[5,5]"
          + "[6,6]"
          = "[[[[5,0],[7,4]],[5,5]],[6,6]]"
        );
        test_add!(
            "[[[[4,3],4],4],[7,[[8,4],9]]]"
          + "[1,1]"
          = "[[[[0,7],4],[[7,8],[6,0]]],[8,1]]"
        );
        test_add!(
            "[[[0,[4,5]],[0,0]],[[[4,5],[2,6]],[9,5]]]"
          + "[7,[[[3,7],[4,3]],[[6,3],[8,8]]]]"
          = "[[[[4,0],[5,4]],[[7,7],[6,0]]],[[8,[7,7]],[[7,9],[5,0]]]]"
        );
        test_add!(
            "[[[[4,0],[5,4]],[[7,7],[6,0]]],[[8,[7,7]],[[7,9],[5,0]]]]"
          + "[[2,[[0,8],[3,4]]],[[[6,7],1],[7,[1,6]]]]"
          = "[[[[6,7],[6,7]],[[7,7],[0,7]]],[[[8,7],[7,7]],[[8,8],[8,0]]]]"
        );
        test_add!(
            "[[[[6,7],[6,7]],[[7,7],[0,7]]],[[[8,7],[7,7]],[[8,8],[8,0]]]]"
          + "[[[[2,4],7],[6,[0,5]]],[[[6,8],[2,8]],[[2,1],[4,5]]]]"
          = "[[[[7,0],[7,7]],[[7,7],[7,8]]],[[[7,7],[8,8]],[[7,7],[8,7]]]]"
        );
        test_add!(
            "[[[[7,0],[7,7]],[[7,7],[7,8]]],[[[7,7],[8,8]],[[7,7],[8,7]]]]"
          + "[7,[5,[[3,8],[1,4]]]]"
          = "[[[[7,7],[7,8]],[[9,5],[8,7]]],[[[6,8],[0,8]],[[9,9],[9,0]]]]"
        );
        test_add!(
            "[[[[6,6],[6,6]],[[6,0],[6,7]]],[[[7,7],[8,9]],[8,[8,1]]]]"
          + "[2,9]"
          = "[[[[6,6],[7,7]],[[0,7],[7,7]]],[[[5,5],[5,6]],9]]"
        );
        test_add!(
            "[[[[6,6],[7,7]],[[0,7],[7,7]]],[[[5,5],[5,6]],9]]"
          + "[1,[[[9,3],9],[[9,0],[0,7]]]]"
          = "[[[[7,8],[6,7]],[[6,8],[0,8]]],[[[7,7],[5,0]],[[5,5],[5,6]]]]"
        );
        test_add!(
            "[[[[7,8],[6,7]],[[6,8],[0,8]]],[[[7,7],[5,0]],[[5,5],[5,6]]]]"
          + "[[[5,[7,4]],7],1]"
          = "[[[[7,7],[7,7]],[[8,7],[8,7]]],[[[7,0],[7,7]],9]]"
        );
        test_add!(
            "[[[[7,7],[7,7]],[[8,7],[8,7]]],[[[7,0],[7,7]],9]]"
          + "[[[[4,2],2],6],[8,7]]"
          = "[[[[8,7],[7,7]],[[8,6],[7,7]]],[[[0,7],[6,6]],[8,7]]]"
        );
        test_add!(
            "[[[0,[4,5]],[0,0]],[[[4,5],[2,6]],[9,5]]]"
          + "[7,[[[3,7],[4,3]],[[6,3],[8,8]]]]"
          + "[[2,[[0,8],[3,4]]],[[[6,7],1],[7,[1,6]]]]"
          + "[[[[2,4],7],[6,[0,5]]],[[[6,8],[2,8]],[[2,1],[4,5]]]]"
          + "[7,[5,[[3,8],[1,4]]]]"
          + "[[2,[2,2]],[8,[8,1]]]"
          + "[2,9]"
          + "[1,[[[9,3],9],[[9,0],[0,7]]]]"
          + "[[[5,[7,4]],7],1]"
          + "[[[[4,2],2],6],[8,7]]"
          = "[[[[8,7],[7,7]],[[8,6],[7,7]]],[[[0,7],[6,6]],[8,7]]]"
        );
        test_add!(
            "[[[0,[5,8]],[[1,7],[9,6]]],[[4,[1,2]],[[1,4],2]]]"
          + "[[[5,[2,8]],4],[5,[[9,9],0]]]"
          + "[6,[[[6,2],[5,6]],[[7,6],[4,7]]]]"
          + "[[[6,[0,7]],[0,9]],[4,[9,[9,0]]]]"
          + "[[[7,[6,4]],[3,[1,3]]],[[[5,5],1],9]]"
          + "[[6,[[7,3],[3,2]]],[[[3,8],[5,7]],4]]"
          + "[[[[5,4],[7,7]],8],[[8,3],8]]"
          + "[[9,3],[[9,9],[6,[4,9]]]]"
          + "[[2,[[7,7],7]],[[5,8],[[9,3],[0,2]]]]"
          + "[[[[5,2],5],[8,[3,7]]],[[5,[7,5]],[4,4]]]"
          = "[[[[6,6],[7,6]],[[7,7],[7,0]]],[[[7,7],[7,7]],[[7,8],[9,9]]]]"
        );
    }

    fn test_magnitude(expected: i64, input: &str) {
        assert_eq!(
            parse_sf_number(input.as_bytes())
                .unwrap()
                .0
                .reduce()
                .magnitude(),
            expected,
        );
    }

    #[test]
    fn test_magnitudes() {
        test_magnitude(143, "[[1,2],[[3,4],5]]");
        test_magnitude(1384, "[[[[0,7],4],[[7,8],[6,0]]],[8,1]]");
        test_magnitude(445, "[[[[1,1],[2,2]],[3,3]],[4,4]]");
        test_magnitude(791, "[[[[3,0],[5,3]],[4,4]],[5,5]]");
        test_magnitude(1137, "[[[[5,0],[7,4]],[5,5]],[6,6]]");
        test_magnitude(
            3488,
            "[[[[8,7],[7,7]],[[8,6],[7,7]]],[[[0,7],[6,6]],[8,7]]]",
        );
        test_magnitude(
            4140,
            "[[[[6,6],[7,6]],[[7,7],[7,0]]],[[[7,7],[7,7]],[[7,8],[9,9]]]]",
        );
    }
}

//============================================================================
// jimb sed what!?
//============================================================================

use anyhow::{anyhow, bail, Result};
use aoc_runner_derive::{aoc, aoc_generator};
use std::fmt::Write;
use std::mem::swap;
use std::ops::Range;
use std::str::FromStr;

pub fn cartesian_product<A, B>(a: A, b: B) -> impl Iterator<Item = (A::Item, B::Item)> + Clone
where
    A: IntoIterator,
    B: IntoIterator,
    A::Item: Clone,
    A::IntoIter: Clone,
    B::IntoIter: Clone,
{
    let a = a.into_iter();
    let b = b.into_iter();
    a.flat_map(move |i| b.clone().map(move |j| (i.clone(), j)))
}

fn explode(num: &str, out: &mut String) -> bool {
    out.clear();

    let mut last_n: Option<Range<usize>> = None;
    let mut depth = 0;
    for (start, ch) in num.char_indices() {
        match ch {
            '[' => {
                depth += 1;
                if depth > 4 {
                    let (left, right, pair_len) = get_num_pair(&num[start..]);
                    if let Some(ref range) = last_n {
                        let value = u64::from_str(&num[range.clone()]).unwrap();
                        write!(
                            out,
                            "{}{}{}",
                            &num[..range.start],
                            value + left,
                            &num[range.end..start]
                        )
                        .unwrap();
                    } else {
                        out.push_str(&num[..start]);
                    }
                    out.push_str("0");
                    match num[start + pair_len..].find(|ch: char| ch.is_digit(10)) {
                        Some(right_start) => {
                            let (n, right_len) = get_num(&num[start + pair_len + right_start..]);
                            write!(
                                out,
                                "{}{}{}",
                                &num[start + pair_len..start + pair_len + right_start],
                                right + n,
                                &num[start + pair_len + right_start + right_len..]
                            )
                            .unwrap();
                        }
                        None => {
                            out.push_str(&num[start + pair_len..]);
                        }
                    }

                    return true;
                }
            }
            ']' => depth -= 1,
            '0'..='9' => match last_n {
                None => {
                    last_n = Some(start..start + 1);
                }
                Some(ref mut range) => {
                    if range.end == start {
                        range.end = start + 1;
                    } else {
                        last_n = Some(start..start + 1);
                    }
                }
            },
            ',' => {}
            _ => panic!("unexpected character: {:?}", ch),
        }
    }

    false
}

fn get_num_pair(s: &str) -> (u64, u64, usize) {
    let bytes = s.as_bytes();
    assert_eq!(bytes[0], b'[');
    let mut pos = 1;
    let mut left = 0;
    while let Some(digit) = (bytes[pos] as char).to_digit(10) {
        left = left * 10 + digit as u64;
        pos += 1;
    }
    assert_eq!(bytes[pos], b',');
    pos += 1;
    let mut right = 0;
    while let Some(digit) = (bytes[pos] as char).to_digit(10) {
        right = right * 10 + digit as u64;
        pos += 1;
    }
    assert_eq!(bytes[pos], b']');
    (left, right, pos + 1)
}

fn get_num(s: &str) -> (u64, usize) {
    let bytes = s.as_bytes();
    let mut pos = 0;
    let mut n = 0;
    while let Some(digit) = (bytes[pos] as char).to_digit(10) {
        n = n * 10 + digit as u64;
        pos += 1;
    }
    (n, pos)
}

fn split(num: &str, out: &mut String) -> bool {
    out.clear();

    let mut rest = 0;
    while let Some(pos) = num[rest..].find(|ch: char| ch.is_digit(10)) {
        let (n, len) = get_num(&num[rest + pos..]);

        if n >= 10 {
            write!(
                out,
                "{}[{},{}]{}",
                &num[..rest + pos],
                n / 2,
                n - n / 2,
                &num[rest + pos + len..]
            )
            .unwrap();
            return true;
        }

        rest += pos + len;
    }

    false
}

fn reduce(num: &mut String, temp: &mut String) {
    loop {
        if explode(num, temp) {
            swap(num, temp);
            continue;
        }

        if split(num, temp) {
            swap(num, temp);
            continue;
        }

        break;
    }
}

fn magnitude(num: &str) -> u64 {
    let mut stack = vec![];
    let mut n = 0;
    for ch in num.chars() {
        match ch {
            '0'..='9' => {
                n = n * 10 + ch.to_digit(10).unwrap() as u64;
            }
            '[' => {}
            ',' => {
                stack.push(n);
                n = 0;
            }
            ']' => {
                let left = stack.pop().unwrap();
                n = 3 * left + 2 * n;
            }
            _ => panic!("unexpected character {:?}", ch),
        }
    }
    n
}

fn sum_list<'i, I, T>(list: I, out: &mut String, temp: &mut String)
where
    I: IntoIterator<Item = &'i T> + 'i,
    T: AsRef<str> + 'i,
    T: ?Sized,
{
    let mut list = list.into_iter();
    let mut left = list.next().unwrap().as_ref();
    for right in list {
        temp.clear();
        write!(temp, "[{},{}]", left, right.as_ref()).unwrap();
        reduce(temp, out);
        swap(temp, out);
        left = &out;
    }
}

#[aoc_generator(day18, part1, jimb_sed)]
#[aoc_generator(day18, part2, jimb_sed)]
fn generator(input: &str) -> Vec<String> {
    input.lines().map(|s| s.to_owned()).collect()
}

#[aoc(day18, part1, jimb_sed)]
fn part1(input: &Vec<String>) -> u64 {
    let mut out = String::new();
    let mut temp = String::new();
    sum_list(input, &mut out, &mut temp);
    magnitude(&out)
}

#[aoc(day18, part2, jimb_sed)]
fn part2(input: &Vec<String>) -> u64 {
    let mut sum = String::new();
    let mut temp = String::new();

    cartesian_product(0..input.len(), 0..input.len())
        .filter(|(i, j)| i != j)
        .map(|(i, j)| {
            sum.clear();
            write!(sum, "[{},{}]", &input[i], &input[j]).unwrap();
            reduce(&mut sum, &mut temp);
            magnitude(&sum)
        })
        .max()
        .unwrap()
}

//============================================================================
// jimb heaped it!
//============================================================================

type Value = u8;

/// A snailfish number represented as a tree flattened into an array.
///
/// A three-level-deep tree, with pairs nested at most two deep, would have
/// nodes placed like this in the array:
///
///     0   1   2   3   4   5   6   7
///
///             +------root-----+
///         +---*---+       +---*---+
///         *       *       *       *
///
/// If a node is a constant, then its entire subtree is left empty.
///
/// In general, a tree with pairs nested at most N deep needs an N + 1 level tree,
/// which requires 2^(N + 1) elements.
///
/// This lets us store no pointers, hold all data in a compact range of memory,
/// use array scans to find nearest neighbors in the tree, and use bit twiddling
/// to find children and parents.
struct Basis<const N: usize> {
    elts: [Elt; N],
}

#[derive(Clone, Copy, Debug, PartialEq)]
enum Elt {
    Empty,
    Pair,
    Value(Value),
}

impl<const N: usize> fmt::Debug for Basis<N> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for i in 0..N {
            match self.elts[i] {
                Elt::Empty => write!(f, ". ")?,
                Elt::Pair => write!(f, "* ")?,
                Elt::Value(v) => write!(f, "{:<2}", v)?,
            }
        }
        writeln!(f)?;
        write!(f, "  ")?;
        for i in 1..N {
            let ch = match (i - 1) & !i {
                0 => ' ',
                1 => '_',
                3 => '.',
                7 => '-',
                15 => '^',
                31 => '*',
                _ => panic!("unexpected mask"),
            };
            write!(f, "{} ", ch)?;
        }
        writeln!(f)?;

        Ok(())
    }
}

/// AoC trees are temporarily 5 deep, so we need 2^(5+1) = 64 elements.
type AocBasis = Basis<64>;

type Node = usize;

/// N should be a power of two.
impl<const N: usize> Basis<N> {
    const ROOT: usize = (N + 1) / 2;

    fn new() -> Self {
        Basis {
            elts: [Elt::Empty; N],
        }
    }

    fn clear(&mut self) {
        self.elts.fill(Elt::Empty);
    }

    fn is_pair(&self, n: Node) -> bool {
        self.elts[n] == Elt::Pair
    }

    fn children(&self, n: Node) -> (Node, Node) {
        assert!(self.is_pair(n));
        // the bit below the least significant 1-bit.
        let d = (n & !(n - 1)) / 2;
        (n - d, n + d)
    }

    /// The extent of the subtree rooted at the node n.
    fn subtree(&self, n: Node) -> Range<Node> {
        // all bits below the least significant 1-bit.
        let d = (n - 1) & !n;
        n - d..n + d
    }

    // This doesn't preserve tree invariants; the caller
    // must promise to set both children as well.
    fn set_pair(&mut self, n: Node) -> (Node, Node) {
        assert!(n & 1 == 0, "too low to be a pair");
        self.elts[n] = Elt::Pair;
        self.children(n)
    }

    fn set_const(&mut self, n: Node, v: Value) {
        let t = self.subtree(n);
        self.elts[t].fill(Elt::Empty);
        self.elts[n] = Elt::Value(v);
    }

    fn get(&self, n: Node) -> Value {
        match self.elts[n] {
            Elt::Value(v) => v,
            _ => panic!("element at {} is not a value", n),
        }
    }

    fn get_mut(&mut self, n: Node) -> &mut Value {
        match self.elts[n] {
            Elt::Value(ref mut v) => v,
            _ => panic!("element at {} is not a value", n),
        }
    }

    /// Find the next node to explode, if any. Assume this is the deepest possible level.
    fn find_too_deep(&self) -> Option<Node> {
        // The deepest possible pairs appear at multiples of four remainder 2.
        (0..N / 4)
            .map(|n| 4 * n + 2)
            .find(|&n| self.elts[n] == Elt::Pair)
    }

    /// Find the first node that needs to split.
    fn find_needs_split(&self) -> Option<Node> {
        self.elts.iter().position(|&e| match e {
            Elt::Value(v) if v >= 10 => true,
            _ => false,
        })
    }

    /// Find the next number to the left of the pair at `n`, if any.
    fn find_left(&self, n: Node) -> Option<Node> {
        let t = self.subtree(n);
        (1..t.start)
            .rev()
            .find(|&n| matches!(self.elts[n], Elt::Value(_)))
    }

    /// Find the next number to the right of the pair at `n`, if any.
    fn find_right(&self, n: Node) -> Option<Node> {
        let t = self.subtree(n);
        (t.end + 1..N).find(|&n| matches!(self.elts[n], Elt::Value(_)))
    }

    fn change_bottom_pair_to_value(&mut self, pair: Node, v: Value) {
        assert!(pair & 0b11 == 0b10); // bottom-level pairs only
        let (left, right) = self.children(pair);
        self.elts[left] = Elt::Empty;
        self.elts[right] = Elt::Empty;
        self.elts[pair] = Elt::Value(v);
    }
}

impl<const N: usize> fmt::Display for Basis<N> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let r = NodeRef {
            basis: self,
            node: Self::ROOT,
        };

        r.fmt(f)
    }
}

/// A `Basis` and `Node` wrapped up together for formatting.
struct NodeRef<'a, const N: usize> {
    basis: &'a Basis<N>,
    node: Node,
}

impl<'a, const N: usize> NodeRef<'a, N> {
    fn is_pair(&self) -> bool {
        self.basis.is_pair(self.node)
    }

    fn children(&self) -> (Self, Self) {
        let (left, right) = self.basis.children(self.node);
        (self.at(left), self.at(right))
    }

    fn get(&self) -> Value {
        self.basis.get(self.node)
    }

    fn at(&self, node: Node) -> Self {
        NodeRef {
            basis: self.basis,
            node,
        }
    }
}

impl<'a, const N: usize> fmt::Display for NodeRef<'a, N> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.is_pair() {
            let (left, right) = self.children();
            write!(f, "[{},{}]", left, right)
        } else {
            write!(f, "{}", self.get())
        }
    }
}

/// Parse `input` as a snailfish number, and store it at `pos` in `basis`.
fn parse<const N: usize>(input: &str, basis: &mut Basis<N>) -> Result<()> {
    fn recur<'i, 'b, const N: usize>(
        input: &'i str,
        basis: &'b mut Basis<N>,
        pos: Node,
    ) -> Result<&'i str> {
        if input.starts_with("[") {
            let (left, right) = basis.set_pair(pos);
            let rest = recur(&input[1..], basis, left)?;
            let rest = match rest.split_once(",") {
                Some(("", rest)) => rest,
                _ => bail!("missing , in pair: {:?}, rest = {:?}", input, rest),
            };
            let rest = recur(rest, basis, right)?;
            match rest.split_once("]") {
                Some(("", rest)) => Ok(rest),
                _ => bail!("expected closing bracket: {:?}", rest),
            }
        } else {
            let (value, rest) = match input.find(|ch: char| !ch.is_digit(10)) {
                Some(0) => bail!("confusing start to input: {:?}", input),
                Some(end) => (Value::from_str(&input[..end])?, &input[end..]),
                None => (Value::from_str(input)?, ""),
            };

            basis.set_const(pos, value);
            Ok(rest)
        }
    }

    basis.clear();
    let rest = recur(input.trim(), basis, Basis::<N>::ROOT)?;
    if !rest.is_empty() {
        bail!("Garbage at end of input: {:?}", rest);
    }

    Ok(())
}

impl<const N: usize> Basis<N> {
    fn explode(&mut self) -> bool {
        if let Some(node) = self.find_too_deep() {
            let (left, right) = self.children(node);
            let left_value = self.get(left);
            let right_value = self.get(right);
            self.change_bottom_pair_to_value(node, 0);
            if let Some(left_into) = self.find_left(node) {
                *self.get_mut(left_into) += left_value;
            }
            if let Some(right_into) = self.find_right(node) {
                *self.get_mut(right_into) += right_value;
            }
            true
        } else {
            false
        }
    }
}

impl<const N: usize> Basis<N> {
    fn split(&mut self) -> bool {
        if let Some(node) = self.find_needs_split() {
            let value = self.get(node);
            let (left, right) = self.set_pair(node);
            self.set_const(left, value / 2);
            self.set_const(right, value - value / 2);
            true
        } else {
            false
        }
    }
}

impl<const N: usize> Basis<N> {
    fn reduce(&mut self) {
        while self.explode() || self.split() {}
    }
}

impl<const N: usize> Basis<N> {
    fn add(&mut self, other: &Self) {
        assert!((0..N / 2).all(|i| self.elts[2 * i + 1] == Elt::Empty));
        assert!((0..N / 2).all(|i| other.elts[2 * i + 1] == Elt::Empty));
        for i in 0..N / 2 {
            self.elts[i] = self.elts[2 * i];
        }
        for i in 0..N / 2 {
            self.elts[N / 2 + i] = other.elts[2 * i];
        }
        assert!(self.elts[N / 2] == Elt::Empty);
        self.elts[N / 2] = Elt::Pair;

        self.reduce();
    }
}

impl<const N: usize> Basis<N> {
    fn magnitude_from(&self, n: Node) -> usize {
        match self.elts[n] {
            Elt::Empty => panic!("malformed tree"),
            Elt::Pair => {
                let (left, right) = self.children(n);
                3 * self.magnitude_from(left) + 2 * self.magnitude_from(right)
            }
            Elt::Value(v) => v as usize,
        }
    }
    fn magnitude(&self) -> usize {
        self.magnitude_from(Self::ROOT)
    }
}

fn sum_list_heap<'i, I, T>(list: I) -> Result<AocBasis>
where
    I: IntoIterator<Item = &'i T> + 'i,
    T: AsRef<str> + 'i,
    T: ?Sized,
{
    let mut left = AocBasis::new();
    let mut right = AocBasis::new();
    let mut list = list.into_iter();
    let first = list.next().unwrap().as_ref();
    parse(first, &mut left)?;

    for next in list {
        parse(next.as_ref(), &mut right)?;
        left.add(&right);
    }

    Ok(left)
}

#[aoc_generator(day18, part1, jimb_heap)]
fn generator_heap(input: &str) -> Vec<String> {
    input.lines().map(|s| s.to_owned()).collect()
}

#[aoc(day18, part1, jimb_heap)]
fn part1_heap(input: &Vec<String>) -> usize {
    let sum = sum_list_heap(input).unwrap();
    sum.magnitude()
}

//============================================================================
// jorendorff
//============================================================================

#[derive(Clone)]
enum Number {
    Regular(i64),
    Pair(Box<Number>, Box<Number>),
}

impl std::fmt::Debug for Number {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Number::Regular(n) => write!(f, "{:?}", n),
            Number::Pair(left, right) => write!(f, "[{:?},{:?}]", *left, *right),
        }
    }
}

impl Number {
    fn magnitude(&self) -> i64 {
        match self {
            Number::Regular(n) => *n,
            Number::Pair(left, right) => 3 * left.magnitude() + 2 * right.magnitude(),
        }
    }

    fn reduce(&mut self) {
        self.try_explode(0);
        while self.split_once() {
            self.try_explode(0);
        }
    }

    // Ok if successfully exploded, Err(self) unchanged if nothing was nested
    // deeply enough to explode.
    fn try_explode(&mut self, depth: usize) -> (i64, i64) {
        match self {
            Number::Regular(_n) => (0, 0),
            Number::Pair(left, right) if depth == 4 => {
                let left = left.magnitude();
                let right = right.magnitude();
                *self = Number::Regular(0);
                (left, right)
            }
            Number::Pair(left, right) => {
                let (ll, lr) = left.try_explode(depth + 1);
                if lr != 0 {
                    right.add_to_leftmost(lr);
                }
                let (rl, rr) = right.try_explode(depth + 1);
                if rl != 0 {
                    left.add_to_rightmost(rl);
                }
                (ll, rr)
            }
        }
    }

    fn split_once(&mut self) -> bool {
        match self {
            Number::Regular(n) => {
                if *n >= 10 {
                    *self = Number::Pair(
                        Box::new(Number::Regular(*n / 2)),
                        Box::new(Number::Regular((*n + 1) / 2)),
                    );
                    true
                } else {
                    false
                }
            }
            Number::Pair(left, right) => left.split_once() || right.split_once(),
        }
    }

    fn add_to_leftmost(&mut self, n: i64) {
        match self {
            Self::Regular(m) => *m += n,
            Self::Pair(left, _right) => left.add_to_leftmost(n),
        }
    }

    fn add_to_rightmost(&mut self, n: i64) {
        match self {
            Self::Regular(m) => *m += n,
            Self::Pair(_left, right) => right.add_to_rightmost(n),
        }
    }

    fn add(self, other: Number) -> Self {
        let mut out = Number::Pair(Box::new(self), Box::new(other));
        out.reduce();
        out
    }
}

struct Parser<'a> {
    text: &'a str,
    point: usize,
}

impl<'a> Parser<'a> {
    fn looking_at(&self, s: &str) -> bool {
        self.text[self.point..].starts_with(s)
    }

    fn at_end(&self) -> bool {
        self.point == self.text.len()
    }

    fn parse_number(&mut self) -> anyhow::Result<Number> {
        anyhow::ensure!(!self.at_end());
        if self.looking_at("[") {
            self.point += 1;
            let lhs = Box::new(self.parse_number()?);
            anyhow::ensure!(self.looking_at(","));
            self.point += 1;
            let rhs = Box::new(self.parse_number()?);
            anyhow::ensure!(self.looking_at("]"));
            self.point += 1;
            Ok(Number::Pair(lhs, rhs))
        } else {
            let mut j = self.point;
            while let Some(next_ch) = self.text[j..].chars().next() {
                if !next_ch.is_ascii_digit() {
                    break;
                }
                j += 1;
            }
            let n = self.text[self.point..j].parse::<i64>()?;
            self.point = j;
            Ok(Number::Regular(n))
        }
    }
}

fn parse_number(s: &str) -> anyhow::Result<Number> {
    let mut parser = Parser { text: s, point: 0 };
    let num = parser.parse_number()?;
    anyhow::ensure!(parser.at_end());
    Ok(num)
}

#[aoc_generator(day18, part1, jorendorff)]
#[aoc_generator(day18, part2, jorendorff)]
fn parse_input(text: &str) -> anyhow::Result<Vec<Number>> {
    text.lines().map(parse_number).collect()
}

fn sum(nums: impl IntoIterator<Item = Number>) -> Number {
    let mut nums = nums.into_iter();
    let first = nums.next().unwrap();
    nums.fold(first, |acc, next| acc.add(next))
}

#[aoc(day18, part1, jorendorff)]
fn part_1(input: &Vec<Number>) -> i64 {
    sum(input.clone()).magnitude()
}

#[aoc(day18, part2, jorendorff)]
fn part_2(input: &Vec<Number>) -> i64 {
    (0..input.len())
        .flat_map(|i| {
            (0..input.len()).filter_map(move |j| {
                if i == j {
                    None
                } else {
                    Some(input[i].clone().add(input[j].clone()).magnitude())
                }
            })
        })
        .max()
        .unwrap()
}

//=======================================================================
// pmetzger
//=======================================================================

#[derive(Debug, Clone, PartialEq)]
enum PMElt {
    Open,
    Int(usize),
    Close,
}

type Num = Vec<PMElt>;

type Data = Vec<Num>;

#[allow(dead_code)]
fn dump_num(num: &Num) {
    for (i, elt) in num.iter().enumerate() {
        match elt {
            PMElt::Open => {print!("["); continue;},
            PMElt::Close => print!("]"),
            PMElt::Int(j) => print!("{}", j),
        }
        if let Some(t) = num.get(i+1) {
            match t {
                PMElt::Close => (),
                PMElt::Open | PMElt::Int(_) => print!(","),
            };
        };
    }
    println!("");
}

#[allow(dead_code)]
fn dump_data(d: &Data) {
    for num in d {
        dump_num(&num);
    }
}

fn concat(a: &Num, b: &Num) -> Num {
    let mut ret = Num::new();
    ret.push(PMElt::Open);
    ret.extend_from_slice(a);
    ret.extend_from_slice(b);
    ret.push(PMElt::Close);
    ret
}

fn unwrap_usize(x: &PMElt) -> usize {
    if let PMElt::Int(a) = x { *a } else { panic!("unexpected elt: {:?}", x) }
}

// Disgustibus non est disputandum.
// ...and this is disgusting.
fn do_explode(x: &mut Num, i: usize) {
    let a = unwrap_usize(&x[i]);
    let b = unwrap_usize(&x[i+1]);

    // find something slightly more efficient?
    x.remove(i+2);
    x.remove(i+1);
    x[i] = PMElt::Int(0);
    x.remove(i-1);
    // point at the correct spot
    let loc = i - 1;
    let mut i = loc - 1;
    loop {
        if i == 0 { break; }
        if let PMElt::Int(n) = x[i] {
            x[i] = PMElt::Int(n + a);
            break;
        }
        i -= 1;
    }
    let mut i = loc + 1;
    let len = x.len();
    loop {
        if i >= len { break; }
        if let PMElt::Int(n) = x[i] {
            x[i] = PMElt::Int(n + b);
            break;
        }
        i += 1;
    };
}

fn pmexplode(x: &mut Num) -> bool {
    let len = x.len();
    let mut level = 0;
    for i in 0..len {
        match x[i] {
            PMElt::Open => level += 1,
            PMElt::Close => level -= 1,
            PMElt::Int(_) => {
                if level == 5 {
                    if let Some(PMElt::Int(_)) = x.get(i+1) {
                        do_explode(x, i);
                        return true;
                    }
                }
            }
        }
    }
    return false;
}

fn pmsplit(x: &mut Num) -> bool {
    for i in 0..x.len() {
        if let PMElt::Int(n) = x[i] {
            if n > 9 {
                let left = n / 2;
                let right = n - left;
                // truly disgusting.
                x[i] = PMElt::Close;
                x.insert(i, PMElt::Int(right));
                x.insert(i, PMElt::Int(left));
                x.insert(i, PMElt::Open);
                return true;
            }
        }
    }
    return false;
}

fn pmreduce(x: &Num) -> Num {
    let mut x = x.clone();
    loop {
        if !pmexplode(&mut x) && !pmsplit(&mut x) {break;}
    };
    x
}

fn add(a: &Num, b: &Num) -> Num {
    pmreduce(&concat(&a, &b))
}

fn magnitude_pair(iter: &mut Peekable<std::slice::Iter<PMElt>>) -> usize {
    assert_eq!(*iter.next().unwrap(), PMElt::Open);
    let peek: &PMElt = *iter.peek().unwrap();
    let left =
        match peek {
            PMElt::Open => magnitude_pair(iter),
            PMElt::Int(i) => {let _ = iter.next(); *i},
            _ => panic!("unexpected elt: {:?}", peek),
        };
    let peek: &PMElt = *iter.peek().unwrap();
    let right =
        match peek {
            PMElt::Open => magnitude_pair(iter),
            PMElt::Int(i) => {let _ = iter.next(); *i},
            _ => panic!("unexpected elt: {:?}", peek),
        };
    assert_eq!(*iter.next().unwrap(), PMElt::Close);
    left * 3 + right * 2
}


fn pmmagnitude(x: &Num) -> usize {
    magnitude_pair(&mut x.iter().peekable())
}

fn max_sum_all_distinct_pairs(data: &Data) -> usize {
    let mut max: usize = 0;
    let len = data.len();
    for i in 0..len {
        for j in 0..len {
            if i != j {
                let mag1 = pmmagnitude(&add(&data[i], &data[j]));
                let mag2 = pmmagnitude(&add(&data[j], &data[i]));
                let larger = if mag1 > mag2 { mag1 } else { mag2 };
                max = if larger > max { larger } else { max };
            }
        }
    }
    max
}

#[aoc_generator(day18, part1, pmetzger)]
#[aoc_generator(day18, part2, pmetzger)]
fn parse_data(s: &str) -> Data {
    s.lines().map(|s| parse_line(s)).collect()
}

#[aoc(day18, part1, pmetzger)]
fn do_part1(data: &Data) -> usize {
    let mut iter = data.iter().cloned();
    let mut x = iter.next().unwrap();
    for y in iter {
        x = add(&x, &y);
    };
    pmmagnitude(&x)
}

#[aoc(day18, part2, pmetzger)]
fn do_part2(data: &Data) -> usize {
    max_sum_all_distinct_pairs(&data)
}

// utilities

// parser

fn parse_line(s: &str) -> Num {
    let mut ret = Num::new();
    for c in s.chars() {
        match c {
            '[' => ret.push(PMElt::Open),
            '0'..='9' => ret.push(PMElt::Int(c.to_digit(10).unwrap() as usize)),
            ']' => ret.push(PMElt::Close),
            ',' => (),
            _ => panic!("unexpected char: {:?}", c),
        }
    }
    ret
}

