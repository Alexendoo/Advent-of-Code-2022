#![feature(iter_next_chunk)]

use std::collections::HashSet;

fn main() {
    let points: HashSet<[i32; 3]> = include_str!("input")
        .lines()
        .map(|line| {
            line.split(',')
                .map(|n| n.parse::<i32>().unwrap())
                .next_chunk()
                .unwrap()
        })
        .collect();

    let mut bounds = i32::MAX..i32::MIN;

    for &coord in points.iter().flatten() {
        bounds.start = bounds.start.min(coord - 1);
        bounds.end = bounds.end.max(coord + 2);
    }

    let neighbours = |x, y, z| {
        [
            [1, 0, 0],
            [-1, 0, 0],
            [0, 1, 0],
            [0, -1, 0],
            [0, 0, 1],
            [0, 0, -1],
        ]
        .into_iter()
        .map(move |[dx, dy, dz]| [x + dx, y + dy, z + dz])
        .filter(|point| point.iter().all(|coord| bounds.contains(coord)))
    };

    let surface_area = points
        .iter()
        .flat_map(|&[x, y, z]| neighbours(x, y, z))
        .filter(|point| !points.contains(point))
        .count();

    println!("Part 1: {surface_area}");
}
