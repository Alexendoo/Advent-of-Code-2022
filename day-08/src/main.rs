use std::collections::HashSet;
use std::mem;

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

    let set: HashSet<_> = visible(&board, ltr)
        .chain(visible(&board, rtl))
        .chain(visible(&board, ttb))
        .chain(visible(&board, btt))
        .collect();

    println!("Part 1: {}", set.len());

    let max_scenic: usize = set
        .into_iter()
        .map(|(x, y)| {
            let house_height = board[y][x];

            [
                &mut (0..y).rev().map(|y| (x, y)) as &mut dyn Iterator<Item = (usize, usize)>,
                &mut (0..x).rev().map(|x| (x, y)),
                &mut (y..height).map(|y| (x, y)).skip(1),
                &mut (x..width).map(|x| (x, y)).skip(1),
            ]
            .into_iter()
            .map(|iter| {
                let mut viewable = true;
                iter.take_while(|&(x, y)| mem::replace(&mut viewable, board[y][x] < house_height))
                    .count()
            })
            .product()
        })
        .max()
        .unwrap();

    println!("Part 2: {max_scenic}");
}
