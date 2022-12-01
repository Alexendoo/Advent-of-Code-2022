fn main() {
    let input = include_str!("input");

    let elves = input
        .split("\n\n")
        .map(|elf| elf.lines().map(|line| line.parse::<u64>().unwrap()));

    let largest = elves.map(|foods| foods.sum::<u64>()).max().unwrap();

    println!("Part 1: {largest}");
}
