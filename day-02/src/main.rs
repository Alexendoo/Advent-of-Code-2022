fn chunks(input: &str) -> &[[u8; 4]] {
    unsafe { std::slice::from_raw_parts(input.as_ptr().cast(), input.len() / 4) }
}

#[derive(Clone, Copy)]
enum Hand {
    Rock,
    Paper,
    Scissors,
}
use Hand::*;

fn main() {
    let input = include_str!("input");

    let mut score = 0;
    for &[theirs, _, ours, _] in chunks(input) {
        let theirs = match theirs {
            b'A' => Rock,
            b'B' => Paper,
            b'C' => Scissors,
            _ => unreachable!(),
        };

        let ours = match ours {
            b'X' => Rock,
            b'Y' => Paper,
            b'Z' => Scissors,
            _ => unreachable!(),
        };

        score += match (ours, theirs) {
            (Rock, Rock) => 3,
            (Rock, Paper) => 0,
            (Rock, Scissors) => 6,
            (Paper, Rock) => 6,
            (Paper, Paper) => 3,
            (Paper, Scissors) => 0,
            (Scissors, Rock) => 0,
            (Scissors, Paper) => 6,
            (Scissors, Scissors) => 3,
        };

        score += 1 + ours as i32;
    }

    println!("Part 1: {score}");
}
