use aoc_runner_derive::*;
use text_io::scan;

type Input = (Vec<u64>, Vec<u64>);
type Output = u64;

#[aoc_generator(day1, part1, Original)]
#[aoc_generator(day1, part2, Original)]
fn input_generator(raw_input: &str) -> Input {
  let (mut list1, mut list2) = raw_input
    .lines()
    .map(|line| {
      let value1;
      let value2;
      scan!(line.bytes() => "{} {}", value1, value2);
      (value1, value2)
    })
    .fold(
      (Vec::new(), Vec::new()),
      |(mut list1, mut list2), (value1, value2)| {
        list1.push(value1);
        list2.push(value2);
        (list1, list2)
      },
    );

  list1.sort();
  list2.sort();

  (list1, list2)
}

#[aoc(day1, part1, Original)]
fn solve_part1((list1, list2): &Input) -> Output {
  list1
    .iter()
    .zip(list2)
    .map(|(&value1, &value2)| value1.abs_diff(value2))
    .sum()
}

#[aoc(day1, part2, Original)]
fn solve_part2((list1, list2): &Input) -> Output {
  list1
    .iter()
    .map(|value1| value1 * list2.iter().filter(|&value2| value1 == value2).count() as u64)
    .sum()
}
