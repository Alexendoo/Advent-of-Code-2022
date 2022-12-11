#![feature(iter_next_chunk)]

use std::cell::RefCell;
use std::mem;

#[derive(Debug, Clone)]
struct Monkey {
    items: Vec<u64>,
    op: char,
    operands: [Option<u64>; 2],
    test: u64,
    then: usize,
    r#else: usize,
    inspected: usize,
}

fn solve(monkeys: Vec<RefCell<Monkey>>, divisor: u64, rounds: u64, domain: u64) -> usize {
    for _ in 0..rounds {
        for mut monkey in monkeys.iter().map(RefCell::borrow_mut) {
            monkey.inspected += monkey.items.len();

            for item in mem::take(&mut monkey.items) {
                let [lhs, rhs] = monkey.operands.map(|side| side.unwrap_or(item) % domain);
                let item = match monkey.op {
                    '+' => (lhs + rhs) / divisor,
                    '*' => (lhs * rhs) / divisor,
                    _ => unreachable!(),
                };

                let throw = if item % monkey.test == 0 {
                    monkey.then
                } else {
                    monkey.r#else
                };
                monkeys[throw].borrow_mut().items.push(item);
            }
        }
    }

    let mut largest: Vec<usize> = monkeys
        .iter()
        .map(|monkey| monkey.borrow().inspected)
        .collect();
    largest.sort_unstable();

    if let [.., a, b] = largest[..] {
        a * b
    } else {
        panic!()
    }
}

fn main() {
    let mut monkeys = Vec::new();

    for monkey in include_str!("input").split("\n\n") {
        let [_, starting, operation, test, then, r#else] = monkey.lines().next_chunk().unwrap();

        let [lhs, op, rhs] = operation[19..].split(' ').next_chunk().unwrap();
        let [lhs, rhs] = [lhs, rhs].map(|side| side.parse().ok());

        monkeys.push(RefCell::new(Monkey {
            items: starting[18..]
                .split(", ")
                .map(|x| x.parse().unwrap())
                .collect(),
            op: op.chars().next().unwrap(),
            operands: [lhs, rhs],
            test: test[21..].parse().unwrap(),
            then: then[29..].parse().unwrap(),
            r#else: r#else[30..].parse().unwrap(),
            inspected: 0,
        }));
    }

    let domain: u64 = monkeys.iter().map(|monkey| monkey.borrow().test).product();

    println!("Part 1: {}", solve(monkeys.clone(), 3, 20, domain));
    println!("Part 2: {}", solve(monkeys, 1, 10000, domain));
}
