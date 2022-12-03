fn main() {
    let input = include_str!("input");

    let mut part_1 = 0;

    for line in input.lines() {
        let (left, right) = line.as_bytes().split_at(line.len() / 2);

        let duplicate = right.iter().find(|ch| left.contains(ch)).unwrap();
        let priority = match duplicate {
            b'a'..=b'z' => duplicate - (b'a' - 1),
            b'A'..=b'Z' => duplicate - (b'A' - 27),
            _ => unreachable!(),
        };
        part_1 += u32::from(priority);
    }

    println!("Part 1: {part_1}");
}
