use bittle::{Bits, BitsOwned, Set};
use pathfinding::directed::dfs::dfs_reach;
use std::ops::Add;

#[derive(Default, Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct State {
    opened: Set<u64>,

    human: u32,
    elephant: Option<u32>,

    rate: i32,
    released: i32,
    time: u32,
}

impl Add<Delta> for State {
    type Output = State;

    fn add(self, rhs: Delta) -> State {
        match rhs {
            Delta::Open { flow, idx } => State {
                opened: self.opened.with_bit(idx),
                rate: self.rate + flow,
                ..self
            },
            Delta::MoveHuman(pos) => State { human: pos, ..self },
            Delta::MoveElephant(pos) => State {
                elephant: Some(pos),
                ..self
            },
        }
    }
}

fn possible_route(state: &State, valves: &[(i32, Vec<u32>)], max_released: i32) -> bool {
    let mut released = state.released;
    let mut rate = state.rate;

    let mut unopened = state
        .opened
        .iter_zeros()
        .filter_map(|idx| valves.get(idx as usize));
    for _ in 0..state.time {
        released += rate;

        if let Some(&(highest_unopened_rate, _)) = unopened.next() {
            rate += highest_unopened_rate;
        }

        if state.elephant.is_some() {
            if let Some(&(highest_unopened_rate, _)) = unopened.next() {
                rate += highest_unopened_rate;
            }
        }
    }

    released > max_released
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum Delta {
    Open { flow: i32, idx: u32 },
    MoveHuman(u32),
    MoveElephant(u32),
}

fn deltas(current: u32, human: bool, opened: Set<u64>, valves: &[(i32, Vec<u32>)]) -> Vec<Delta> {
    let (flow, ref tunnels) = valves[current as usize];

    let mut deltas = Vec::new();

    if flow > 0 && !opened.test_bit(current) {
        deltas.push(Delta::Open { flow, idx: current })
    }

    for &tunnel in tunnels {
        deltas.push(if human {
            Delta::MoveHuman(tunnel)
        } else {
            Delta::MoveElephant(tunnel)
        });
    }

    deltas
}

fn neighbours(state: &State, valves: &[(i32, Vec<u32>)]) -> Vec<State> {
    let mut states = Vec::new();

    if state.time == 0 {
        return states;
    }

    let mut next = *state;
    next.time -= 1;
    next.released += next.rate;

    if state.opened.count_ones() == valves.len() as u32 {
        states.push(next);
        return states;
    }

    let human_deltas = deltas(state.human, true, state.opened, valves);

    if let Some(elephant) = state.elephant {
        let elephant_deltas = deltas(elephant, false, state.opened, valves);

        for &human_delta in &human_deltas {
            for &elephant_delta in &elephant_deltas {
                if human_delta != elephant_delta {
                    states.push(next + human_delta + elephant_delta);
                }
            }
        }
    } else {
        for delta in human_deltas {
            states.push(next + delta);
        }
    }

    states
}

fn solve(human: u32, elephant: Option<u32>, time: u32, valves: &[(i32, Vec<u32>)]) -> i32 {
    let mut max_released = 0;

    let reachable = dfs_reach(
        State {
            human,
            elephant,
            time,
            ..State::default()
        },
        |state| {
            if !possible_route(state, valves, max_released) {
                return Vec::new();
            }

            max_released = max_released.max(state.released);

            neighbours(state, valves)
        },
    );

    for _ in reachable {}

    max_released
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
    let start_pos = valves
        .iter()
        .position(|&(valve, ..)| valve == "AA")
        .unwrap() as u32;

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

    println!("Part 1: {}", solve(start_pos, None, 30, &valves));
    println!("Part 2: {}", solve(start_pos, Some(start_pos), 26, &valves));
}
