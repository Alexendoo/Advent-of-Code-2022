fn main() {
    let input = include_str!("input");

    let ranges = input.lines().map(|line| {
        let mut nums = line.split(['-', ',']);
        let mut next = || nums.next().unwrap().parse::<u8>().unwrap();

        (next()..=next(), next()..=next())
    });

    let count = ranges
        .filter(|(l, r)| {
            (l.contains(&r.start()) && l.contains(&r.end()))
                || (r.contains(&l.start()) && r.contains(&l.end()))
        })
        .count();

    println!("Part 1: {count}");
}
