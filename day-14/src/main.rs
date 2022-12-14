use std::collections::BTreeMap;

fn main() {
    let mut cave = BTreeMap::new();

    for path in include_str!("input").lines() {
        let mut points = path.split(" -> ").map(|coords| {
            let (x, y) = coords.split_once(',').unwrap();
            (x.parse::<i32>().unwrap(), -y.parse::<i32>().unwrap())
        });

        let (mut curr_x, mut curr_y) = points.next().unwrap();
        cave.insert((curr_x, curr_y), '#');

        for (x, y) in points {
            while (curr_x, curr_y) != (x, y) {
                curr_x += (x - curr_x).signum();
                curr_y += (y - curr_y).signum();

                cave.insert((curr_x, curr_y), '#');
            }
        }
    }

    let floor = cave.keys().map(|&(_, y)| y).min().unwrap();
    let rocks = cave.len();

    'outer: loop {
        let (mut sand_x, mut sand_y) = (500, 0);

        loop {
            if sand_y < floor {
                break 'outer;
            } else if !cave.contains_key(&(sand_x, sand_y - 1)) {
                sand_y -= 1;
            } else if !cave.contains_key(&(sand_x - 1, sand_y - 1)) {
                sand_x -= 1;
                sand_y -= 1;
            } else if !cave.contains_key(&(sand_x + 1, sand_y - 1)) {
                sand_x += 1;
                sand_y -= 1;
            } else {
                cave.insert((sand_x, sand_y), 'o');
                continue 'outer;
            }
        }
    }

    println!("Part 1: {}", cave.len() - rocks);
}
