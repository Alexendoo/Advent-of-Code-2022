#[derive(Clone, Copy)]
struct Num {
    value: i64,
    order: i64,
}

impl std::fmt::Debug for Num {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}

fn mix(mut input: Vec<Num>, rounds: usize) -> i64 {
    let len = input.len() as i64;
    for _ in 0..rounds {
        for i in 0..len {
            let pos = input.iter().position(|n| n.order == i).unwrap();
            let num = input.remove(pos);
            let new_pos = (pos as i64 + num.value).rem_euclid(len - 1) as usize;
            input.insert(new_pos, num);
        }
    }

    let zero_pos = input.iter().position(|n| n.value == 0).unwrap();
    input.rotate_left(zero_pos);

    [1000, 2000, 3000]
        .into_iter()
        .map(|n| input[n % input.len()].value)
        .sum()
}

fn main() {
    let mut input: Vec<_> = include_str!("input")
        .lines()
        .enumerate()
        .map(|(i, line)| Num {
            value: line.parse().unwrap(),
            order: i as _,
        })
        .collect();

    println!("Part 1: {}", mix(input.clone(), 1));

    for num in &mut input {
        num.value *= 811589153;
    }

    println!("Part 2: {}", mix(input, 10));
}
