use aoc_runner_derive::*;
use flow_control::return_if;
use text_io::scan;

type Input = Vec<Cuboid>;
type Output = i64;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
struct Range(i64, i64);

impl Range {
    fn len(self) -> i64 {
        1 + (self.0 - self.1).abs()
    }

    fn intersection_with(self, other: Self) -> Option<Self> {
        return_if!(self.1 < other.0, None);
        return_if!(other.1 < self.0, None);
        return_if!(
            self.0 <= other.0,
            Some(Range(other.0, std::cmp::min(self.1, other.1))),
        );
        return_if!(
            other.0 <= self.0,
            Some(Range(self.0, std::cmp::min(self.1, other.1))),
        );
        None
    }
}

#[derive(Debug, Copy, Clone)]
struct CubeRange {
    xs: Range,
    ys: Range,
    zs: Range,
}

impl CubeRange {
    fn new([x1, x2, y1, y2, z1, z2]: [i64; 6]) -> Self {
        Self {
            xs: Range(x1, x2),
            ys: Range(y1, y2),
            zs: Range(z1, z2),
        }
    }

    fn volume(self) -> i64 {
        self.xs.len() * self.ys.len() * self.zs.len()
    }

    fn intersection_with(self, other: Self) -> Option<Self> {
        if let Some(xs) = self.xs.intersection_with(other.xs) {
            if let Some(ys) = self.ys.intersection_with(other.ys) {
                if let Some(zs) = self.zs.intersection_with(other.zs) {
                    return Some(Self { xs, ys, zs });
                }
            }
        }
        None
    }
}

#[derive(Debug, Copy, Clone)]
enum Cuboid {
    On(CubeRange),
    Off(CubeRange),
}

impl Cuboid {
    fn is_on(self) -> bool {
        matches!(self, Cuboid::On(_))
    }

    fn volume(self) -> i64 {
        match self {
            Cuboid::On(range) => range.volume(),
            Cuboid::Off(range) => -range.volume(),
        }
    }

    fn intersection_with(self, other: Self) -> Option<Self> {
        match (self, other) {
            (Cuboid::On(lhs), Cuboid::On(rhs)) => lhs.intersection_with(rhs).map(Cuboid::Off),
            (Cuboid::On(lhs), Cuboid::Off(rhs)) => lhs.intersection_with(rhs).map(Cuboid::Off),
            (Cuboid::Off(lhs), Cuboid::On(rhs)) => lhs.intersection_with(rhs).map(Cuboid::On),
            (Cuboid::Off(lhs), Cuboid::Off(rhs)) => lhs.intersection_with(rhs).map(Cuboid::On),
        }
    }
}

#[aoc_generator(day22, part1, nordzilla)]
fn input_generator1(raw_input: &str) -> Input {
    raw_input
        .lines()
        .filter_map(|line| {
            let onoff: String;
            let [x1, x2, y1, y2, z1, z2]: [i64; 6];
            scan!(line.bytes() => "{} x={}..{},y={}..{},z={}..{}", onoff, x1, x2, y1, y2, z1, z2);
            (-50 <= x1
                && x1 <= 50
                && -50 <= x2
                && x2 <= 50
                && -50 <= y1
                && y1 <= 50
                && -50 <= y2
                && y2 <= 50
                && -50 <= z1
                && z1 <= 50
                && -50 <= z2
                && z2 <= 50)
                .then(|| {
                    if "on" == onoff {
                        Cuboid::On(CubeRange::new([x1, x2, y1, y2, z1, z2]))
                    } else {
                        Cuboid::Off(CubeRange::new([x1, x2, y1, y2, z1, z2]))
                    }
                })
        })
        .collect()
}

#[aoc_generator(day22, part2, nordzilla)]
fn input_generator2(raw_input: &str) -> Input {
    raw_input
        .lines()
        .map(|line| {
            let onoff: String;
            let [x1, x2, y1, y2, z1, z2]: [i64; 6];
            scan!(line.bytes() => "{} x={}..{},y={}..{},z={}..{}", onoff, x1, x2, y1, y2, z1, z2);
            if "on" == onoff {
                Cuboid::On(CubeRange::new([x1, x2, y1, y2, z1, z2]))
            } else {
                Cuboid::Off(CubeRange::new([x1, x2, y1, y2, z1, z2]))
            }
        })
        .collect()
}

fn solve(instructions: &Input) -> i64 {
    let mut cuboids = Vec::with_capacity(instructions.len().pow(2));
    for &instruction in instructions {
        cuboids.extend(
            cuboids
                .clone()
                .iter()
                .filter_map(|&cuboid: &Cuboid| cuboid.intersection_with(instruction)),
        );
        if instruction.is_on() {
            cuboids.push(instruction);
        }
    }
    cuboids.into_iter().map(Cuboid::volume).sum::<i64>()
}

#[aoc(day22, part1, nordzilla)]
fn solve_part1(input: &Input) -> Output {
    solve(input)
}

#[aoc(day22, part2, nordzilla)]
fn solve_part2(input: &Input) -> Output {
    solve(input)
}
