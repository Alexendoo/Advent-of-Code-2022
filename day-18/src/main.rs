#![feature(iter_next_chunk)]

use pathfinding::prelude::dfs_reach;
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

    let neighbours = |&[x, y, z]: &[i32; 3]| {
        [
            [x + 1, y, z],
            [x - 1, y, z],
            [x, y + 1, z],
            [x, y - 1, z],
            [x, y, z + 1],
            [x, y, z - 1],
        ]
        .into_iter()
        .filter(|point| point.iter().all(|coord| bounds.contains(coord)))
        .filter(|point| !points.contains(point))
    };

    let surface_area = points.iter().flat_map(neighbours).count();

    println!("Part 1: {surface_area}");

    let exterior_points: HashSet<[i32; 3]> = dfs_reach([bounds.start; 3], neighbours).collect();
    let exterior_surface_area = points
        .iter()
        .flat_map(neighbours)
        .filter(|point| exterior_points.contains(point))
        .count();

    println!("Part 2: {exterior_surface_area}");
}
