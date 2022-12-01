use std::cmp::Reverse;
use std::collections::BinaryHeap;

fn main() {
    let input = include_str!("input");

    let mut sums = input.split("\n\n").map(|elf| {
        Reverse(
            elf.lines()
                .map(|line| line.parse::<u64>().unwrap())
                .sum::<u64>(),
        )
    });

    let mut heap = BinaryHeap::from_iter(sums.by_ref().take(3));

    for sum in sums {
        let mut head = heap.peek_mut().unwrap();

        if *head > sum {
            *head = sum;
        }
    }

    let largest = heap.iter().map(|sum| sum.0);

    println!("Part 1: {}", largest.clone().max().unwrap());
    println!("Part 2: {}", heap.iter().map(|sum| sum.0).sum::<u64>())
}
