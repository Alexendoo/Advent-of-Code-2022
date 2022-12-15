use std::collections::HashSet;

fn distance(a: (i32, i32), b: (i32, i32)) -> i32 {
    (a.0 - b.0).abs() + (a.1 - b.1).abs()
}

fn main() {
    let sensors: Vec<_> = include_str!("input")
        .lines()
        .map(|line| {
            let coord = |c: &str| -> (i32, i32) {
                let (x, y) = c.split_once(", y=").unwrap();
                (x.parse().unwrap(), y.parse().unwrap())
            };
            let (sensor, beacon) = line[12..].split_once(": closest beacon is at x=").unwrap();
            (coord(sensor), coord(beacon))
        })
        .collect();

    let y = 2000000;
    let xs = sensors
        .iter()
        .flat_map(|&((sx, sy), (bx, by))| {
            let span = distance((sx, sy), (bx, by)) - (sy - y).abs();
            sx - span..sx + span + 1
        })
        .collect::<HashSet<_>>();

    let overlap = sensors
        .iter()
        .map(|&(_, beacon)| beacon)
        .filter(|&(bx, by)| by == y && xs.contains(&bx))
        .collect::<HashSet<_>>();

    println!("Part 1: {}", xs.len() - overlap.len());
}
