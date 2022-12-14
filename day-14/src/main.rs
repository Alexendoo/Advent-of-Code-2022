use std::collections::HashSet;

fn fill(cave: &mut HashSet<(i32, i32)>, void: i32) {
    loop {
        let (mut sand_x, mut sand_y) = (500, 0);
        if cave.contains(&(sand_x, sand_y)) {
            return;
        }

        loop {
            if sand_y < void {
                return;
            } else if !cave.contains(&(sand_x, sand_y - 1)) {
                sand_y -= 1;
            } else if !cave.contains(&(sand_x - 1, sand_y - 1)) {
                sand_x -= 1;
                sand_y -= 1;
            } else if !cave.contains(&(sand_x + 1, sand_y - 1)) {
                sand_x += 1;
                sand_y -= 1;
            } else {
                cave.insert((sand_x, sand_y));
                break;
            }
        }
    }
}

fn main() {
    let mut cave = HashSet::new();

    for path in include_str!("input").lines() {
        let mut points = path.split(" -> ").map(|coords| {
            let (x, y) = coords.split_once(',').unwrap();
            (x.parse::<i32>().unwrap(), -y.parse::<i32>().unwrap())
        });

        let (mut curr_x, mut curr_y) = points.next().unwrap();
        cave.insert((curr_x, curr_y));

        for (x, y) in points {
            while (curr_x, curr_y) != (x, y) {
                curr_x += (x - curr_x).signum();
                curr_y += (y - curr_y).signum();

                cave.insert((curr_x, curr_y));
            }
        }
    }

    let void = cave.iter().map(|&(_, y)| y).min().unwrap() - 2;
    let rocks = cave.len();
    fill(&mut cave, void);
    println!("Part 1: {}", cave.len() - rocks);

    let floor = void..-void+1;
    for x in floor.clone() {
        cave.insert((500 + x, void));
    }
    fill(&mut cave, void);
    println!("Part 2: {}", cave.len() - rocks - floor.len());
}
