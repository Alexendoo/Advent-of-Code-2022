fn tower_height(board: &[[bool; 7]]) -> i32 {
    board.partition_point(|row| row.contains(&true)) as i32
}

fn fall(board: &mut [[bool; 7]], rock: &[(i32, i32)], mut jets: impl Iterator<Item = i32>) {
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

                board[y as usize][x as usize] = true;
            }

            break;
        }
    }
}

fn find_period(board: &[[bool; 7]]) -> (usize, usize) {
    let end = tower_height(board) as usize;

    for period_height in (0..end / 2).rev() {
        for start in 0.. {
            let mut chunks = board[start..end].chunks(period_height);
            let Some(a) = chunks.next() else { break };
            let Some(b) = chunks.next() else { break };

            if a == b {
                let rock_spaces = a.iter().flatten().filter(|&&b| b).count();
                // 22 total spaces in the 5 rocks
                let rocks_per_period = (rock_spaces / 22) * 5;

                return (rocks_per_period, period_height);
            }
        }
    }

    unreachable!()
}

fn main() {
    let input = include_str!("input").trim();
    let mut jets = input
        .bytes()
        .map(|byte| if byte == b'>' { 1 } else { -1 })
        .cycle();

    let rocks: [&[(i32, i32)]; 5] = [
        &[(0, 0), (1, 0), (2, 0), (3, 0)],
        &[(1, 2), (0, 1), (1, 1), (2, 1), (1, 0)],
        &[(2, 2), (2, 1), (0, 0), (1, 0), (2, 0)],
        &[(0, 3), (0, 2), (0, 1), (0, 0)],
        &[(0, 1), (1, 1), (0, 0), (1, 0)],
    ];
    let mut rock_cycle = rocks.into_iter().cycle();

    let mut board = vec![[false; 7]; 10_000_000];

    for _ in 0..2022 {
        fall(&mut board, rock_cycle.next().unwrap(), &mut jets);
    }

    println!("Part 1: {}", tower_height(&board));

    // Assume there's a repetition within the first 5000 rocks
    for _ in 2022..5000 {
        fall(&mut board, rock_cycle.next().unwrap(), &mut jets);
    }

    let pending = 1000000000000 - 5000;
    let (rocks_per_period, period_height) = find_period(&board);

    let fake_turns = pending / rocks_per_period;
    let pending = pending % rocks_per_period;

    for _ in 0..pending {
        fall(&mut board, rock_cycle.next().unwrap(), &mut jets);
    }

    println!(
        "Part 2: {}",
        tower_height(&board) as usize + fake_turns * period_height
    );
}
