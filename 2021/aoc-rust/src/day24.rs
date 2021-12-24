use std::str::FromStr;

use aoc_runner_derive::*;
use flow_control::return_if;
use rayon::prelude::*;

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

impl FromStr for Var {
    type Err = <i64 as FromStr>::Err;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "w" => Var::W,
            "x" => Var::X,
            "y" => Var::Y,
            "z" => Var::Z,
            n => Var::Lit(n.parse::<i64>()?),
        })
    }
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

impl FromStr for Instruction {
    type Err = <Var as FromStr>::Err;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match &s[0..3] {
            "inp" => Instruction::Inp((&s[4..5]).parse::<Var>()?),
            "add" => Instruction::Add((&s[4..5]).parse::<Var>()?, (&s[6..]).parse::<Var>()?),
            "mul" => Instruction::Mul((&s[4..5]).parse::<Var>()?, (&s[6..]).parse::<Var>()?),
            "div" => Instruction::Div((&s[4..5]).parse::<Var>()?, (&s[6..]).parse::<Var>()?),
            "mod" => Instruction::Mod((&s[4..5]).parse::<Var>()?, (&s[6..]).parse::<Var>()?),
            "eql" => Instruction::Eql((&s[4..5]).parse::<Var>()?, (&s[6..]).parse::<Var>()?),
            _ => panic!("at the disco!"),
        })
    }
}

#[aoc_generator(day24, part1, nordzilla)]
#[aoc_generator(day24, part2, nordzilla)]
fn input_generator(raw_input: &str) -> Input {
    raw_input
        .lines()
        .flat_map(Instruction::from_str)
        .filter(|instruction| !matches!(instruction, Instruction::Div(_, Var::Lit(1))))
        .collect()
}

#[derive(Debug, Default, Clone, Copy)]
struct Monad<'a> {
    w: i64,
    x: i64,
    y: i64,
    z: i64,
    counter: usize,
    instructions: &'a [Instruction],
}

//====================================================================================
// ALU operations
//====================================================================================
impl<'a> Monad<'a> {
    fn new(instructions: &'a [Instruction]) -> Self {
        Self {
            instructions,
            ..Self::default()
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
            Instruction::Div(lhs, rhs) => *self.var(lhs)? /= self.value(rhs),
            Instruction::Mod(lhs, rhs) => *self.var(lhs)? %= self.value(rhs),
            Instruction::Eql(lhs, rhs) => {
                *self.var(lhs)? = (self.value(lhs) == self.value(rhs)) as i64
            }
        }
        Some(())
    }
}

//====================================================================================
// Numeric manipulation helper functions
//====================================================================================

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

/// Retuns a parallel iterator of all numbers with the specified number of `digits` that do not contain 0
/// in any digit. The numbers will start with all 1's and end with all 9's, for the given digit length.
fn increasing_segment_candidates(digits: u32) -> impl ParallelIterator<Item = i64> {
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
    .collect::<Vec<_>>()
    .into_par_iter()
}

/// Retuns a parallel iterator of all numbers with the specified number of `digits` that do not contain 0
/// in any digit. The numbers will start with all 9's and end with all 1's, for the given digit length.
fn decreasing_segment_candidates(digits: u32) -> impl ParallelIterator<Item = i64> {
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
    .collect::<Vec<_>>()
    .into_par_iter()
}

/// Returns the concatenation of two numbers.
/// e.g. concat_segments(123, 456) == 123456
fn concat_segments(lhs: i64, rhs: i64) -> i64 {
    lhs * 10_i64.pow(digit_count(rhs)) + rhs
}

//====================================================================================
// Depth-first search for a valid number
//====================================================================================
impl<'a> Monad<'a> {
    /// A critical instruction is an instruction where we need the x-register to be 0.
    /// The critical instruction is `eql x 0`, but only when it follows `div z 26`.
    ///
    /// The critical instruction is always three instructions after `div z 26`.
    ///
    /// A number will only be valid if the x-register is zero after every critical instruction.
    fn find_critical_instructions(self) -> Vec<usize> {
        self.instructions
            .iter()
            .zip(4..)
            .filter(|(instruction, _)| {
                matches!(instruction, Instruction::Div(Var::Z, Var::Lit(26)))
            })
            .map(|(_, n)| n)
            .chain(std::iter::once(self.instructions.len()))
            .collect()
    }

    /// A solution segment is valid only if the x-register is zero after evaluating
    /// the critical instruction with regard to this segment.
    ///
    /// Returns the segment along with the Monad itself, whose register values and
    /// instruction counter are saved to the spot after evaluating the critical
    /// instruction for the segment.
    fn validate_solution_segment(
        mut self,
        segment_candidate: i64,
        critical_instruction: usize,
    ) -> Option<(Self, i64)> {
        let mut digits = digits_iter(segment_candidate);
        for &instruction in self
            .instructions
            .iter()
            .skip(self.counter)
            .take(critical_instruction - self.counter)
        {
            self.eval(instruction, &mut digits)?;
        }
        self.counter += critical_instruction - self.counter;
        (self.x == 0).then(|| (self, segment_candidate))
    }

    /// Return all segments that pass the test at their critical instruction.
    fn valid_segment_candidates<Iter: ParallelIterator<Item = i64> + 'static>(
        self,
        generate_candidates: fn(u32) -> Iter,
        critical_instruction: usize,
    ) -> impl ParallelIterator<Item = (Monad<'a>, i64)> + 'a {
        let digit_count = if self.counter == 0 {
            1 + critical_instruction / 18
        } else {
            critical_instruction / 18 - self.counter / 18
        } as u32;
        generate_candidates(digit_count).filter_map(move |segment_candidate| {
            self.validate_solution_segment(segment_candidate, critical_instruction)
        })
    }

    /// Recursive helper to find the first valid number.
    fn find_first_valid_number<Iter: ParallelIterator<Item = i64> + 'static>(
        self,
        previous_segment: i64,
        generate_candidates: fn(u32) -> Iter,
        critical_instructions: &[usize],
    ) -> Option<i64> {
        return_if!(critical_instructions.is_empty(), Some(0));
        self.valid_segment_candidates(generate_candidates, critical_instructions[0])
            .find_map_first(|(monad, segment_candidate)| {
                monad
                    .find_first_valid_number(
                        segment_candidate,
                        generate_candidates,
                        &critical_instructions[1..],
                    )
                    .map(|next_segment| concat_segments(previous_segment, next_segment))
            })
    }

    fn first_valid_number<Iter: ParallelIterator<Item = i64> + 'static>(
        self,
        generate_candidates: fn(u32) -> Iter,
    ) -> Option<i64> {
        self.find_first_valid_number(0, generate_candidates, &self.find_critical_instructions())
    }
}

#[aoc(day24, part1, nordzilla)]
fn solve_part1(instructions: &Input) -> Output {
    Monad::new(instructions)
        .first_valid_number(decreasing_segment_candidates)
        .unwrap()
}

#[aoc(day24, part2, nordzilla)]
fn solve_part2(instructions: &Input) -> Output {
    Monad::new(instructions)
        .first_valid_number(increasing_segment_candidates)
        .unwrap()
}
