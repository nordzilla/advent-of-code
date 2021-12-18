use aoc_runner_derive::*;
use flow_control::return_if;
use itertools::Itertools;

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

#[aoc_generator(day18)]
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

#[aoc(day18, part1)]
fn solve_part1(input: &Input) -> Output {
    input
        .iter()
        .cloned()
        .reduce(|lhs, rhs| lhs.add(rhs))
        .unwrap()
        .magnitude()
}

#[aoc(day18, part2)]
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
