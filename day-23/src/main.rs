use std::collections::{HashMap, HashSet};

fn tick(board: &mut HashSet<(i32, i32)>, i: usize) -> bool {
    let directions = [
        // N, NE, NW
        [(0, 1), (1, 1), (-1, 1)],
        // S, SE, SW
        [(0, -1), (1, -1), (-1, -1)],
        // W, NW, SW
        [(-1, 0), (-1, 1), (-1, -1)],
        // E, NE, SE
        [(1, 0), (1, 1), (1, -1)],
    ];

    let mut counts: HashMap<(i32, i32), u32> = HashMap::new();
    let mut moves = HashMap::new();

    for &(x, y) in board.iter() {
        let mut cycle = directions
            .into_iter()
            .cycle()
            .skip(i % 4)
            .take(4)
            .map(|arr| arr.map(|(dx, dy)| (x + dx, y + dy)));

        if !cycle.clone().flatten().any(|point| board.contains(&point)) {
            continue;
        }

        let proposed = cycle.find_map(|adjacents| {
            adjacents
                .iter()
                .all(|point| !board.contains(point))
                .then(|| adjacents[0])
        });

        if let Some(proposed) = proposed {
            *counts.entry(proposed).or_default() += 1;
            moves.insert((x, y), proposed);
        }
    }

    for (from, to) in moves {
        if counts[&to] == 1 {
            board.remove(&from);
            board.insert(to);
        }
    }

    counts.is_empty()
}

fn main() {
    let mut board: HashSet<(i32, i32)> = include_str!("input")
        .lines()
        .rev()
        .enumerate()
        .flat_map(|(y, line)| {
            line.char_indices().filter_map(move |(x, ch)| {
                if ch == '#' {
                    Some((x as i32, y as i32))
                } else {
                    None
                }
            })
        })
        .collect();

    for i in 0..10 {
        tick(&mut board, i);
    }

    let (mut x0, mut x1, mut y0, mut y1) = (i32::MAX, i32::MIN, i32::MAX, i32::MIN);

    for &(x, y) in &board {
        x0 = x0.min(x);
        x1 = x1.max(x + 1);
        y0 = y0.min(y);
        y1 = y1.max(y + 1);
    }

    println!("Part 1: {}", (x1 - x0) * (y1 - y0) - board.len() as i32);
    println!(
        "Part 2: {}",
        (10..).find(|&i| tick(&mut board, i)).unwrap() + 1
    );
}
