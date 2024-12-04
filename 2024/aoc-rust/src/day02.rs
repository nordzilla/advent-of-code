use aoc_runner_derive::*;

type Input = Vec<Vec<i64>>;
type Output = usize;

#[aoc_generator(day2, part1, Original)]
#[aoc_generator(day2, part2, Original)]
fn input_generator(raw_input: &str) -> Input {
  raw_input
    .lines()
    .map(|line| {
      line
        .split_ascii_whitespace()
        .map(|num| num.parse().unwrap())
        .collect()
    })
    .collect()
}

fn is_safe_increasing(report: &[i64]) -> bool {
  report.windows(2).all(|window| {
    let diff = window[0] - window[1];
    diff < 0 && diff.abs() <= 3
  })
}

fn is_safe_increasing_without_index(n: usize, report: &mut Vec<i64>) -> bool {
  let value = report.remove(n);
  let is_safe = is_safe_increasing(report);
  report.insert(n, value);

  is_safe
}

fn is_safe_increasing_with_single_correction(report: &[i64]) -> bool {
  if is_safe_increasing(report) {
    return true;
  }

  let increasing_problem_points = report.windows(2).enumerate().filter(|(_, window)| {
    let diff = window[0] - window[1];
    diff >= 0 || diff.abs() > 3
  });

  let report = &mut Vec::from(report);

  for (n, _) in increasing_problem_points {
    if is_safe_increasing_without_index(n, report)
      || is_safe_increasing_without_index(n + 1, report)
    {
      return true;
    }
  }

  return false;
}

fn is_safe_decreasing(report: &[i64]) -> bool {
  report.windows(2).all(|window| {
    let diff = window[0] - window[1];
    diff > 0 && diff.abs() <= 3
  })
}

fn is_safe_decreasing_without_index(n: usize, report: &mut Vec<i64>) -> bool {
  let value = report.remove(n);
  let is_safe = is_safe_decreasing(report);
  report.insert(n, value);

  is_safe
}

fn is_safe_decreasing_with_single_correction(report: &[i64]) -> bool {
  if is_safe_decreasing(report) {
    return true;
  }

  let decreasing_problem_points = report.windows(2).enumerate().filter(|(_, window)| {
    let diff = window[0] - window[1];
    diff <= 0 || diff.abs() > 3
  });

  let report = &mut Vec::from(report);

  for (n, _) in decreasing_problem_points {
    if is_safe_decreasing_without_index(n, report)
      || is_safe_decreasing_without_index(n + 1, report)
    {
      return true;
    }
  }

  return false;
}

#[aoc(day2, part1, Original)]
fn solve_part1(reports: &Input) -> Output {
  reports
    .iter()
    .filter(|report| is_safe_increasing(report) || is_safe_decreasing(report))
    .count()
}

#[aoc(day2, part2, Original)]
fn solve_part2(reports: &Input) -> Output {
  reports
    .iter()
    .filter(|&report| {
      is_safe_increasing_with_single_correction(report)
        || is_safe_decreasing_with_single_correction(report)
    })
    .count()
}
