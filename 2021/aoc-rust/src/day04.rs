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
    fn mark(&mut self, n: i64) {
        self.board
            .iter_mut()
            .for_each(|row| row.iter_mut().for_each(|square| square.mark_if_matches(n)))
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
                        .into()
                })
                .collect::<Vec<_>>()
                .into()
        })
        .collect();
    (calls, boards)
}

#[aoc(day4, part1)]
fn solve_part1((calls, boards): &Input) -> Output {
    let mut boards = boards.clone();
    let mut first_winning_call = 0;
    let mut first_winning_board = BingoBoard::default();
    for &n in calls {
        boards.iter_mut().for_each(|board| board.mark(n));
        if let Some(winner) = boards.iter().find(|board| board.is_winner()) {
            first_winning_call = n;
            first_winning_board = winner.clone();
            break;
        }
    }
    first_winning_board.sum_unmarked_squares() * first_winning_call
}

#[aoc(day4, part2)]
fn solve_part2((calls, boards): &Input) -> Output {
    let mut boards = boards.clone();
    let mut final_winning_call = 0;
    let mut final_winning_board = BingoBoard::default();
    calls.iter().for_each(|&n| {
        boards.iter_mut().for_each(|board| {
            board.mark(n);
            board.is_winner().then(|| {
                final_winning_call = n;
                final_winning_board = board.clone();
            });
        });
        boards.retain(|board| !board.is_winner());
    });
    final_winning_board.sum_unmarked_squares() * final_winning_call
}
