use bittle::{Bits, BitsMut, Set};
use pathfinding::directed::dfs::dfs_reach;

#[derive(Default, Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct State {
    opened: Set<u64>,
    current: u32,
    rate: i32,
    released: i32,
    time: u32,
}

fn main() {
    let mut valves = Vec::new();

    for line in include_str!("input").lines() {
        let valve = &line[6..8];
        let rate = line[23..line.find(';').unwrap()].parse::<i32>().unwrap();
        let tunnels: Vec<_> = line[23..]
            .trim_start_matches(|ch: char| !ch.is_ascii_uppercase())
            .split(", ")
            .collect();

        valves.push((valve, rate, tunnels));
    }

    valves.sort_unstable_by_key(|&(_, rate, _)| -rate);
    let aa = valves
        .iter()
        .position(|&(valve, ..)| valve == "AA")
        .unwrap();

    let valves: Vec<(i32, Vec<u32>)> = valves
        .iter()
        .map(|(_, rate, tunnels)| {
            (
                *rate,
                tunnels
                    .iter()
                    .map(|next| {
                        valves
                            .iter()
                            .position(|(target, ..)| target == next)
                            .unwrap() as u32
                    })
                    .collect(),
            )
        })
        .collect();

    let mut max_released = 0;

    let potential_states = |state: &State| {
        let mut next_states = Vec::new();

        if state.time == 0 {
            return next_states;
        }

        let mut next = *state;
        next.time -= 1;
        next.released += next.rate;

        if state.opened.count_ones() == valves.len() as u32 {
            next_states.push(next);
            return next_states;
        }

        let (flow, ref tunnels) = valves[state.current as usize];

        if flow > 0 && !state.opened.test_bit(state.current) {
            let mut n = next.clone();
            n.rate += flow;
            n.opened.set_bit(state.current);
            next_states.push(n);
        }

        for &tunnel in tunnels {
            let mut n = next.clone();
            n.current = tunnel;
            next_states.push(n);
        }

        next_states
    };

    let reachable = dfs_reach(
        State {
            current: aa as u32,
            time: 30,
            ..State::default()
        },
        |state| {
            max_released = max_released.max(state.released);
            let mut next_states = potential_states(state);

            // Omit paths that cannot beat the current best solution
            next_states.retain(|next| {
                let mut released = next.released;
                let mut rate = next.rate;

                let mut unopened = next
                    .opened
                    .iter_zeros()
                    .filter_map(|idx| valves.get(idx as usize));
                for _ in 0..next.time {
                    released += rate;

                    if let Some(&(highest_unopened_rate, _)) = unopened.next() {
                        rate += highest_unopened_rate;
                    }
                }

                released > max_released
            });

            next_states
        },
    );

    for _ in reachable {}

    println!("Part 1: {max_released}");
}
