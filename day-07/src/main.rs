use std::collections::HashMap;
use std::path::PathBuf;

fn main() {
    let input = include_str!("input");

    let mut cwd = PathBuf::new();
    let mut dirs = HashMap::from([(PathBuf::from("/"), 0)]);

    for line in input.lines() {
        match line.split_once(' ').unwrap() {
            ("$", "ls") => {}
            ("$", cd) => match &cd[3..] {
                ".." => {
                    cwd.pop();
                }
                path => cwd.push(path),
            },
            ("dir", dir) => {
                dirs.insert(cwd.join(dir), 0);
            }
            (size, file) => {
                let size = size.parse::<u32>().unwrap();
                for dir in cwd.ancestors() {
                    *dirs.get_mut(dir).unwrap() += size;
                }
            }
        }
    }

    let part_1: u32 = dirs.values().filter(|&&v| v <= 100000).sum();
    println!("Part 1: {part_1}");
}
