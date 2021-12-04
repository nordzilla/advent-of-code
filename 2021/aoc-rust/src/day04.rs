use aoc_runner_derive::*;
use itertools::Itertools;

type Input = (Vec<i64>, Vec<BingoBoard>);
type Output = i64;

#[derive(Debug, Clone, Copy)]
struct Squre {
    value: i64,
    is_marked: bool,
}

impl From<i64> for Squre {
    fn from(value: i64) -> Self {
        Self {
            value,
            is_marked: false,
        }
    }
}

impl Squre {
    fn mark_if_matches(&mut self, n: i64) {
        self.value.eq(&n).then(|| self.is_marked = true);
    }

    fn unmarked_value(&self) -> Option<i64> {
        (!self.is_marked).then(|| self.value)
    }
}

#[derive(Debug, Default, Clone)]
struct BingoBoard {
    board: Vec<Vec<Squre>>,
}

impl From<Vec<Vec<Squre>>> for BingoBoard {
    fn from(board: Vec<Vec<Squre>>) -> Self {
        Self { board }
    }
}

impl BingoBoard {
    fn marked_with(&mut self, n: i64) -> &mut Self {
        self.board
            .iter_mut()
            .for_each(|row| row.iter_mut().for_each(|square| square.mark_if_matches(n)));
        self
    }

    fn is_winner(&self) -> bool {
        (0..5).all(|diag| self.board[diag][diag].is_marked)
            | (0..5).all(|diag| self.board[diag][4 - diag].is_marked)
            | (0..5).any(|row| (0..5).all(|col| self.board[row][col].is_marked))
            | (0..5).any(|col| (0..5).all(|row| self.board[row][col].is_marked))
    }

    fn sum_unmarked_squares(&self) -> i64 {
        self.board
            .iter()
            .flat_map(|row| row.iter().filter_map(|square| square.unmarked_value()))
            .sum()
    }
}

#[aoc_generator(day4)]
fn input_generator(raw_input: &str) -> Input {
    let mut lines = raw_input.lines().filter(|line| !line.trim().is_empty());
    let calls = lines
        .next()
        .unwrap()
        .split(',')
        .map(|n| n.parse().unwrap())
        .collect();
    let boards = lines
        .chunks(5)
        .into_iter()
        .map(|chunk| {
            chunk
                .into_iter()
                .map(|line| {
                    line.split_whitespace()
                        .map(|n| n.parse::<i64>().unwrap().into())
                        .collect::<Vec<_>>()
                })
                .collect::<Vec<_>>()
                .into()
        })
        .collect();
    (calls, boards)
}

fn play_bingo((calls, boards): &Input) -> Vec<(i64, BingoBoard)> {
    let mut boards = boards.clone();
    let mut winners = Vec::with_capacity(boards.len());
    calls.iter().for_each(|&n| {
        winners.extend(
            boards
                .drain_filter(|board| board.marked_with(n).is_winner())
                .map(|board| (n, board)),
        );
    });
    winners
}

#[aoc(day4, part1)]
fn solve_part1(input: &Input) -> Output {
    play_bingo(input)
        .first()
        .map(|(winning_call, board)| winning_call * board.sum_unmarked_squares())
        .unwrap()
}

#[aoc(day4, part2)]
fn solve_part2(input: &Input) -> Output {
    play_bingo(input)
        .last()
        .map(|(winning_call, board)| winning_call * board.sum_unmarked_squares())
        .unwrap()
}
