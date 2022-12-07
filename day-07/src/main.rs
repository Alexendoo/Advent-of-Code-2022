use std::collections::HashMap;
use std::path::{Path, PathBuf};

fn main() {
    let input = include_str!("input");

    let mut cwd = PathBuf::new();
    let mut dirs = HashMap::new();

    for line in input.lines() {
        match line.split_once(' ').unwrap() {
            ("$", "ls") | ("dir", _) => {}
            ("$", "cd ..") => {
                cwd.pop();
            }
            ("$", cd) => {
                cwd.push(&cd[3..]);
                dirs.insert(cwd.clone(), 0);
            }
            (size, _) => {
                let size = size.parse::<u32>().unwrap();
                for dir in cwd.ancestors() {
                    *dirs.get_mut(dir).unwrap() += size;
                }
            }
        }
    }

    let part_1: u32 = dirs.values().filter(|&&v| v <= 100000).sum();

    let needed = dirs[Path::new("/")] - 40000000;
    let part_2 = dirs.into_values().filter(|&v| v >= needed).min().unwrap();

    println!("Part 1: {part_1}");
    println!("Part 2: {part_2}");
}
