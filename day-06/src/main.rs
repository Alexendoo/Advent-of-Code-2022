fn solve(marker_len: usize) -> usize {
    include_bytes!("input")
        .windows(marker_len)
        .position(|window| {
            window
                .iter()
                .all(|ch| window.iter().filter(|&c| c == ch).count() == 1)
        })
        .unwrap()
        + marker_len
}

fn main() {
    println!("Part 1: {}", solve(4));
    println!("Part 2: {}", solve(14));
}
