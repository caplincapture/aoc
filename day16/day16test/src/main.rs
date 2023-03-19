use std::collections::{hash_map::Entry, HashMap, HashSet};

use itertools::Itertools;
use nom::{combinator::all_consuming, Finish};
use parse::{Name, Valve};

mod parse;

use parse::NameMap;

type Connections = NameMap<(Path, Flow)>;

use std::sync::atomic::AtomicU64;

static MOVES_CALLED: AtomicU64 = AtomicU64::new(0);

// maps "valves open" to "best pressure achieved"
type Best = HashMap<NameMap<()>, u64>;

#[derive(Clone, Copy, PartialEq, Eq)]
struct Flow(u64);

pub struct Network {
    // was: `HashMap<Name, (Valve, Connections)>`
    valves: NameMap<(Valve, Connections)>,
}

pub type Path = Vec<(Name, Name)>;

impl Network {
    #![allow(clippy::new_without_default)]
    pub fn new() -> Self {
        let mut net = Self {
            valves: Default::default(),
        };

        // we'd have to implement `FromIterator` if we wanted to use `collect`,
        // let's just it do manually for now
        for valve in include_str!("../input.txt")
            .lines()
            .map(|l| all_consuming(parse::Valve::parse)(l).finish().unwrap().1)
        {
            net.valves.insert(valve.name, (valve, Default::default()));
        }

        let names = net.valves.keys().collect::<Vec<_>>();
        for name in names {
            let conns = net.connections(name);
            net.valves.get_mut(name).unwrap().1 = conns;
        }
        net
    }

    /// Given a valve name, return a list of valves we can travel to, along
    /// with the path to get there, and their flow.
    ///
    /// Only the shortest paths are considered, so the search ends.
    fn connections(&self, start: Name) -> Connections {
        // some light changes here
        let mut current = Connections::default();
        {
            let valve = &self.valves.get(start).unwrap().0;
            current.insert(start, (vec![], Flow(valve.flow)));
        }

        let mut connections = current.clone();

        while !current.is_empty() {
            // and here
            let mut next = Connections::default();
            for (name, (path, _flow)) in current.iter() {
                // can't use the nice `Entry` API anymore:
                for link in self.valves.get(name).unwrap().0.links.iter().copied() {
                    let valve = &self.valves.get(link).unwrap().0;
                    if !connections.contains(link) {
                        let conn_path: Path = path
                            .iter()
                            .copied()
                            .chain(std::iter::once((name, link)))
                            .collect();
                        let item = (conn_path.clone(), Flow(valve.flow));
                        connections.insert(link, item.clone());
                        next.insert(link, item);
                    }
                }
            }
            current = next;
        }

        connections
    }
}

#[derive(Debug, Clone)]
struct Move<'a> {
    reward: u64,
    target: Name,
    //    👇 now borrowed
    path: &'a Path,
}

impl Move<'_> {
    fn cost(&self) -> u64 {
        let travel_turns = self.path.len() as u64;
        let open_turns = 1_u64;
        travel_turns + open_turns
    }
}

#[derive(Clone)]
struct State<'a> {
    net: &'a Network,
    position: Name,
    max_turns: u64,
    turn: u64,
    pressure: u64,
    // 👇 was: HashSet<Name>
    open_valves: NameMap<()>,
}

impl State<'_> {
    fn apply(&self, mv: &Move) -> Self {
        let mut next = self.clone();
        next.position = mv.target;
        next.turn += mv.cost();
        next.pressure += mv.reward;
        // 👇 inserting an empty tuple here
        next.open_valves.insert(mv.target, ());
        next
    }

    fn turns_left(&self) -> u64 {
        self.max_turns - self.turn
    }

    /// Compute all moves and expected reward (pressure contributed till time
    /// runs out if we travel to it and open it now)
    fn moves(&self) -> impl Iterator<Item = Move> + '_ {
        // note: I removed the `MOVES_CALLED` business

        let (_valve, connections) = self.net.valves.get(self.position).unwrap();
        connections.iter().filter_map(|(name, (path, flow))| {
            if self.open_valves.contains(name) {
                return None;
            }

            if flow.0 == 0 {
                return None;
            }

            let travel_turns = path.len() as u64;
            let open_turns = 1_u64;
            let turns_spent_open = self.turns_left().checked_sub(travel_turns + open_turns)?;
            let reward = flow.0 * turns_spent_open;
            Some(Move {
                reward,
                target: name,
                path,
            })
        })
    }

    fn apply_best_moves(&self, best: &mut Best) -> Self {
        let mut best_state = self.clone();

        best.entry(self.open_valves.clone())
            .and_modify(|v| {
                if self.pressure > *v {
                    *v = self.pressure
                }
            })
            .or_insert(self.pressure);

        for mv in self.moves() {
            let next = self.apply(&mv).apply_best_moves(best);
            if next.pressure > best_state.pressure {
                best_state = next;
            }
        }
        best_state
    }
}

fn main() {
    let net = Network::new();
    let p1_state = State {
        net: &net,
        position: Name(*b"AA"),
        max_turns: 30,
        turn: 0,
        pressure: 0,
        open_valves: Default::default(),
    };

    {
        let mut best = Best::default();
        let state = p1_state.apply_best_moves(&mut best);
        println!("part 1 pressure = {}", state.pressure);
    }

    {
        let mut best = Best::default();
        let mut p2_state = p1_state.clone();
        p2_state.max_turns = 26;
        p2_state.apply_best_moves(&mut best);

        let best_pressure = best
            .iter()
            .tuple_combinations()
            .filter(|(human, elephant)| human.0.is_disjoint(elephant.0))
            .map(|(human, elephant)| human.1 + elephant.1)
            .max()
            .unwrap();
        println!("part 2 pressure: {best_pressure}");
    }
}