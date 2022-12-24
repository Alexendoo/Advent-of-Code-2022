use pathfinding::prelude::{astar, Matrix};

#[derive(Clone, Copy, PartialEq, Eq)]
enum Tile {
    Up,
    Down,
    Left,
    Right,
    Empty,
}

impl std::fmt::Debug for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Tile::Up => '^',
                Tile::Down => 'v',
                Tile::Left => '<',
                Tile::Right => '>',
                Tile::Empty => '.',
            }
        )
    }
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
struct State {
    x: i32,
    y: i32,
    turn: i32,
}

impl State {
    fn moves(self, board: &Matrix<Tile>) -> impl Iterator<Item = (State, usize)> + '_ {
        let (x, y) = (self.x, self.y);
        let turn = self.turn + 1;

        [(x, y), (x + 1, y), (x - 1, y), (x, y + 1), (x, y - 1)]
            .into_iter()
            .filter(move |&(x, y)| (x, y) == (0, -1) || !occupied(board, turn, x, y))
            .map(move |(x, y)| (Self { x, y, turn }, 1))
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

    let state = State {
        x: 0,
        y: -1,
        turn: 0,
    };

    let (_, n) = astar(
        &state,
        |state| state.moves(&board),
        |state| 0,
        |state| (state.x + 1 == width as i32) && (state.y + 1 == height as i32),
    )
    .unwrap();

    println!("Part 1: {}", n + 1);
}
