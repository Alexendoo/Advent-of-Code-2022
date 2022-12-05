fn main() {
    let input = include_str!("input");

    let (initial_stacks, instructions) = input.split_once("\n\n").unwrap();
    let n_stacks: usize = initial_stacks
        .split_ascii_whitespace()
        .next_back()
        .unwrap()
        .parse()
        .unwrap();

    let mut stacks = vec![Vec::<char>::new(); n_stacks];

    for line in initial_stacks.lines().rev().skip(1) {
        for n in 0..n_stacks {
            if let Some(&ch @ b'A'..=b'Z') = line.as_bytes().get(n * 4 + 1) {
                stacks[n].push(ch.into());
            }
        }
    }

    for instruction in instructions.lines() {
        let mut iter = instruction.split(' ');
        let mut next = || iter.nth(1).unwrap().parse::<usize>().unwrap();

        let count = next();
        let from = next() - 1;
        let to = next() - 1;

        for _ in 0..count {
            let ch = stacks[from].pop().unwrap();
            stacks[to].push(ch);
        }
    }

    let part_1 = stacks.iter().map(|stack| stack.last().unwrap()).collect::<String>();
    println!("Part 1: {part_1}");
}
