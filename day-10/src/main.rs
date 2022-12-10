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
    let mut part_1 = 0;
    let mut part_2 = String::with_capacity(246);

    for (i, x) in cycles().enumerate() {
        let draw_pos = i as i32 % 40;

        if x.abs_diff(draw_pos) <= 1 {
            part_2.push('#');
        } else {
            part_2.push('.');
        }

        match draw_pos {
            19 => part_1 += (i + 1) as i32 * x,
            39 => part_2.push('\n'),
            _ => {}
        }
    }

    println!("Part 1: {part_1}");
    println!("Part 2:\n{part_2}");
}
