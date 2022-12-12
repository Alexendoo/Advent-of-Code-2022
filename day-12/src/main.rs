use pathfinding::directed::bfs::bfs;
use std::mem;

fn solve(board: &[&[u8]], start: (usize, usize), target: u8, reverse: bool) -> usize {
    bfs(
        &start,
        |&(x, y)| {
            let elevation = |x: usize, y: usize| {
                board.get(y)?.get(x).map(|&ch| match ch {
                    b'S' => b'a',
                    b'E' => b'z',
                    _ => ch,
                })
            };

            [(1, 0), (0, 1), (-1, 0), (0, -1)]
                .into_iter()
                .filter_map(move |(dx, dy)| {
                    let x_next = x.checked_add_signed(dx)?;
                    let y_next = y.checked_add_signed(dy)?;

                    let mut curr = elevation(x, y)?;
                    let mut next = elevation(x_next, y_next)?;
                    if reverse {
                        mem::swap(&mut curr, &mut next);
                    }

                    (curr + 1 >= next).then_some((x_next, y_next))
                })
        },
        |&(x, y)| board[y][x] == target,
    )
    .unwrap()
    .len()
    .saturating_sub(1)
}

fn main() {
    let input = include_str!("input");
    let board: Vec<&[u8]> = input.lines().map(|line| line.as_bytes()).collect();

    let pos = |target| {
        board
            .iter()
            .enumerate()
            .find_map(|(y, row)| row.iter().position(|&ch| ch == target).map(|x| (x, y)))
            .unwrap()
    };

    println!("Part 1: {}", solve(&board, pos(b'S'), b'E', false));
    println!("Part 2: {}", solve(&board, pos(b'E'), b'a', true));
}
