fn bitset(input: &[u8]) -> u64 {
    let mut set = 0;
    for ch in input {
        let priority = match ch {
            b'a'..=b'z' => ch - (b'a' - 1),
            b'A'..=b'Z' => ch - (b'A' - 27),
            _ => unreachable!(),
        };
        set |= 1 << priority;
    }
    set
}

fn main() {
    let input = include_str!("input");

    let mut part_1 = 0;
    let mut part_2 = 0;
    let mut carry = !0;

    for (idx, line) in input.lines().enumerate() {
        let (left, right) = line.as_bytes().split_at(line.len() / 2);
        let left = bitset(left);
        let right = bitset(right);

        part_1 += (left & right).trailing_zeros();
        carry &= left | right;

        if idx % 3 == 2 {
            part_2 += carry.trailing_zeros();
            carry = !0;
        }
    }

    println!("Part 1: {part_1}");
    println!("Part 2: {part_2}");
}
