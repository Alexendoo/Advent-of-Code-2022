use std::collections::{BTreeSet, HashSet};

fn visible<'a, I>(board: &'a [Vec<i8>], positions: I) -> impl Iterator<Item = (usize, usize)> + 'a
where
    I: Iterator + 'a,
    I::Item: Iterator<Item = (usize, usize)>,
{
    positions.flat_map(move |iter| {
        let mut highest = -1;
        iter.filter(move |&(x, y)| {
            let height = board[y][x];

            if height > highest {
                highest = height;
                true
            } else {
                false
            }
        })
    })
}

fn main() {
    let board: Vec<Vec<i8>> = include_str!("input")
        .lines()
        .map(|line| line.bytes().map(|b| (b - b'0') as i8).collect())
        .collect();

    let width = board[0].len();
    let height = board.len();

    let ltr = (0..width).map(|x| (0..height).map(move |y| (x, y)));
    let rtl = ltr.clone().map(|i| i.rev());
    let ttb = (0..height).map(|y| (0..width).map(move |x| (x, y)));
    let btt = ttb.clone().map(|i| i.rev());

    let set: BTreeSet<_> = visible(&board, ltr)
        .chain(visible(&board, rtl))
        .chain(visible(&board, ttb))
        .chain(visible(&board, btt))
        .collect();

    println!("Part 1: {}", set.len());
}
