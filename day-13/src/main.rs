#![feature(iter_array_chunks)]

use serde::Deserialize;
use serde_json::Deserializer;
use std::cmp::Ordering;
use std::iter::zip;

#[derive(Deserialize, Debug)]
#[serde(untagged)]
enum Packet {
    Int(u32),
    List(Vec<Packet>),
}

fn compare(left_packet: &Packet, right_packet: &Packet) -> Ordering {
    match (left_packet, right_packet) {
        (Packet::Int(l), Packet::Int(r)) => l.cmp(&r),
        (Packet::List(ls), Packet::List(rs)) => zip(ls, rs)
            .map(|(l, r)| compare(l, r))
            .find(|ordering| ordering.is_ne())
            .unwrap_or_else(|| ls.len().cmp(&rs.len())),
        (&Packet::Int(l), Packet::List(_)) => {
            compare(&Packet::List(vec![Packet::Int(l)]), right_packet)
        }
        (Packet::List(_), &Packet::Int(r)) => {
            compare(left_packet, &Packet::List(vec![Packet::Int(r)]))
        }
    }
}

fn main() {
    let pairs = Deserializer::from_str(include_str!("input"))
        .into_iter::<Packet>()
        .map(Result::unwrap)
        .array_chunks();

    let sum: usize = pairs
        .enumerate()
        .map(|(i, [l, r])| if compare(&l, &r).is_le() { i + 1 } else { 0 })
        .sum();

    println!("Part 1: {sum}");
}
