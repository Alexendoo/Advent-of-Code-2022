use std::collections::{BTreeSet, HashMap};

use pathfinding::prelude::dfs_reach;

#[derive(Default, Debug, PartialEq, Eq, Hash, Clone)]
struct State {
    opened: BTreeSet<&'static str>,
    current: &'static str,
    rate: u32,
    released: u32,
    time: u32,
}

fn main() {
    let mut valves = HashMap::new();

    for line in include_str!("input").lines() {
        let valve = &line[6..8];
        let rate = line[23..line.find(';').unwrap()].parse::<u32>().unwrap();
        let tunnels: Vec<_> = line[23..]
            .trim_start_matches(|ch: char| !ch.is_ascii_uppercase())
            .split(", ")
            .collect();

        valves.insert(valve, (rate, tunnels));
    }

    let x = dfs_reach(
        State {
            current: "AA",
            time: 30,
            ..State::default()
        },
        |state| {
            let mut next_states = Vec::new();

            if state.time == 0 {
                return next_states;
            }

            let mut next = state.clone();
            next.time -= 1;
            next.released += next.rate;

            if state.opened.len() == valves.len() {
                next_states.push(next);
                return next_states;
            }

            let (flow, ref tunnels) = valves[state.current];

            if flow > 0 && !state.opened.contains(state.current) {
                let mut n = next.clone();
                n.rate += flow;
                n.opened.insert(state.current);
                next_states.push(n);
            }

            for &tunnel in tunnels {
                let mut n = next.clone();
                n.current = tunnel;
                next_states.push(n);
            }

            next_states
        },
    );

    let mut max = 0;
    for z in x {
        if z.released > max {
            max = z.released;
            println!("{max}");
        }
    }
}
