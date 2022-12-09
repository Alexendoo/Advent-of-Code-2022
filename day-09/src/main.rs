use std::collections::HashSet;

fn main() {
    let input = include_str!("input");

    let mut head: (i32, i32) = (0, 0);
    let mut tail = (0, 0);

    let mut visited = HashSet::from([tail]);

    for line in input.lines() {
        let (direction, steps) = line.split_once(' ').unwrap();
        let steps: usize = steps.parse().unwrap();

        for _ in 0..steps {
            match direction {
                "U" => head.1 += 1,
                "R" => head.0 += 1,
                "D" => head.1 -= 1,
                "L" => head.0 -= 1,
                _ => unreachable!(),
            }

            if head.0.abs_diff(tail.0) > 1 || head.1.abs_diff(tail.1) > 1 {
                tail.0 += (head.0 - tail.0).signum();
                tail.1 += (head.1 - tail.1).signum();
            } else {
                tail.0 += ((head.0 - tail.0) / 2).signum();
                tail.1 += ((head.1 - tail.1) / 2).signum();
            }

            visited.insert(tail);
        }
    }

    println!("Part 1: {}", visited.len());
}
