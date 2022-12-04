fn main() {
    let input = include_str!("input");

    let ranges = input
        .lines()
        .map(|line| {
            let mut nums = line.split(['-', ',']);
            let mut next = || nums.next().unwrap().parse::<u8>().unwrap();

            (next()..=next(), next()..=next())
        })
        .collect::<Vec<_>>();

    let part_1 = ranges
        .iter()
        .filter(|(l, r)| {
            (l.contains(&r.start()) && l.contains(&r.end()))
                || (r.contains(&l.start()) && r.contains(&l.end()))
        })
        .count();

    println!("Part 1: {part_1}");

    let part_2 = ranges
        .iter()
        .filter(|(l, r)| {
            l.contains(&r.start())
                || l.contains(&r.end())
                || r.contains(&l.start())
                || r.contains(&l.end())
        })
        .count();

    println!("Part 2: {part_2}");
}
