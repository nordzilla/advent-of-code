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

fn get_count(key: Cave, id: u16, map: &Input, visited: &mut Vec<Cave>) -> usize {
    map[&key]
        .iter()
        .map(|&cave| {
            return_if!(visited.contains(&cave), 0);
            let count = match (key, cave) {
                (_, Cave::Start) => 1,
                (Cave::End | Cave::Large(_), rhs) => get_count(rhs, id, map, visited),
                (lhs @ Cave::Small(_), rhs) => {
                    return_if!(lhs.matches_id(id), get_count(rhs, 0, map, visited));
                    visited.push(lhs);
                    let count = get_count(rhs, id, map, visited);
                    visited.pop();
                    count
                }
                _ => 0,
            };
            count
        })
        .sum()
}

fn get_count_memoized(
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
                (Cave::End | Cave::Large(_), rhs) => {
                    get_count_memoized(rhs, id, map, visited, solved)
                }
                (lhs @ Cave::Small(_), rhs) => {
                    return_if!(
                        lhs.matches_id(id),
                        get_count_memoized(rhs, 0, map, visited, solved)
                    );
                    visited.push(lhs);
                    let count = get_count_memoized(rhs, id, map, visited, solved);
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

#[aoc_generator(day12, part1, nordzilla)]
#[aoc_generator(day12, part2, nordzilla)]
#[aoc_generator(day12, part1, nordzillaMemoized)]
#[aoc_generator(day12, part2, nordzillaMemoized)]
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

#[aoc(day12, part1, nordzillaMemoized)]
fn solve_part1_memoized(input: &Input) -> Output {
    let mut visited = Vec::new();
    let mut solved = BTreeMap::new();
    get_count_memoized(Cave::End, 0, input, &mut visited, &mut solved)
}

#[aoc(day12, part2, nordzillaMemoized)]
fn solve_part2_memoized(input: &Input) -> Output {
    let mut visited = Vec::new();
    let mut solved = BTreeMap::new();
    let part1 = get_count_memoized(Cave::End, 0, input, &mut visited, &mut solved);
    part1
        + input
            .keys()
            .filter(|cave| matches!(cave, Cave::Small(_)))
            .map(|&cave| {
                get_count_memoized(Cave::End, cave.id(), input, &mut visited, &mut solved) - part1
            })
            .sum::<usize>()
}

#[aoc(day12, part1, nordzilla)]
fn solve_part1(input: &Input) -> Output {
    let mut visited = Vec::new();
    get_count(Cave::End, 0, input, &mut visited)
}

#[aoc(day12, part2, nordzilla)]
fn solve_part2(input: &Input) -> Output {
    let mut visited = Vec::new();
    let part1 = get_count(Cave::End, 0, input, &mut visited);
    part1
        + input
            .keys()
            .filter(|cave| matches!(cave, Cave::Small(_)))
            .map(|&cave| get_count(Cave::End, cave.id(), input, &mut visited) - part1)
            .sum::<usize>()
}

use std::collections::HashMap;

const START: usize = 0;
const END: usize = 1;

struct Room {
    name: String,
    large: bool,
    adj: Vec<usize>,
}

#[aoc_generator(day12, part1, jorendorff)]
#[aoc_generator(day12, part2, jorendorff)]
fn parse_input(text: &str) -> anyhow::Result<Vec<Room>> {
    let mut names: HashMap<String, usize> = HashMap::new();
    let mut rooms: Vec<Room> = vec![];

    let name_to_id =
        |names: &mut HashMap<String, usize>, rooms: &mut Vec<Room>, name: &str| -> usize {
            *names.entry(name.to_string()).or_insert_with(|| {
                let n = rooms.len();
                rooms.push(Room {
                    name: name.to_string(),
                    large: name.to_uppercase() == name,
                    adj: vec![],
                });
                n
            })
        };

    assert_eq!(name_to_id(&mut names, &mut rooms, "start"), START);
    assert_eq!(name_to_id(&mut names, &mut rooms, "end"), END);

    for line in text.lines() {
        let bits = line.split('-').collect::<Vec<&str>>();
        anyhow::ensure!(bits.len() == 2);
        let origin = name_to_id(&mut names, &mut rooms, bits[0]);
        let dest = name_to_id(&mut names, &mut rooms, bits[1]);
        rooms[origin].adj.push(dest);
        rooms[dest].adj.push(origin);
    }

    Ok(rooms)
}

fn solve(rooms: &[Room], can_revisit: bool) -> u64 {
    let mut visited = vec![0; rooms.len()];
    visited[START] = 1;
    let mut any_small_visited_multi = false;
    let mut count = 0;
    let mut breadcrumbs = vec![(START, 0)];
    while let Some((i, j)) = breadcrumbs.pop() {
        if j == rooms[i].adj.len() {
            visited[i] -= 1;
            if !rooms[i].large && visited[i] == 1 {
                any_small_visited_multi = false;
            }
        } else {
            breadcrumbs.push((i, j + 1));
            let next = rooms[i].adj[j];
            if next == END {
                count += 1;
            } else if rooms[next].large
                || visited[next] == 0
                || (can_revisit && next != START && !any_small_visited_multi && visited[next] == 1)
            {
                visited[next] += 1;
                breadcrumbs.push((next, 0));
                if !rooms[next].large && visited[next] == 2 {
                    any_small_visited_multi = true;
                }
            }
        }
    }
    count
}

#[aoc(day12, part1, jorendorff)]
fn part_1(rooms: &[Room]) -> u64 {
    solve(rooms, false)
}

#[aoc(day12, part2, jorendorff)]
fn part_2(rooms: &[Room]) -> u64 {
    solve(rooms, true)
}

use aoc_runner_derive::{aoc, aoc_generator};
use anyhow::{anyhow, Result};

#[derive(Debug, Eq, PartialEq)]
struct Node {
    name: String,
    big: bool,
    out: Vec<usize>,
}

impl Node {
    fn new(name: &str) -> Node {
        Node {
            name: name.to_string(),
            big: name.starts_with(char::is_uppercase),
            out: vec![]
        }
    }
}

/// Indexed by node id.
type Graph = Vec<Node>;

#[aoc_generator(day12, part1, jimblandy)]
#[aoc_generator(day12, part2, jimblandy)]
fn generate(input: &str) -> Result<Graph> {
    let mut graph = vec![Node::new("start"), Node::new("end")];
    let mut numbers = HashMap::new();
    numbers.insert("start", 0);
    numbers.insert("end", 1);

    for line in input.lines() {
        let (start, end) = line.split_once("-")
            .ok_or_else(|| anyhow!("missing '-' separator: {:?}", line))?;

        // Not needed, for the actual inputs.
        let start = start.trim();
        let end = end.trim();

        let start_ix = *numbers.entry(start).or_insert_with(|| {
            graph.push(Node::new(start));
            graph.len() - 1
        });

        let end_ix = *numbers.entry(end).or_insert_with(|| {
            graph.push(Node::new(end));
            graph.len() - 1
        });

        graph[start_ix].out.push(end_ix);
        graph[end_ix].out.push(start_ix);
    }

    Ok(graph)
}

#[cfg(test)]
fn small_sample() -> Graph {
    generate(include_str!("sample/day12.small"))
        .expect("failed to parse day12.small")
}

#[cfg(test)]
fn bigger_sample() -> Graph {
    generate(include_str!("sample/day12.bigger"))
        .expect("failed to parse day12.bigger")
}

#[cfg(test)]
fn even_larger_sample() -> Graph {
    generate(include_str!("sample/day12.even-larger"))
        .expect("failed to parse day12.even-larger")
}

#[test]
fn test_generate() {
    assert_eq!(small_sample(),
               vec![
                   Node { name: "start".to_string(), big: false, out: vec![2, 3] },
                   Node { name: "end".to_string(), big: false, out: vec![2, 3] },
                   Node { name: "A".to_string(), big: true, out: vec![0, 4, 3, 1] },
                   Node { name: "b".to_string(), big: false, out: vec![0, 2, 5, 1] },
                   Node { name: "c".to_string(), big: false, out: vec![2] },
                   Node { name: "d".to_string(), big: false, out: vec![3] },
               ]);
}

fn count_from(graph: &Graph, start: usize, visited: u64) -> usize {
    let visited = visited | (1 << start);
    if start == END {
        return 1;
    }

    graph[start].out
        .iter()
        .filter(|&&out| graph[out].big || visited & (1 << out) == 0)
        .map(|&out| count_from(graph, out, visited))
        .sum()
}

#[aoc(day12, part1, jimblandy)]
fn part1(input: &Graph) -> usize {
    count_from(input, START, 0)
}

#[test]
fn test_part1() {
    assert_eq!(part1(&small_sample()), 10);
    assert_eq!(part1(&bigger_sample()), 19);
    assert_eq!(part1(&even_larger_sample()), 226);
}

fn count_from2(graph: &Graph, start: usize, twice: Option<usize>, visited: u64) -> usize {
    //println!("{:>depth$}{}", "", graph[start].name, depth = 4 * visited.count_ones() as usize);

    let visited = visited | (1 << start);
    if start == END {
        return 1;
    }

    let mut count = 0;
    for &out in &graph[start].out {
        let out_bit = 1 << out;
        if graph[out].big || visited & out_bit == 0 {
            count += count_from2(graph, out, twice, visited);
        } else if twice.is_none() && out != START {
            count += count_from2(graph, out, Some(out), visited);
        }
    }
    count
}

#[aoc(day12, part2, jimblandy)]
fn part2(input: &Graph) -> usize {
    count_from2(input, START, None, 0)
}

#[test]
fn test_part2() {
    assert_eq!(part2(&small_sample()), 36);
    assert_eq!(part2(&bigger_sample()), 103);
    assert_eq!(part2(&even_larger_sample()), 3509);
}
