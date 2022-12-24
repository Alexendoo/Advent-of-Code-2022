use pathfinding::prelude::{astar, Matrix};

#[derive(Clone, Copy, PartialEq, Eq)]
enum Tile {
    Up,
    Down,
    Left,
    Right,
    Empty,
}

fn occupied(board: &Matrix<Tile>, turn: i32, x: i32, y: i32) -> bool {
    if !(0..board.columns as i32).contains(&x) || !(0..board.rows as i32).contains(&y) {
        return true;
    }

    [
        (x + turn, y, Tile::Left),
        (x - turn, y, Tile::Right),
        (x, y + turn, Tile::Up),
        (x, y - turn, Tile::Down),
    ]
    .into_iter()
    .any(|(x, y, direction)| {
        board[(
            y.rem_euclid(board.rows as i32) as usize,
            x.rem_euclid(board.columns as i32) as usize,
        )] == direction
    })
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Trip {
    First,
    Back,
    Second,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct State {
    x: i32,
    y: i32,
    turn: i32,
    trip: Trip,
}

impl State {
    fn moves(self, board: &Matrix<Tile>) -> impl Iterator<Item = (State, u32)> + '_ {
        let Self { x, y, turn, trip } = self;
        let turn = turn + 1;

        let start = (0, -1);
        let end = (board.columns as i32 - 1, board.rows as i32);

        [(x, y), (x + 1, y), (x - 1, y), (x, y + 1), (x, y - 1)]
            .into_iter()
            .filter_map(move |(x, y)| {
                let mut trip = trip;
                if (x, y) == start {
                    if trip == Trip::Back {
                        trip = Trip::Second;
                    }
                } else if (x, y) == end {
                    if trip == Trip::First {
                        trip = Trip::Back;
                    }
                } else if occupied(board, turn, x, y) {
                    return None;
                };

                Some((Self { x, y, turn, trip }, 1))
            })
    }

    fn heuristic(self, board: &Matrix<Tile>) -> u32 {
        let Self { x, y, .. } = self;
        let (start_x, start_y) = (0i32, -1i32);
        let (end_x, end_y) = (board.columns as i32 - 1, board.rows as i32);

        let start_to_end = start_x.abs_diff(end_x) + start_y.abs_diff(end_y);

        match self.trip {
            Trip::First => x.abs_diff(end_x) + y.abs_diff(end_y) + start_to_end * 2,
            Trip::Back => x.abs_diff(start_x) + y.abs_diff(start_y) + start_to_end,
            Trip::Second => x.abs_diff(end_x) + y.abs_diff(end_y),
        }
    }
}

fn main() {
    let input = include_str!("input");

    let width = input.lines().next().unwrap().len() - 2;
    let height = input.lines().count() - 2;

    let mut board = Matrix::new(height, width, Tile::Empty);

    for (row, line) in input.lines().skip(1).enumerate() {
        for (column, ch) in line[1..].char_indices() {
            board[(row, column)] = match ch {
                '^' => Tile::Up,
                'v' => Tile::Down,
                '<' => Tile::Left,
                '>' => Tile::Right,
                _ => continue,
            };
        }
    }

    let solve = |trip| {
        astar(
            &State {
                x: 0,
                y: -1,
                turn: 0,
                trip,
            },
            |state| state.moves(&board),
            |state| state.heuristic(&board),
            |&State { x, y, trip, .. }| {
                (x + 1 == width as i32) && (y == height as i32) && trip == Trip::Second
            },
        )
        .unwrap()
        .1
    };

    println!("Part 1: {}", solve(Trip::Second));
    println!("Part 2: {}", solve(Trip::First));
}
