fn cycles() -> impl Iterator<Item = i32> {
    let mut lines = include_str!("input").lines();
    let mut x = 1;
    let mut addx: Option<i32> = None;

    std::iter::from_fn(move || {
        let during = x;

        if let Some(added) = addx.take() {
            x += added;
        } else {
            addx = lines.next()?.rsplit(' ').next()?.parse().ok();
        }

        Some(during)
    })
}

fn main() {
    let part_1: i32 = cycles()
        .enumerate()
        .skip(19)
        .step_by(40)
        .map(|(i, x)| (i + 1) as i32 * x)
        .sum();
    println!("Part 1: {part_1}");

    for (cycle, x) in cycles().enumerate() {
        let draw_pos = cycle as i32 % 40;

        if x.abs_diff(draw_pos) <= 1 {
            print!("#");
        } else {
            print!(".");
        }

        if draw_pos == 39 {
            println!();
        }
    }
}
