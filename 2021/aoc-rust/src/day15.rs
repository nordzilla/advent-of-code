use aoc_runner_derive::*;
type Input = Vec<Vec<Vertex>>;
type Output = u32;
use std::collections::BTreeSet;

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
struct Vertex {
    location: (usize, usize),
    adj_cost: u32,
    min_cost: u32,
}

impl Vertex {
    fn new((x, y): (usize, usize), byte: u8) -> Self {
        Self {
            location: (x, y),
            adj_cost: byte as u32,
            min_cost: u32::MAX - 10,
        }
    }

    fn is_known(self) -> bool {
        self.min_cost < u32::MAX - 10
    }
}

fn adjacent_vertices(graph: &Input, (x, y): (usize, usize)) -> impl Iterator<Item = Vertex> {
    [
        graph.get(y).and_then(|row| row.get(x + 1)).copied(),
        graph.get(y).and_then(|row| row.get(x - 1)).copied(),
        graph.get(y + 1).and_then(|row| row.get(x)).copied(),
        graph.get(y - 1).and_then(|row| row.get(x)).copied(),
    ]
    .into_iter()
    .flatten()
}

#[aoc_generator(day15, part1, nordzilla)]
fn input_generator1(raw_input: &str) -> Input {
    raw_input
        .lines()
        .enumerate()
        .map(|(y, line)| {
            line.bytes()
                .enumerate()
                .map(|(x, byte)| Vertex::new((x, y), byte - b'0'))
                .collect()
        })
        .collect()
}

#[aoc_generator(day15, part2, nordzilla)]
fn input_generator2(raw_input: &str) -> Input {
    let mut vec = raw_input
        .lines()
        .map(|line| line.bytes().map(|byte| byte - b'0').collect::<Vec<_>>())
        .collect::<Vec<_>>();
    for row in &mut vec {
        let r = row.clone();
        for n in 1..5 {
            row.extend(r.iter().copied().map(|byte| {
                if byte + n > 9 {
                    (byte + n) % 10 + 1
                } else {
                    byte + n
                }
            }));
        }
    }
    let vec = std::iter::repeat(vec.clone())
        .zip(0..5)
        .flat_map(|(mut vec, n)| {
            for row in &mut vec {
                row.iter_mut().for_each(|byte| {
                    *byte = if *byte + n > 9 {
                        (*byte + n) % 10 + 1
                    } else {
                        *byte + n
                    }
                });
            }
            vec
        })
        .collect::<Vec<_>>();
    vec.into_iter()
        .enumerate()
        .map(|(y, line)| {
            line.into_iter()
                .enumerate()
                .map(|(x, byte)| Vertex::new((x, y), byte))
                .collect()
        })
        .collect()
}

fn do_the_dijkstra(graph: &mut Input) -> &Input {
    let mut vertex_set = BTreeSet::new();
    graph[0][0].adj_cost = u32::MAX;
    graph[0][0].min_cost = 0;
    vertex_set.insert(graph[0][0]);
    while !vertex_set.is_empty() {
        let vertex = vertex_set.iter().next().copied().unwrap();
        vertex_set.remove(&vertex);
        for v in adjacent_vertices(graph, vertex.location) {
            if !v.is_known() || v.is_known() && (vertex.min_cost + v.adj_cost) < v.min_cost {
                let (x, y) = v.location;
                graph[y][x].min_cost = vertex.min_cost + v.adj_cost;
                vertex_set.insert(graph[y][x]);
            }
        }
    }
    graph
}

#[aoc(day15, part1, nordzilla)]
fn solve_part1(input: &Input) -> Output {
    let mut input = input.clone();
    do_the_dijkstra(&mut input);
    input[input.len() - 1][input[0].len() - 1].min_cost
}

#[aoc(day15, part2, nordzilla)]
fn solve_part2(input: &Input) -> Output {
    solve_part1(input)
}
