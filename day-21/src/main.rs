use std::collections::HashMap;

#[derive(Debug, Clone, Copy)]
enum Job {
    Val(i64),
    Op(&'static str, char, &'static str),
}

fn solve(monkey: &str, jobs: &HashMap<&str, Job>) -> Option<i64> {
    match jobs[monkey] {
        Job::Val(val) => Some(val),
        Job::Op(lhs, op, rhs) => {
            let lhs = solve(lhs, jobs)?;
            let rhs = solve(rhs, jobs)?;

            match op {
                '+' => lhs.checked_add(rhs),
                '-' => lhs.checked_sub(rhs),
                '/' => lhs.checked_div(rhs),
                '*' => lhs.checked_mul(rhs),
                _ => unreachable!(),
            }
        }
    }
}

fn contains_humn(monkey: &str, jobs: &HashMap<&str, Job>) -> bool {
    if monkey == "humn" {
        return true;
    }

    match jobs[monkey] {
        Job::Val(_) => false,
        Job::Op(l, _, r) => contains_humn(l, jobs) || contains_humn(r, jobs),
    }
}

fn binary_search(mut start: i64, mut end: i64, mut f: impl FnMut(i64) -> bool) -> i64 {
    while start < end {
        let mid = start + (end - start) / 2;
        if f(mid) {
            start = mid + 1;
        } else {
            end = mid;
        }
    }

    start
}

fn main() {
    let mut jobs = include_str!("input")
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

    println!("Part 1: {}", solve("root", &jobs).unwrap());

    let Job::Op(l, _, r) = jobs["root"] else { unreachable!() };

    let (human_branch, target) = if contains_humn(l, &jobs) {
        (l, solve(r, &jobs).unwrap())
    } else {
        (r, solve(l, &jobs).unwrap())
    };

    let initial = solve(human_branch, &jobs).unwrap();
    jobs.insert("humn", Job::Val(0));
    let zero = solve(human_branch, &jobs).unwrap();

    let direction = zero.cmp(&initial);

    let humn = binary_search(0, i64::MAX, |i| {
        jobs.insert("humn", Job::Val(i));
        solve(human_branch, &jobs).cmp(&Some(target)) == direction
    });

    println!("Part 2: {humn}");
}
