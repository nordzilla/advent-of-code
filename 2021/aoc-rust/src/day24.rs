use aoc_runner_derive::*;
use flow_control::return_if;
use text_io::scan;

type Input = Vec<Instruction>;
type Output = i64;

#[derive(Debug, Copy, Clone)]
enum Var {
    W,
    X,
    Y,
    Z,
    Lit(i64),
}

#[derive(Debug, Copy, Clone)]
enum Instruction {
    Inp(Var),
    Add(Var, Var),
    Mul(Var, Var),
    Div(Var, Var),
    Mod(Var, Var),
    Eql(Var, Var),
}

#[aoc_generator(day24, part1, nordzilla)]
#[aoc_generator(day24, part2, nordzilla)]
fn input_generator(raw_input: &str) -> Input {
    raw_input
        .lines()
        .map(|line| {
            if line.starts_with("inp") {
                let lhs: char;
                scan!(line.bytes() => "inp {}", lhs);
                let lhs = match lhs {
                    'w' => Var::W,
                    'x' => Var::X,
                    'y' => Var::Y,
                    'z' => Var::Z,
                    _ => panic!("at the disco!"),
                };
                Instruction::Inp(lhs)
            } else {
                let lhs: char;
                let rhs: String;
                scan!(line[4..].bytes() => "{} {}", lhs, rhs);
                let lhs = match lhs {
                    'w' => Var::W,
                    'x' => Var::X,
                    'y' => Var::Y,
                    'z' => Var::Z,
                    _ => panic!("at the disco!"),
                };
                let rhs = match rhs.as_str() {
                    "w" => Var::W,
                    "x" => Var::X,
                    "y" => Var::Y,
                    "z" => Var::Z,
                    n => Var::Lit(n.parse::<i64>().unwrap()),
                };
                match &line[0..3] {
                    "add" => Instruction::Add(lhs, rhs),
                    "mul" => Instruction::Mul(lhs, rhs),
                    "div" => Instruction::Div(lhs, rhs),
                    "mod" => Instruction::Mod(lhs, rhs),
                    "eql" => Instruction::Eql(lhs, rhs),
                    _ => panic!("at the disco!"),
                }
            }
        })
        .collect()
}

/// Returns the count of digits in a number, or 0 if the number itself is 0.
fn digit_count(n: i64) -> u32 {
    return_if!(n == 0, 0);
    1 + (n as f64).log10() as u32
}

/// Returns an iterator over the digits of a number from highest place value to lowest place value.
fn digits_iter(n: i64) -> impl Iterator<Item = i64> {
    (0..digit_count(n))
        .rev()
        .map(move |exp| n / 10_i64.pow(exp) % 10)
}

/// Retuns an iterator of integers that do not contain 0s, from 11111111111111 to 99999999999999
fn increasing_segment_candidates(digits: u32) -> impl Iterator<Item = i64> {
    std::iter::successors(Some(11111111111111 % 10_i64.pow(digits)), move |n| {
        return_if!(n + 1 > 99999999999999 % 10_i64.pow(digits), None);
        Some(
            n + 1
                + (1..=digits)
                    .map(|place_value| {
                        if (n + 1) % 10_i64.pow(place_value) < 10_i64.pow(place_value - 1) {
                            10_i64.pow(place_value - 1)
                        } else {
                            0
                        }
                    })
                    .sum::<i64>(),
        )
    })
}

/// Retuns an iterator of integers that do not contain 0s, from 99999999999999 to 11111111111111
fn decreasing_segment_candidates(digits: u32) -> impl Iterator<Item = i64> {
    std::iter::successors(Some(99999999999999 % 10_i64.pow(digits)), move |n| {
        return_if!(n - 1 < 11111111111111 % 10_i64.pow(digits), None);
        Some(
            n - 1
                - (1..=digits)
                    .map(|place_value| {
                        if (n - 1) % 10_i64.pow(place_value) < 10_i64.pow(place_value - 1) {
                            10_i64.pow(place_value - 1)
                        } else {
                            0
                        }
                    })
                    .sum::<i64>(),
        )
    })
}

/// Returns the concatenation of number with segment.
/// e.g. concat_segment(123, 456) == 123456
fn concat_segment(number: i64, segment: i64) -> i64 {
    number * 10_i64.pow(digit_count(segment)) + segment
}

#[derive(Debug, Default, Clone, Copy)]
struct Monad<'a> {
    w: i64,
    x: i64,
    y: i64,
    z: i64,
    instructions: &'a [Instruction],
}

impl<'a> Monad<'a> {
    fn new(instructions: &'a [Instruction]) -> Self {
        Self {
            w: 0,
            x: 0,
            y: 0,
            z: 0,
            instructions,
        }
    }

    fn var(&mut self, var: Var) -> Option<&mut i64> {
        match var {
            Var::W => Some(&mut self.w),
            Var::X => Some(&mut self.x),
            Var::Y => Some(&mut self.y),
            Var::Z => Some(&mut self.z),
            Var::Lit(_) => None,
        }
    }

    fn value(self, var: Var) -> i64 {
        match var {
            Var::W => self.w,
            Var::X => self.x,
            Var::Y => self.y,
            Var::Z => self.z,
            Var::Lit(n) => n,
        }
    }

    fn inp(var: &mut i64, digits: &mut impl Iterator<Item = i64>) -> Option<()> {
        digits.next().map(|digit| *var = digit)
    }

    fn eval(
        &mut self,
        instruction: Instruction,
        digits: &mut impl Iterator<Item = i64>,
    ) -> Option<()> {
        match instruction {
            Instruction::Inp(v) => Monad::inp(self.var(v)?, digits)?,
            Instruction::Add(lhs, rhs) => *self.var(lhs)? += self.value(rhs),
            Instruction::Mul(lhs, rhs) => *self.var(lhs)? *= self.value(rhs),
            Instruction::Div(lhs, rhs) => {
                return_if!(self.value(rhs) == 0, None);
                *self.var(lhs)? /= self.value(rhs)
            }
            Instruction::Mod(lhs, rhs) => {
                return_if!(self.value(lhs) < 0, None);
                return_if!(self.value(rhs) <= 0, None);
                *self.var(lhs)? %= self.value(rhs)
            }
            Instruction::Eql(lhs, rhs) => {
                *self.var(lhs)? = (self.value(lhs) == self.value(rhs)) as i64
            }
        }
        Some(())
    }

    fn validate_candidate(mut self, candidate: i64, critical_instruction: usize) -> Option<i64> {
        let mut digits = digits_iter(candidate);
        for &instruction in self.instructions.iter().take(critical_instruction + 1) {
            self.eval(instruction, &mut digits)?;
        }
        (self.x == 0).then(|| candidate)
    }

    /// A critical section is an instruction where we need the x-register to be 0.
    /// Returns (digit, instruction) where digit is the digit number being evaluated
    /// during the critical section, and instruction is the nth that should
    /// leave the x-register with 0.
    fn find_critical_sections(self) -> Vec<(u32, usize)> {
        self.instructions
            .iter()
            .zip(3..)
            .filter(|(instruction, _)| {
                matches!(instruction, Instruction::Div(Var::Z, Var::Lit(26)))
            })
            .map(|(_, n)| (n / 18 + 1, n as usize))
            .collect()
    }

    fn increasing_valid_segment_candidates(
        self,
        partial_solution: i64,
        (critical_digit, critical_instruction): (u32, usize),
    ) -> impl Iterator<Item = i64> + 'a {
        increasing_segment_candidates(critical_digit - digit_count(partial_solution))
            .into_iter()
            .filter(move |&segment_candidate| {
                let solution_candidate = concat_segment(partial_solution, segment_candidate);
                self.validate_candidate(solution_candidate, critical_instruction)
                    .is_some()
            })
    }

    fn decreasing_valid_segment_candidates(
        self,
        partial_solution: i64,
        (critical_digit, critical_instruction): (u32, usize),
    ) -> impl Iterator<Item = i64> + 'a {
        decreasing_segment_candidates(critical_digit - digit_count(partial_solution))
            .into_iter()
            .filter(move |&segment_candidate| {
                let solution_candidate = concat_segment(partial_solution, segment_candidate);
                self.validate_candidate(solution_candidate, critical_instruction)
                    .is_some()
            })
    }

    fn find_smallest_valid_number(
        self,
        solution: i64,
        critical_sections: &[(u32, usize)],
    ) -> Option<i64> {
        return_if!(critical_sections.is_empty(), Some(solution));
        self.increasing_valid_segment_candidates(solution, critical_sections[0])
            .find_map(|segment_candidate| {
                self.find_smallest_valid_number(
                    concat_segment(solution, segment_candidate),
                    &critical_sections[1..],
                )
            })
    }

    fn find_largest_valid_number(
        self,
        solution: i64,
        critical_sections: &[(u32, usize)],
    ) -> Option<i64> {
        return_if!(critical_sections.is_empty(), Some(solution));
        self.decreasing_valid_segment_candidates(solution, critical_sections[0])
            .find_map(|segment_candidate| {
                self.find_largest_valid_number(
                    concat_segment(solution, segment_candidate),
                    &critical_sections[1..],
                )
            })
    }

    fn smallest_valid_number(self) -> Option<i64> {
        self.find_smallest_valid_number(0, &self.find_critical_sections())
    }

    fn largest_valid_number(self) -> Option<i64> {
        self.find_largest_valid_number(0, &self.find_critical_sections())
    }
}

#[aoc(day24, part1, nordzilla)]
fn solve_part1(instructions: &Input) -> Output {
    Monad::new(instructions).largest_valid_number().unwrap()
}

#[aoc(day24, part2, nordzilla)]
fn solve_part2(instructions: &Input) -> Output {
    Monad::new(instructions).smallest_valid_number().unwrap()
}
