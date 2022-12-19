use pathfinding::prelude::dfs_reach;
use rayon::prelude::{IntoParallelRefIterator, ParallelIterator};

#[derive(Debug, Clone, Copy)]
struct Blueprint {
    id: u32,

    ore: u32,
    clay: u32,
    obsidian: (u32, u32),
    geode: (u32, u32),
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
struct Robots {
    ore: u32,
    clay: u32,
    obsidian: u32,
    geode: u32,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
struct Resources {
    ore: u32,
    clay: u32,
    obsidian: u32,
    geode: u32,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
struct State {
    minute: u32,
    robots: Robots,
    resources: Resources,
}

impl State {
    fn neighbours(mut self, blueprint: Blueprint) -> impl Iterator<Item = State> {
        self.minute += 1;

        let next_state = || {
            let mut next = self;
            let State {
                resources, robots, ..
            } = &mut next;

            resources.ore += robots.ore;
            resources.clay += robots.clay;
            resources.obsidian += robots.obsidian;
            resources.geode += robots.geode;

            next
        };

        [
            Some(next_state()),
            (self.resources.ore >= blueprint.ore).then(|| {
                let mut next = next_state();
                next.robots.ore += 1;
                next.resources.ore -= blueprint.ore;
                next
            }),
            (self.resources.ore >= blueprint.clay).then(|| {
                let mut next = next_state();
                next.robots.clay += 1;
                next.resources.ore -= blueprint.clay;
                next
            }),
            (self.resources.ore >= blueprint.obsidian.0
                && self.resources.clay >= blueprint.obsidian.1)
                .then(|| {
                    let mut next = next_state();
                    next.robots.obsidian += 1;
                    next.resources.ore -= blueprint.obsidian.0;
                    next.resources.clay -= blueprint.obsidian.1;
                    next
                }),
            (self.resources.ore >= blueprint.geode.0
                && self.resources.obsidian >= blueprint.geode.1)
                .then(|| {
                    let mut next = next_state();
                    next.robots.geode += 1;
                    next.resources.ore -= blueprint.geode.0;
                    next.resources.obsidian -= blueprint.geode.1;
                    next
                }),
        ]
        .into_iter()
        .filter_map(move |state| if self.minute <= 24 { state } else { None })
    }
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
            let mut state = State::default();
            state.robots.ore = 1;

            let geodes = dfs_reach(state, |state| state.neighbours(blueprint))
                .filter(|state| state.minute == 24)
                .map(|state| state.resources.geode)
                .max()
                .unwrap();

            geodes * blueprint.id
        })
        .sum();

    println!("Part 1: {quality}");
}
