use std::collections::HashSet;
use std::ops::Range;

fn distance(a: (i32, i32), b: (i32, i32)) -> i32 {
    (a.0 - b.0).abs() + (a.1 - b.1).abs()
}

enum Merged {
    Overlapping(Range<i32>),
    Disjoint(i32),
}

fn merge(sensors: &[((i32, i32), (i32, i32))], y: i32) -> Merged {
    let mut ranges = Vec::new();

    for &((sx, sy), (bx, by)) in sensors {
        let span = distance((sx, sy), (bx, by)) - (sy - y).abs();
        if span >= 0 {
            ranges.push((sx - span, sx + span + 1));
        }
    }

    ranges.sort_unstable();

    let mut iter = ranges.into_iter();
    let (start, mut end) = iter.next().unwrap();
    for (s, e) in iter {
        if s <= end {
            end = end.max(e);
        } else {
            return Merged::Disjoint(s - 1);
        }
    }

    Merged::Overlapping(start..end)
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

    let part_1_y = 2000000;
    let Merged::Overlapping(xs) = merge(&sensors, part_1_y) else {
        unreachable!()
    };

    let overlap = sensors
        .iter()
        .map(|&(_, beacon)| beacon)
        .filter(|&(bx, by)| by == part_1_y && xs.contains(&bx))
        .collect::<HashSet<_>>();

    println!("Part 1: {}", xs.len() - overlap.len());

    let (x, y) = (0..)
        .find_map(|y| match merge(&sensors, y) {
            Merged::Disjoint(x) => Some((x, y)),
            Merged::Overlapping(_) => None,
        })
        .unwrap();

    println!("Part 2: {}", x as i64 * 4000000 + y as i64);
}
