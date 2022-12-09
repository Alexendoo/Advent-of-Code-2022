use std::collections::HashSet;

type Point = (i32, i32);

fn walk<const N: usize>(mut rope: [Point; N]) -> usize {
    let mut visited = HashSet::from([(0, 0)]);

    for line in include_str!("input").lines() {
        let (direction, steps) = line.split_once(' ').unwrap();
        let steps: usize = steps.parse().unwrap();

        for _ in 0..steps {
            match direction {
                "U" => rope[0].1 += 1,
                "R" => rope[0].0 += 1,
                "D" => rope[0].1 -= 1,
                "L" => rope[0].0 -= 1,
                _ => unreachable!(),
            }

            for i in 1..N {
                let prev = rope[i - 1];
                let curr = &mut rope[i];

                if prev.0.abs_diff(curr.0) > 1 || prev.1.abs_diff(curr.1) > 1 {
                    curr.0 += (prev.0 - curr.0).signum();
                    curr.1 += (prev.1 - curr.1).signum();
                } else {
                    curr.0 += ((prev.0 - curr.0) / 2).signum();
                    curr.1 += ((prev.1 - curr.1) / 2).signum();
                }
            }

            visited.insert(*rope.last().unwrap());
        }
    }

    visited.len()
}

fn main() {
    println!("Part 1: {}", walk(<[Point; 2]>::default()));
    println!("Part 2: {}", walk(<[Point; 10]>::default()));
}
