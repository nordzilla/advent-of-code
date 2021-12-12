use aoc_runner_derive::*;
use flow_control::return_if;
use std::collections::BTreeMap;
use text_io::scan;

type Input = BTreeMap<Cave, Vec<Cave>>;
type Output = usize;

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum Cave {
    Start,
    Large(u16),
    Small(u16),
    End,
}

impl Cave {
    fn id(self) -> u16 {
        match self {
            Cave::Large(id) | Cave::Small(id) => id,
            _ => 0,
        }
    }

    fn matches_id(self, rhs: u16) -> bool {
        match self {
            Cave::Large(id) | Cave::Small(id) => id == rhs,
            _ => false,
        }
    }
}

impl From<String> for Cave {
    fn from(s: String) -> Self {
        if s == "start" {
            Cave::Start
        } else if s == "end" {
            Cave::End
        } else {
            let value = (0..)
                .map(|place_value| 1 << place_value)
                .zip(s.bytes())
                .map(|(place_value, byte)| place_value * byte as u16)
                .product();
            if s.chars().next().unwrap().is_uppercase() {
                Cave::Large(value)
            } else {
                Cave::Small(value)
            }
        }
    }
}

fn get_count(
    key: Cave,
    id: u16,
    map: &Input,
    visited: &mut Vec<Cave>,
    solved: &mut BTreeMap<(Cave, u16, u16), usize>,
) -> usize {
    let check_sum = visited.iter().copied().map(Cave::id).sum();
    return_if!(
        solved.contains_key(&(key, check_sum, id)),
        solved[&(key, check_sum, id)]
    );
    let count = map[&key]
        .iter()
        .map(|&cave| {
            return_if!(visited.contains(&cave), 0);
            let count = match (key, cave) {
                (_, Cave::Start) => 1,
                (Cave::End | Cave::Large(_), rhs) => get_count(rhs, id, map, visited, solved),
                (lhs @ Cave::Small(_), rhs) => {
                    return_if!(lhs.matches_id(id), get_count(rhs, 0, map, visited, solved));
                    visited.push(lhs);
                    let count = get_count(rhs, id, map, visited, solved);
                    visited.pop();
                    count
                }
                _ => 0,
            };
            count
        })
        .sum();
    solved.insert((key, check_sum, id), count);
    count
}

#[aoc_generator(day12)]
fn input_generator(raw_input: &str) -> Input {
    raw_input.lines().fold(BTreeMap::new(), |mut map, line| {
        let [key, value]: [String; 2];
        scan!(line.bytes() => "{}-{}", key, value);
        let (key, value): (Cave, Cave) = (key.into(), value.into());
        if !matches!(key, Cave::Start) && !matches!(value, Cave::End) {
            map.entry(key)
                .and_modify(|caves| caves.push(value))
                .or_insert(vec![value]);
        }
        if !matches!(value, Cave::Start) && !matches!(key, Cave::End) {
            map.entry(value)
                .and_modify(|caves| caves.push(key))
                .or_insert(vec![key]);
        }
        map
    })
}

#[aoc(day12, part1)]
fn solve_part1(input: &Input) -> Output {
    let mut visited = Vec::new();
    let mut solved = BTreeMap::new();
    get_count(Cave::End, 0, input, &mut visited, &mut solved)
}

#[aoc(day12, part2)]
fn solve_part2(input: &Input) -> Output {
    let mut visited = Vec::new();
    let mut solved = BTreeMap::new();
    let part1 = get_count(Cave::End, 0, input, &mut visited, &mut solved);
    part1
        + input
            .keys()
            .filter(|cave| matches!(cave, Cave::Small(_)))
            .map(|&cave| get_count(Cave::End, cave.id(), input, &mut visited, &mut solved) - part1)
            .sum::<usize>()
}
