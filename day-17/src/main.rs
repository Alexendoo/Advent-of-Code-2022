type Board = [[bool; 7]; 2022 * 3];

fn tower_height(board: &Board) -> i32 {
    board.partition_point(|row| row.contains(&true)) as i32
}

fn fall(board: &mut Board, rock: &[(i32, i32)], mut jets: impl Iterator<Item = i32>) {
    let mut offset_x = 2;
    let mut offset_y = tower_height(board) + 3;

    // print(board, rock, offset_x, offset_y);

    loop {

        let direction = jets.next().unwrap();

        let valid = rock.iter().all(|&(x, y)| {
            let x = x + offset_x + direction;
            let y = y + offset_y;

            x >= 0 && x < 7 && !board[y as usize][x as usize]
        });

        if valid {
            offset_x += direction;
        }


        let valid = rock.iter().all(|&(x, y)| {
            let x = x + offset_x;
            let y = y + offset_y - 1;

            y >= 0 && !board[y as usize][x as usize]
        });

        if valid {
            offset_y -= 1;
        } else {
            for (x, y) in rock {
                let x = x + offset_x;
                let y = y + offset_y;

                debug_assert!(!board[y as usize][x as usize]);
                board[y as usize][x as usize] = true;
            }

            break;
        }
    }
}

fn main() {
    let mut jets = include_str!("input")
        .trim()
        .bytes()
        .map(|byte| if byte == b'>' { 1 } else { -1 })
        .cycle();

    let rocks = [
        &[(0, 0), (1, 0), (2, 0), (3, 0)][..],
        &[(1, 2), (0, 1), (1, 1), (2, 1), (1, 0)],
        &[(2, 2), (2, 1), (0, 0), (1, 0), (2, 0)],
        &[(0, 3), (0, 2), (0, 1), (0, 0)],
        &[(0, 1), (1, 1), (0, 0), (1, 0)],
    ];

    let mut board: Board = [[false; 7]; 2022 * 3];

    for turn in 0..2022 {
        fall(&mut board, rocks[turn % rocks.len()], &mut jets);
    }

    println!("Part 1: {}", tower_height(&board));
}

fn print(board: &Board, rock: &[(i32, i32)], offset_x: i32, offset_y: i32) {
    let top = tower_height(board) + 7;

    for y in (0..top).rev() {
        print!("|");
        for x in 0..7 {
            if rock.contains(&(x - offset_x, y - offset_y)) {
                print!("@");
            } else if board[y as usize][x as usize] {
                print!("#");
            } else {
                print!(".");
            }
        }

        println!("|");
    }
    println!("+-------+\n");
}
