fn main() {
    let mut lines = include_str!("input").lines();
    let mut x = 0;
    let mut addx: Option<i32> = Some(1_i32);

    let iter = std::iter::from_fn(|| {
        let during = x;

        if let Some(added) = addx.take() {
            x += added;
        } else {
            addx = lines.next()?.rsplit(' ').next()?.parse().ok();
        }

        Some(during)
    });

    let part_1: i32 = iter
        .enumerate()
        .skip(20)
        .step_by(40)
        .map(|(i, x)| (i as i32) * x)
        .sum();
    println!("Part 1: {part_1}");
}
