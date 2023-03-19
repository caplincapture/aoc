
use core::fmt;
use std::cmp::Ordering;

use serde::Deserialize;

#[derive(Deserialize, Clone, PartialEq, Eq)]
#[serde(untagged)]
enum Node {
    Number(u64),
    List(Vec<Node>),
}

impl Node {
    fn with_slice<T>(&self, f: impl FnOnce(&[Node]) -> T) -> T {
        match self {
            Self::List(n) => f(&n[..]),
            Self::Number(n) => f(&[Self::Number(*n)]),
        }
    }
}

impl std::cmp::PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match (self, other) {
            (Node::Number(a), Node::Number(b)) => a.partial_cmp(b),
            (l, r) => l.with_slice(|l| r.with_slice(|r| l.partial_cmp(r))),
        }
    }
}

impl std::cmp::Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

impl fmt::Debug for Node {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Number(n) => write!(f, "{n}"),
            Self::List(n) => f.debug_list().entries(n).finish(),
        }
    }
}

fn main() {
    let dividers = vec![
        Node::List(vec![Node::Number(2)]),
        Node::List(vec![Node::Number(6)]),
    ];

    let mut packets = include_str!("../input.txt")
        .lines()
        .filter(|s| !s.is_empty())
        .map(|line| serde_json::from_str::<Node>(line).unwrap())
        .chain(dividers.iter().cloned())
        .collect::<Vec<_>>();

    packets.sort();

    let decoder_key = dividers
        .iter()
        .map(|d| packets.binary_search(d).unwrap() + 1)
        .product::<usize>();

    dbg!(decoder_key);
}