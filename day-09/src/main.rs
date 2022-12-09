use std::collections::HashSet;

fn walk<const N: usize>() -> usize {
    let mut rope = [(0i32, 0i32); N];
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
                }
            }

            visited.insert(*rope.last().unwrap());
        }
    }

    visited.len()
}

fn main() {
    println!("Part 1: {}", walk::<2>());
    println!("Part 2: {}", walk::<10>());
}
