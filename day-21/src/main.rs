use std::collections::HashMap;

#[derive(Debug, Clone, Copy)]
enum Job {
    Val(i64),
    Op(&'static str, char, &'static str),
}

fn solve(monkey: &str, jobs: &HashMap<&str, Job>) -> i64 {
    match jobs[monkey] {
        Job::Val(val) => val,
        Job::Op(l, op, r) => {
            let l = solve(l, jobs);
            let r = solve(r, jobs);

            match op {
                '+' => l + r,
                '-' => l - r,
                '/' => l / r,
                '*' => l * r,
                _ => unreachable!(),
            }
        }
    }
}

fn main() {
    let jobs = include_str!("input")
        .lines()
        .map(|line| {
            let op = match line[6..].parse() {
                Ok(val) => Job::Val(val),
                Err(_) => Job::Op(
                    &line[6..10],
                    line[11..].chars().next().unwrap(),
                    &line[13..],
                ),
            };

            (&line[..4], op)
        })
        .collect();

    println!("Part 1: {}", solve("root", &jobs));
}
