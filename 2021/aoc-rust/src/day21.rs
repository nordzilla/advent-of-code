use aoc_runner_derive::*;
use flow_control::return_if;
use text_io::scan;

type Input = (usize, usize);
type Output = usize;

#[aoc_generator(day21, part1, nordzilla)]
#[aoc_generator(day21, part2, nordzilla)]
fn input_generator(raw_input: &str) -> (usize, usize) {
    let mut lines = raw_input.lines();
    let line1 = lines.next().unwrap();
    let line2 = lines.next().unwrap();
    let [pos1, pos2]: [usize; 2];
    scan!(line1.bytes() => "Player 1 starting position: {}", pos1);
    scan!(line2.bytes() => "Player 2 starting position: {}", pos2);
    (pos1 - 1, pos2 - 1)
}

type State = [usize; 3];
fn next([score, position, roll_count]: State) -> State {
    let position = (position + (roll_count + 1..roll_count + 4).sum::<usize>()) % 10;
    [score + position + 1, position, roll_count + 3]
}

fn play(mut pos1: usize, mut pos2: usize) -> usize {
    let mut roll_count = 0;
    let [mut score1, mut score2] = [0, 0];
    while score1 < 1000 && score2 < 1000 {
        if 0 == roll_count % 2 {
            let [score, position, count] = next([score1, pos1, roll_count]);
            score1 = score;
            pos1 = position;
            roll_count = count;
        } else {
            let [score, position, count] = next([score2, pos2, roll_count]);
            score2 = score;
            pos2 = position;
            roll_count = count;
        }
    }
    std::cmp::min(score1, score2) * roll_count
}

// Pascal's triangle summed by columns:
//
// |+|+|+|+|+|+|+|
// | | | |1| | | |
// | | |3| |3| | |
// | |3| |6| |3| |
// |1| |3| |3| |1|
// |=|=|=|=|=|=|=|
// |1|3|6|7|6|3|1|
//
//=================
//
// Roll sum universe counts:
//
// 1,1,1 = 3
//
// 1,1,2 = 4
// 1,2,1 = 4
// 2,1,1 = 4
//
// 1,1,3 = 5
// 1,2,2 = 5
// 1,3,1 = 5
// 2,1,2 = 5
// 2,2,1 = 5
// 3,1,1 = 5
//
// 1,2,3 = 6
// 1,3,2 = 6
// 2,1,3 = 6
// 2,2,2 = 6
// 2,3,1 = 6
// 3,1,2 = 6
// 3,2,1 = 6
//
// 1,3,3 = 7
// 2,2,3 = 7
// 2,3,2 = 7
// 3,1,3 = 7
// 3,2,2 = 7
// 3,3,1 = 7
//
// 2,3,3 = 8
// 3,3,2 = 8
// 3,2,3 = 8
//
// 3,3,3 = 9
//
//             (value, count)
const ROLLS: &[(usize, usize)] = &[(3, 1), (4, 3), (5, 6), (6, 7), (7, 6), (8, 3), (9, 1)];

type MultiState = [usize; 5];
fn multi_next([score1, score2, pos1, pos2, turn]: MultiState, value: usize) -> MultiState {
    if 0 == turn % 2 {
        [
            1 + score1 + (pos1 + value) % 10,
            score2,
            (pos1 + value) % 10,
            pos2,
            turn + 1,
        ]
    } else {
        [
            score1,
            1 + score2 + (pos2 + value) % 10,
            pos1,
            (pos2 + value) % 10,
            turn + 1,
        ]
    }
}

fn multi_play(state @ [score1, score2, _, _, _]: MultiState) -> (usize, usize) {
    return_if!(score1 >= 21, (1, 0));
    return_if!(score2 >= 21, (0, 1));
    ROLLS
        .into_iter()
        .map(|&(value, count)| {
            let (wins1, wins2) = multi_play(multi_next(state, value));
            (count * wins1, count * wins2)
        })
        .fold((0, 0), |(wins1, wins2), (p1, p2)| (wins1 + p1, wins2 + p2))
}

#[aoc(day21, part1, nordzilla)]
fn solve_part1(&(pos1, pos2): &Input) -> Output {
    play(pos1, pos2)
}

#[aoc(day21, part2, nordzilla)]
fn solve_part2(&(pos1, pos2): &Input) -> Output {
    let (wins1, wins2) = multi_play([0, 0, pos1, pos2, 0]);
    std::cmp::max(wins1, wins2)
}
