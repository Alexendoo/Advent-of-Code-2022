fn main() {
    let input = include_str!("input");

    let part_1 = 4 + input
        .as_bytes()
        .windows(4)
        .position(|window| {
            window
                .iter()
                .all(|ch| window.iter().filter(|&c| c == ch).count() == 1)
        })
        .unwrap();

    println!("Part 1: {part_1}");
}
