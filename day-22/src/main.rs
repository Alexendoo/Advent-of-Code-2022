use std::collections::HashMap;

use regex::Regex;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Tile {
    Open,
    Wall,
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

    let x_max = board.keys().map(|&(x, _)| x).max().unwrap() + 1;
    let y_max = board.keys().map(|&(_, y)| y).max().unwrap() + 1;

    let mut x = (0..)
        .find(|&x| board.get(&(x, 0)).copied() == Some(Tile::Open))
        .unwrap();
    let mut y = 0;

    //      (0, -1)
    // (-1, 0)   (1, 0)
    //      (0, 1)
    let mut dx = 1;
    let mut dy = 0;

    for instruction in Regex::new(r"\d+|[RL]").unwrap().find_iter(path) {
        // println!("INS #{i}: {}", instruction.as_str());

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
                            for step in 1.. {
                                let wrapped_x = (x + dx * step).rem_euclid(x_max);
                                let wrapped_y = (y + dy * step).rem_euclid(y_max);

                                match board.get(&(wrapped_x, wrapped_y)) {
                                    Some(Tile::Open) => {
                                        x = wrapped_x;
                                        y = wrapped_y;
                                        break;
                                    }
                                    Some(Tile::Wall) => {
                                        break;
                                    }
                                    None => {}
                                }
                            }
                        }
                    }
                }
            }
        }

        // print(&board, (x, y), (x_max, y_max), (dx, dy));
    }

    let facing = match (dx, dy) {
        (1, 0) => 0,
        (0, 1) => 1,
        (-1, 0) => 2,
        (0, -1) => 3,
        _ => unreachable!(),
    };
    println!("{}", (y + 1) * 1000 + (x + 1) * 4 + facing);
}

// fn print(
//     board: &HashMap<(i32, i32), Tile>,
//     pos: (i32, i32),
//     max: (i32, i32),
//     direction: (i32, i32),
// ) {
//     let mut out = String::new();

//     for y in 0..max.1 {
//         for x in 0..max.0 {
//             let tile = match board.get(&(x, y)).copied() {
//                 _ if (x, y) == pos => match direction {
//                     (1, 0) => '>',
//                     (0, -1) => '^',
//                     (-1, 0) => '<',
//                     (0, 1) => 'v',
//                     _ => unreachable!(),
//                 },
//                 Some(Tile::Open) => '.',
//                 Some(Tile::Wall) => '#',
//                 None => ' ',
//             };

//             out.push(tile);
//         }

//         out.push('\n');
//     }

//     println!("{out}");
// }
