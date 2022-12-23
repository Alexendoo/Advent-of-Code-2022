use regex::Regex;
use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Tile {
    Open,
    Wall,
}

//      (0, -1)
// (-1, 0)   (1, 0)
//      (0, 1)
const UP: (i32, i32) = (0, -1);
const RIGHT: (i32, i32) = (1, 0);
const DOWN: (i32, i32) = (0, 1);
const LEFT: (i32, i32) = (-1, 0);

type Board = HashMap<(i32, i32), Tile>;

fn solve<F>(board: &Board, path: &str, wrap: F) -> i32
where
    F: Fn(i32, i32, i32, i32) -> (i32, i32, i32, i32),
{
    let mut x = (0..)
        .find(|&x| board.get(&(x, 0)).copied() == Some(Tile::Open))
        .unwrap();
    let mut y = 0;

    let mut dx = 1;
    let mut dy = 0;

    for instruction in Regex::new(r"\d+|[RL]").unwrap().find_iter(path) {
        match instruction.as_str() {
            "R" => (dx, dy) = (-dy, dx),
            "L" => (dx, dy) = (dy, -dx),
            n => {
                for _ in 0..n.parse::<i32>().unwrap() {
                    match board.get(&(x + dx, y + dy)).copied() {
                        Some(Tile::Open) => {
                            x += dx;
                            y += dy;
                        }
                        Some(Tile::Wall) => {
                            break;
                        }
                        None => {
                            let (wrap_x, wrap_y, wrap_dx, wrap_dy) = wrap(x, y, dx, dy);

                            if board.get(&(wrap_x, wrap_y)).copied() == Some(Tile::Open) {
                                x = wrap_x;
                                y = wrap_y;
                                dx = wrap_dx;
                                dy = wrap_dy;
                            }
                        }
                    }
                }
            }
        }
    }

    let facing = match (dx, dy) {
        RIGHT => 0,
        DOWN => 1,
        LEFT => 2,
        UP => 3,
        _ => unreachable!(),
    };
    (y + 1) * 1000 + (x + 1) * 4 + facing
}

fn main() {
    let (board, path) = include_str!("input").trim_end().split_once("\n\n").unwrap();

    let board: HashMap<_, _> = board
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.char_indices().filter_map(move |(x, char)| match char {
                '.' => Some(((x as i32, y as i32), Tile::Open)),
                '#' => Some(((x as i32, y as i32), Tile::Wall)),
                _ => None,
            })
        })
        .collect();

    println!(
        "Part 1: {}",
        solve(&board, path, |x, y, dx, dy| {
            (1..)
                .map(|step| {
                    (
                        (x + dx * step).rem_euclid(150),
                        (y + dy * step).rem_euclid(200),
                        dx,
                        dy,
                    )
                })
                .find(|&(x, y, ..)| board.get(&(x, y)).is_some())
                .unwrap()
        })
    );

    println!(
        "Part 2: {}",
        solve(&board, path, |x, y, dx, dy| {
            let (face_x, face_y) = (x / 50, y / 50);

            let (x, y, (dx, dy)) = match (face_x, face_y, (dx, dy)) {
                (1, 0, LEFT) => (0, 149 - y, RIGHT),
                (0, 2, LEFT) => (50, 149 - y, RIGHT),

                (0, 3, DOWN) => (x + 100, 0, DOWN),
                (2, 0, UP) => (x - 100, 199, UP),

                (1, 2, DOWN) => (49, x + 100, LEFT),
                (0, 3, RIGHT) => (y - 100, 149, UP),

                (0, 2, UP) => (50, 50 + x, RIGHT),
                (1, 1, LEFT) => (y - 50, 100, DOWN),

                (1, 0, UP) => (0, x + 100, RIGHT),
                (0, 3, LEFT) => (y - 100, 0, DOWN),

                (1, 1, RIGHT) => (y + 50, 49, UP),
                (2, 0, DOWN) => (99, x - 50, LEFT),

                (2, 0, RIGHT) => (99, 149 - y, LEFT),
                (1, 2, RIGHT) => (149, 149 - y, LEFT),

                _ => unimplemented!(),
            };

            (x, y, dx, dy)
        })
    )
}
