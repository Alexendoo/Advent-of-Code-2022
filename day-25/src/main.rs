fn main() {
    let sum: i64 = include_str!("input")
        .lines()
        .map(|line| {
            line.chars().fold(0, |dec, ch| {
                let digit = match ch {
                    '=' => -2,
                    '-' => -1,
                    _ => ch as i64 - '0' as i64,
                };
                dec * 5 + digit
            })
        })
        .sum();

    let mut snafu = String::new();
    let mut remaining = sum;

    while remaining != 0 {
        let digit = remaining % 5;
        remaining /= 5;

        if digit > 2 {
            remaining += 1;
        }

        snafu.push(match digit {
            0 => '0',
            1 => '1',
            2 => '2',
            3 => '=',
            _ => '-',
        });
    }

    println!("Part 1: {}", snafu.chars().rev().collect::<String>());
}
