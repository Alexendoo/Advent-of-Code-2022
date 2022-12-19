use rand::prelude::*;
use rayon::prelude::*;

#[derive(Debug, Clone, Copy)]
struct Blueprint {
    id: u32,

    ore: u32,
    clay: u32,
    obsidian: (u32, u32),
    geode: (u32, u32),
}

#[derive(Debug, Default, Clone, Copy)]
struct Resources {
    ore: u32,
    clay: u32,
    obsidian: u32,
    geode: u32,
}

fn random_walk(bp: Blueprint, rng: &mut SmallRng, time: u32) -> u32 {
    let mut produced = Resources::default();
    let mut robots = Resources {
        ore: 1,
        ..Resources::default()
    };

    for _ in 0..time {
        let start = produced;

        produced.ore += robots.ore;
        produced.clay += robots.clay;
        produced.obsidian += robots.obsidian;
        produced.geode += robots.geode;

        if start.ore >= bp.geode.0 && start.obsidian >= bp.geode.1 && rng.gen() {
            robots.geode += 1;
            produced.ore -= bp.geode.0;
            produced.obsidian -= bp.geode.1;
        } else if start.ore >= bp.obsidian.0 && start.clay >= bp.obsidian.1 && rng.gen() {
            robots.obsidian += 1;
            produced.ore -= bp.obsidian.0;
            produced.clay -= bp.obsidian.1;
        } else if start.ore >= bp.clay && rng.gen() {
            robots.clay += 1;
            produced.ore -= bp.clay;
        } else if start.ore >= bp.ore && rng.gen() {
            robots.ore += 1;
            produced.ore -= bp.ore;
        }
    }

    produced.geode
}

fn main() {
    let blueprints = include_str!("input")
        .lines()
        .map(|line| {
            let mut numbers = line.split([' ', ':']).filter_map(|word| word.parse().ok());
            let mut next = || numbers.next().unwrap();
            Blueprint {
                id: next(),
                ore: next(),
                clay: next(),
                obsidian: (next(), next()),
                geode: (next(), next()),
            }
        })
        .collect::<Vec<_>>();

    let quality: u32 = blueprints
        .par_iter()
        .map(|&blueprint| {
            let mut rng = SmallRng::from_entropy();
            let geodes = (0..10_000_000)
                .map(|_| random_walk(blueprint, &mut rng, 24))
                .max()
                .unwrap();

            geodes * blueprint.id
        })
        .sum();

    println!("Part 1: {quality}");

    let geodes: u32 = blueprints
        .par_iter()
        .take(3)
        .map(|&blueprint| {
            (0..100)
                .into_par_iter()
                .map(|_| {
                    let mut rng = SmallRng::from_entropy();
                    (0..10_000_000)
                        .map(|_| random_walk(blueprint, &mut rng, 32))
                        .max()
                        .unwrap()
                })
                .max()
                .unwrap()
        })
        .product();

    println!("Part 2: {geodes}");
}
