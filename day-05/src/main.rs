#![feature(get_many_mut)]

fn solve(mut stacks: Vec<Vec<char>>, instructions: &str, reverse: bool) -> String {
    for instruction in instructions.lines() {
        let mut iter = instruction.split(' ');
        let mut next = || iter.nth(1).unwrap().parse::<usize>().unwrap();

        let count = next();
        let [from, to] = stacks.get_many_mut([next() - 1, next() - 1]).unwrap();

        let drained = from.drain(from.len() - count..);
        if reverse {
            to.extend(drained.rev());
        } else {
            to.extend(drained);
        }
    }

    stacks.iter().map(|stack| stack.last().unwrap()).collect()
}

fn main() {
    let input = include_str!("input");

    let (initial_stacks, instructions) = input.split_once("\n\n").unwrap();
    let n_stacks: usize = initial_stacks
        .split_ascii_whitespace()
        .next_back()
        .unwrap()
        .parse()
        .unwrap();

    let mut stacks = vec![Vec::new(); n_stacks];

    for line in initial_stacks.lines().rev().skip(1) {
        for n in 0..n_stacks {
            if let Some(&byte @ b'A'..=b'Z') = line.as_bytes().get(n * 4 + 1) {
                stacks[n].push(char::from(byte));
            }
        }
    }

    println!("Part 1: {}", solve(stacks.clone(), instructions, true));
    println!("Part 2: {}", solve(stacks, instructions, false));
}
