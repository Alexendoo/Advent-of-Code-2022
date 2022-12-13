#![feature(iter_array_chunks)]

use serde::Deserialize;
use serde_json::Deserializer;
use std::cmp::Ordering;
use std::iter::zip;
use Packet::{Int, List};

#[derive(Deserialize, Debug, PartialEq, Eq)]
#[serde(untagged)]
enum Packet {
    Int(u32),
    List(Vec<Packet>),
}

impl Ord for Packet {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (Int(l), Int(r)) => Ord::cmp(l, r),
            (List(ls), List(rs)) => zip(ls, rs)
                .map(|(l, r)| Ord::cmp(l, r))
                .find(|ordering| ordering.is_ne())
                .unwrap_or_else(|| ls.len().cmp(&rs.len())),
            (&Int(l), List(_)) => Ord::cmp(&List(vec![Int(l)]), other),
            (List(_), &Int(r)) => Ord::cmp(self, &List(vec![Int(r)])),
        }
    }
}

impl PartialOrd for Packet {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn main() {
    let mut packets: Vec<Packet> = Deserializer::from_str(include_str!("input"))
        .into_iter::<Packet>()
        .map(Result::unwrap)
        .collect();

    let sum: usize = packets
        .iter()
        .array_chunks()
        .enumerate()
        .map(|(i, [l, r])| if l < r { i + 1 } else { 0 })
        .sum();

    println!("Part 1: {sum}");

    packets.sort_unstable();

    let divider_a = List(vec![List(vec![Int(2)])]);
    let divider_b = List(vec![List(vec![Int(6)])]);

    let decoder_key = (packets.partition_point(|p| p < &divider_a) + 1)
        * (packets.partition_point(|p| p < &divider_b) + 2);

    println!("Part 2: {decoder_key}");
}
