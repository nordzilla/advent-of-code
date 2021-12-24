use aoc_runner_derive::*;
use flow_control::return_if;
use rayon::prelude::*;
use text_io::scan;

type Input1 = [Room<2>; 4];
type Input2 = [Room<4>; 4];
type Output = usize;

#[derive(Debug, Clone, Copy)]
struct Room<const N: usize> {
    x_index: usize,
    occupants: [u8; N],
}

#[aoc_generator(day23 part1, nordzilla)]
fn input_generator1(raw_input: &str) -> Input1 {
    let mut lines = raw_input.lines();
    lines.next();
    lines.next();

    let [a20, a40, a60, a80]: [char; 4];
    let [a21, a41, a61, a81]: [char; 4];
    scan!(lines.next().unwrap().bytes() => "###{}#{}#{}#{}###", a20, a40, a60, a80);
    scan!(lines.next().unwrap().bytes() => "  #{}#{}#{}#{}#", a21, a41, a61, a81);

    let room2 = Room::<2>::new(2, [a20 as u8, a21 as u8]);
    let room4 = Room::<2>::new(4, [a40 as u8, a41 as u8]);
    let room6 = Room::<2>::new(6, [a60 as u8, a61 as u8]);
    let room8 = Room::<2>::new(8, [a80 as u8, a81 as u8]);

    [room2, room4, room6, room8]
}

#[aoc_generator(day23 part2, nordzilla)]
fn input_generator2(raw_input: &str) -> Input2 {
    let mut lines = raw_input.lines();
    lines.next();
    lines.next();

    let [a20, a40, a60, a80]: [char; 4];
    let [a23, a43, a63, a83]: [char; 4];
    scan!(lines.next().unwrap().bytes() => "###{}#{}#{}#{}###", a20, a40, a60, a80);
    scan!(lines.next().unwrap().bytes() => "  #{}#{}#{}#{}#", a23, a43, a63, a83);

    let room2 = Room::<4>::new(2, [a20 as u8, b'D', b'D', a23 as u8]);
    let room4 = Room::<4>::new(4, [a40 as u8, b'C', b'B', a43 as u8]);
    let room6 = Room::<4>::new(6, [a60 as u8, b'B', b'A', a63 as u8]);
    let room8 = Room::<4>::new(8, [a80 as u8, b'A', b'C', a83 as u8]);

    [room2, room4, room6, room8]
}

impl<const N: usize> Room<N> {
    fn new(x_index: usize, occupants: [u8; N]) -> Self {
        Self { x_index, occupants }
    }

    fn matching_amphipod(self) -> u8 {
        match self.x_index {
            2 => b'A',
            4 => b'B',
            6 => b'C',
            _ => b'D',
        }
    }

    fn is_solved(self) -> bool {
        self.occupants
            .into_iter()
            .all(|amphipod| amphipod == self.matching_amphipod())
    }

    fn is_partially_solved(self) -> bool {
        self.occupants
            .into_iter()
            .filter(|&byte| byte != 0)
            .all(|amphipod| amphipod == self.matching_amphipod())
    }

    fn top_unsolved(self) -> Option<(usize, usize, u8)> {
        return_if!(self.is_partially_solved(), None);
        self.occupants
            .into_iter()
            .enumerate()
            .find(|&(_, byte)| byte != 0)
            .map(|(y_index, byte)| (self.x_index, y_index + 1, byte))
    }

    fn next_empty_spot(self) -> Option<(usize, usize)> {
        self.occupants
            .into_iter()
            .enumerate()
            .rev()
            .find(|&(_, byte)| byte == 0)
            .map(|(y_index, _)| (self.x_index, y_index + 1))
    }
}

const fn target_room(amphipod: u8) -> usize {
    (amphipod - b'A') as usize
}

const fn distance(lhs: usize, rhs: usize) -> isize {
    (lhs as isize - rhs as isize).abs()
}

const fn move_weight(amphipod: u8) -> usize {
    10_usize.pow((amphipod - b'A') as u32)
}

const fn move_cost(amphipod: u8, [start_x, start_y, target_x, target_y]: [usize; 4]) -> usize {
    move_weight(amphipod)
        * (distance(start_y, 0) + distance(start_x, target_x) + distance(0, target_y)) as usize
}

#[derive(Debug, Default, Copy, Clone)]
struct Hall([u8; 11]);

impl Hall {
    const fn stoppable_indices() -> [usize; 7] {
        [0, 1, 3, 5, 7, 9, 10]
    }

    fn amphipods(self) -> impl Iterator<Item = (usize, usize, u8)> {
        self.0
            .into_iter()
            .enumerate()
            .map(|(x_index, byte)| (x_index, 0, byte))
            .filter(|&(_, _, byte)| byte != 0)
    }

    fn move_is_valid(self, [start_x, _, target_x, _]: [usize; 4]) -> bool {
        let range = if start_x < target_x {
            start_x + 1..target_x + 1
        } else {
            target_x..start_x
        };
        self.0[range].iter().all(|&spot| spot == 0)
    }
}

#[derive(Debug, Clone, Copy)]
struct GameState<const N: usize> {
    hall: Hall,
    rooms: [Room<N>; 4],
    cost: usize,
}

impl<const N: usize> GameState<N> {
    fn new(rooms: [Room<N>; 4]) -> Self {
        Self {
            rooms,
            hall: Hall::default(),
            cost: 0,
        }
    }

    fn is_solved(&self) -> bool {
        self.rooms.into_iter().all(Room::is_solved)
    }

    fn room_at(&mut self, x_index: usize) -> &mut Room<N> {
        &mut self.rooms[(x_index - 2) / 2]
    }

    fn outbound_amphipods(&self) -> impl Iterator<Item = (usize, usize, u8)> {
        self.rooms.into_iter().flat_map(|room| room.top_unsolved())
    }

    fn outbound_moves(&self) -> impl ParallelIterator<Item = [usize; 4]> {
        self.outbound_amphipods()
            .flat_map(move |(start_x, start_y, amphipod)| {
                Hall::stoppable_indices()
                    .into_iter()
                    .filter(move |&target_x| self.hall.0[target_x] == 0)
                    .map(move |target_x| (amphipod, [start_x, start_y, target_x, 0]))
            })
            .map(move |(_, move_coords)| move_coords)
            .filter(move |&coords| self.hall.move_is_valid(coords))
            .collect::<Vec<_>>()
            .into_par_iter()
    }

    fn roombound_amphipods(&self) -> impl Iterator<Item = (usize, usize, u8)> {
        self.hall
            .amphipods()
            .chain(self.rooms.into_iter().flat_map(|room| room.top_unsolved()))
    }

    fn roombound_move(
        &self,
        (start_x, start_y, amphipod): (usize, usize, u8),
    ) -> Option<[usize; 4]> {
        let target_room = self.rooms[target_room(amphipod)];
        return_if!(!target_room.is_partially_solved(), None);
        target_room
            .next_empty_spot()
            .or(Some((target_room.x_index, 1)))
            .map(|(target_x, target_y)| [start_x, start_y, target_x, target_y])
            .filter(move |&coords| self.hall.move_is_valid(coords))
    }

    fn next_roombound_move(&self) -> Option<[usize; 4]> {
        self.roombound_amphipods()
            .flat_map(move |amph_state| self.roombound_move(amph_state))
            .next()
    }

    fn move_from_room(
        mut self,
        coords @ [start_x, start_y, target_x, target_y]: [usize; 4],
    ) -> Self {
        let amphipod = self.room_at(start_x).occupants[start_y - 1];
        self.room_at(start_x).occupants[start_y - 1] = 0;
        if target_y == 0 {
            self.hall.0[target_x] = amphipod;
        } else {
            self.room_at(target_x).occupants[target_y - 1] = amphipod;
        }
        self.cost += move_cost(amphipod, coords);
        self
    }

    fn move_from_hall(mut self, coords @ [start_x, _, target_x, target_y]: [usize; 4]) -> Self {
        let amphipod = self.hall.0[start_x];
        self.hall.0[start_x] = 0;
        self.room_at(target_x).occupants[target_y - 1] = amphipod;
        self.cost += move_cost(amphipod, coords);
        self
    }

    fn make_move(self, coords @ [_, start_y, _, _]: [usize; 4]) -> Self {
        if start_y == 0 {
            self.move_from_hall(coords)
        } else {
            self.move_from_room(coords)
        }
    }

    fn solve(mut self) -> Option<usize> {
        while let Some(coords) = self.next_roombound_move() {
            self = self.make_move(coords);
        }
        return_if!(self.is_solved(), Some(self.cost));
        self.outbound_moves()
            .filter_map(|coords| self.make_move(coords).solve())
            .min()
    }
}

#[aoc(day23, part1, nordzilla)]
fn solve_part1(rooms: &Input1) -> Output {
    GameState::new(*rooms).solve().unwrap()
}

#[aoc(day23, part2, nordzilla)]
fn solve_part2(rooms: &Input2) -> Output {
    GameState::new(*rooms).solve().unwrap()
}
