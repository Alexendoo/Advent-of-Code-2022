#[derive(Clone, Copy)]
struct Num {
    value: i32,
    order: i32,
}

impl std::fmt::Debug for Num {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}

fn main() {
    let mut input: Vec<_> = include_str!("input")
        .lines()
        .enumerate()
        .map(|(i, line)| Num {
            value: line.parse().unwrap(),
            order: i as i32,
        })
        .collect();

    let len = input.len() as i32;
    for i in 0..len {
        let pos = input.iter().position(|n| n.order == i).unwrap();
        let num = input.remove(pos);
        let new_pos = (pos as i32 + num.value).rem_euclid(len - 1) as usize;
        input.insert(new_pos, num);
    }

    let zero_pos = input.iter().position(|n| n.value == 0).unwrap();
    input.rotate_left(zero_pos);

    println!(
        "Part 1: {}",
        [1000, 2000, 3000]
            .into_iter()
            .map(|n| input[n % input.len()].value)
            .sum::<i32>()
    )
}
