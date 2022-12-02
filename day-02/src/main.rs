fn chunks(input: &str) -> &[[u8; 4]] {
    unsafe { std::slice::from_raw_parts(input.as_ptr().cast(), input.len() / 4) }
}

const ROCK: u8 = b'A';
const PAPER: u8 = b'B';
const SCISSORS: u8 = b'C';

const LOSE: u8 = b'X';
const DRAW: u8 = b'Y';
const WIN: u8 = b'Z';

fn main() {
    let input = include_str!("input");

    let mut part_1 = 0u32;
    let mut part_2 = 0u32;

    for &[theirs, _, ours, _] in chunks(input) {
        part_1 += match (theirs, ours - (b'X' - b'A')) {
            (ROCK, ROCK) => 3,
            (ROCK, PAPER) => 6,
            (ROCK, SCISSORS) => 0,
            (PAPER, ROCK) => 0,
            (PAPER, PAPER) => 3,
            (PAPER, SCISSORS) => 6,
            (SCISSORS, ROCK) => 6,
            (SCISSORS, PAPER) => 0,
            (SCISSORS, SCISSORS) => 3,
            _ => unreachable!(),
        };

        part_1 += (ours - (b'X' - 1)) as u32;

        let required = match (theirs, ours) {
            (ROCK, LOSE) => SCISSORS,
            (ROCK, DRAW) => ROCK,
            (ROCK, WIN) => PAPER,
            (PAPER, LOSE) => ROCK,
            (PAPER, DRAW) => PAPER,
            (PAPER, WIN) => SCISSORS,
            (SCISSORS, LOSE) => PAPER,
            (SCISSORS, DRAW) => SCISSORS,
            (SCISSORS, WIN) => ROCK,
            _ => unreachable!(),
        };

        part_2 += (required - (ROCK - 1)) as u32;
        part_2 += (ours - LOSE) as u32 * 3;
    }

    println!("Part 1: {part_1}");
    println!("Part 2: {part_2}");
}
