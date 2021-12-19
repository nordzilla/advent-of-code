use aoc_runner_derive::*;
use flow_control::return_if;
use hashbrown::{HashMap, HashSet};
use itertools::Itertools;
use text_io::scan;

type Input = Vec<Scanner>;
type Output = usize;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Scanner {
    data: HashSet<[i16; 3]>,
}

#[aoc_generator(day19)]
fn input_generator(raw_input: &str) -> Input {
    raw_input
        .split("\n\n")
        .map(|lines| Scanner {
            data: lines
                .lines()
                .skip(1)
                .map(|line| {
                    let [x, y, z]: [i16; 3];
                    scan!(line.bytes() => "{},{},{}", x, y, z);
                    [x, y, z]
                })
                .collect::<HashSet<_>>(),
        })
        .collect()
}

#[rustfmt::skip]
const TRANSFORMS: [fn(&[i16; 3]) -> [i16; 3]; 24] = [
    |&[x, y, z]| [ x, -y, -z],
    |&[x, y, z]| [ x,  z,  y],
    |&[x, y, z]| [ x,  z, -y],
    |&[x, y, z]| [ x, -z,  y],

    |&[x, y, z]| [-x,  y, -z],
    |&[x, y, z]| [-x, -y,  z],
    |&[x, y, z]| [-x,  z,  y],
    |&[x, y, z]| [-x, -z, -y],

    |&[x, y, z]| [ y,  x, -z],
    |&[x, y, z]| [ y, -x,  z],
    |&[x, y, z]| [ y,  z,  x],
    |&[x, y, z]| [ y, -z, -x],

    |&[x, y, z]| [-y,  x,  z],
    |&[x, y, z]| [-y, -x, -z],
    |&[x, y, z]| [-y,  z, -x],
    |&[x, y, z]| [-y, -z,  x],

    |&[x, y, z]| [ z,  x,  y],
    |&[x, y, z]| [ z, -x, -y],
    |&[x, y, z]| [ z,  y, -x],
    |&[x, y, z]| [ z, -y,  x],

    |&[x, y, z]| [-z,  x, -y],
    |&[x, y, z]| [-z, -x,  y],
    |&[x, y, z]| [-z,  y,  x],
    |&[x, y, z]| [-z, -y, -x],
];

/// Shifts the first-argument point by the second-argument distance.
fn shift([a1, b1, c1]: [i16; 3], [a2, b2, c2]: [i16; 3]) -> [i16; 3] {
    [a1 + a2, b1 + b2, c1 + c2]
}

/// Returns the manhattan distance between two points.
fn manhattan_distance([a1, b1, c1]: [i16; 3], [a2, b2, c2]: [i16; 3]) -> i16 {
    (a1 - a2).abs() + (b1 - b2).abs() + (c1 - c2).abs()
}

impl Scanner {
    /// Returns an iterator over the scanner's points in every possible orientation.
    fn all_orientations(&self) -> impl Iterator<Item = impl Iterator<Item = [i16; 3]> + '_> + '_ {
        TRANSFORMS
            .iter()
            .map(|transform| self.data.iter().map(transform))
    }

    /// Returns the distances from each beacon detected by the scanner to this given point.
    fn distances(&self, [x2, y2, z2]: [i16; 3]) -> impl Iterator<Item = [i16; 3]> + '_ {
        self.data
            .iter()
            .copied()
            .map(move |[x1, y1, z1]| [(x1 - x2), (y1 - y2), (z1 - z2)])
    }

    /// Returns the TRANSFORMS index that would align scanner to self's orientation,
    /// as well as the distance in that orientation that scanner would have to travel to get to self.
    ///
    /// The THRESHOLD refers to the count of distances among the groups of points that are found to be the same.
    /// If the count is at or above the THRESHOLD, then we found a matching orientation with enough shared points to
    /// determine the distance between the scanners in that orientation.
    fn distance_from<const THRESHOLD: usize>(&self, scanner: &Self) -> Option<(usize, [i16; 3])> {
        for (transform_idx, orientation) in scanner.all_orientations().enumerate() {
            let mut distances: HashMap<[i16; 3], usize> =
                HashMap::with_capacity(self.data.len().pow(2));
            for point in orientation {
                for distance in self.distances(point) {
                    *distances.entry(distance).or_default() += 1;
                }
            }
            let found = distances.into_iter().find_map(|(distance, count)| {
                (count >= THRESHOLD).then(|| (transform_idx, distance))
            });
            return_if!(found.is_some(), found);
        }
        None
    }

    /// Returns self as transformed by TRANSFORMS[transform_idx] and moved by distance
    /// in the new orientation.
    fn transmoved_by(&self, transform_idx: usize, distance: [i16; 3]) -> Self {
        Self {
            data: self
                .data
                .iter()
                .map(|point| shift(TRANSFORMS[transform_idx](point), distance))
                .collect(),
        }
    }

    /// Shifts scanner to self's perspective, if possible, and adds the coordinates
    /// of all of scanner's known beacon's to self's beacon set in self's perspective.
    ///
    /// Returns the number of scanner's beacons that were added to self's beacon set.
    fn gain_perspective<const THRESHOLD: usize>(&mut self, scanner: &Self) -> usize {
        let start_len = self.data.len();
        if let Some((transform_idx, distance)) = self.distance_from::<THRESHOLD>(scanner) {
            for point in scanner.transmoved_by(transform_idx, distance).data {
                self.data.insert(point);
            }
        }
        self.data.len() - start_len
    }
}

/// Merges all the scanners into one scanner that knows the location of every beacon
/// in a single orientation.
fn merge_all_perspectives(mut scanners: Input) -> Scanner {
    let mut scanner = scanners.swap_remove(0);
    while !scanners.is_empty() {
        for i in 0..scanners.len() {
            if scanner.gain_perspective::<12>(&scanners[i]) > 0 {
                scanners.swap_remove(i);
                break;
            }
        }
    }
    scanner
}

#[aoc(day19, part1)]
fn solve_part1(scanners: &Input) -> Output {
    let the_one_scanner_to_rule_them_all = merge_all_perspectives(scanners.clone());
    the_one_scanner_to_rule_them_all.data.len()
}

#[aoc(day19, part2)]
fn solve_part2(scanners: &Input) -> Output {
    let the_one_scanner_to_rule_them_all = merge_all_perspectives(scanners.clone());
    scanners
        .iter()
        .filter_map(|scanner| the_one_scanner_to_rule_them_all.distance_from::<12>(scanner))
        .map(|(_, location)| location)
        .combinations(2)
        .map(|combo| manhattan_distance(combo[0], combo[1]))
        .max()
        .unwrap() as usize
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn transforms_are_unique() {
        let set = std::iter::repeat([1, 2, 3])
            .zip(TRANSFORMS)
            .map(|(point, transform)| transform(&point))
            .collect::<HashSet<_>>();
        assert_eq!(set.len(), TRANSFORMS.len());
    }

    #[test]
    fn gain_perspective() {
        let scanner1 = Scanner {
            data: [[0, 2, 0], [4, 1, 0], [3, 3, 0]].into_iter().collect(),
        };
        let scanner2 = Scanner {
            data: [[-1, -1, 0], [-5, 0, 0], [-2, 1, 0]].into_iter().collect(),
        };

        let mut s1 = scanner1.clone();
        s1.gain_perspective::<3>(&scanner2);
        assert_eq!(s1, scanner1);

        let mut s2 = scanner2.clone();
        s2.gain_perspective::<3>(&scanner1);
        assert_eq!(s2, scanner2);
    }
}
