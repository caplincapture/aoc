
use std::fmt;

use nom::{
    branch::alt,
    bytes::complete::{tag, take, take_while1},
    combinator::{all_consuming, map, opt, map_res},
    sequence::{delimited, preceded, tuple},
    Finish, IResult,
};

use std::alloc::System;
use tracking_allocator::{
    AllocationGroupId, AllocationGroupToken, AllocationRegistry, AllocationTracker, Allocator,
};

#[derive(Clone, Copy)]
struct Crate(char);

impl fmt::Debug for Crate {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl fmt::Display for Crate {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Debug::fmt(self, f)
    }
}

fn parse_crate(i: &str) -> IResult<&str, Crate> {
    let first_char = |s: &str| Crate(s.chars().next().unwrap());
    let f = delimited(tag("["), take(1_usize), tag("]"));
    map(f, first_char)(i)
}

fn parse_hole(i: &str) -> IResult<&str, ()> {
    // `drop` takes a value and returns nothing, which is
    // perfect for our case
    map(tag("   "), drop)(i)
}

fn parse_crate_or_hole(i: &str) -> IResult<&str, Option<Crate>> {
    alt((map(parse_crate, Some), map(parse_hole, |_| None)))(i)
}

fn parse_crate_line(i: &str) -> IResult<&str, Vec<Option<Crate>>> {
    let (mut i, c) = parse_crate_or_hole(i)?;
    let mut v = vec![c];

    loop {
        let (next_i, maybe_c) = opt(preceded(tag(" "), parse_crate_or_hole))(i)?;
        match maybe_c {
            Some(c) => v.push(c),
            None => break,
        }
        i = next_i;
    }

    Ok((i, v))
}

fn transpose_rev<T>(v: Vec<Vec<Option<T>>>) -> Vec<Vec<T>> {
    assert!(!v.is_empty());
    let len = v[0].len();
    let mut iters: Vec<_> = v.into_iter().map(|n| n.into_iter()).collect();
    (0..len)
        .map(|_| {
            let mut v = Vec::with_capacity(256); // just to be on the safe side
            v.extend(iters.iter_mut().rev().filter_map(|n| n.next().unwrap()));
            v
        })
        .collect()
}

fn parse_number(i: &str) -> IResult<&str, usize> {
    map_res(take_while1(|c: char| c.is_ascii_digit()), |s: &str| {
        s.parse::<usize>()
    })(i)
}

fn parse_pile_number(i: &str) -> IResult<&str, usize> {
    map(parse_number, |i| i - 1)(i)
}

#[derive(Debug)]
struct Instruction {
    quantity: usize,
    src: usize,
    dst: usize,
}

fn parse_instruction(i: &str) -> IResult<&str, Instruction> {
    map(
        tuple((
            preceded(tag("move "), parse_number),
            preceded(tag(" from "), parse_pile_number),
            preceded(tag(" to "), parse_pile_number),
        )),
        |(quantity, src, dst)| Instruction { quantity, src, dst },
    )(i)
}

use std::cell::RefCell;

struct Piles(Vec<RefCell<Vec<Crate>>>);

impl fmt::Debug for Piles {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for (i, pile) in self.0.iter().enumerate() {
            writeln!(f, "Pile {}: {:?}", i, pile)?;
        }
        Ok(())
    }
}

impl Piles {
    fn apply(&mut self, ins: Instruction) {
        // N.B: `crate` is a keyword
        for krate in (0..ins.quantity)
            .map(|_| self.0[ins.src].borrow_mut().pop().unwrap())
            .collect::<Vec<_>>()
            .into_iter()
            .rev()
        {
            self.0[ins.dst].borrow_mut().push(krate);
        }
    }
}

pub trait BorrowTwoMut<T> {
    fn borrow_two_mut(&mut self, a: usize, b: usize) -> (&mut T, &mut T);
}

impl<T> BorrowTwoMut<T> for [T] {
    fn borrow_two_mut(&mut self, a: usize, b: usize) -> (&mut T, &mut T) {
        assert!(a != b);
        unsafe {
            let ptr = self.as_mut_ptr();
            let a_ptr = ptr.add(a);
            let b_ptr = ptr.add(b);
            (&mut *a_ptr, &mut *b_ptr)
        }
    }
}

#[global_allocator]
static GLOBAL: tracking_allocator::Allocator<std::alloc::System> =
    tracking_allocator::Allocator::system();
static alloc: Allocator<System> = Allocator::system();

struct StdoutTracker;

impl AllocationTracker for StdoutTracker {

    fn allocated(
        &self,
        addr: usize,
        object_size: usize,
        wrapped_size: usize,
        group_id: AllocationGroupId,
    ) {
        println!(
            "allocation -> addr=0x{:0x} object_size={} wrapped_size={} group_id={:?}",
            addr, object_size, wrapped_size, group_id
        );
        // 👇
        panic!();
    }

    

    fn deallocated(
        &self,
        addr: usize,
        object_size: usize,
        wrapped_size: usize,
        source_group_id: AllocationGroupId,
        current_group_id: AllocationGroupId,
    ) {
        println!(
            "deallocation -> addr=0x{:0x} object_size={} wrapped_size={} source_group_id={:?} current_group_id={:?}",
            addr, object_size, wrapped_size, source_group_id, current_group_id
        );
    }
}

fn main() {
    let mut lines = include_str!("input.txt").lines();

    let crate_lines: Vec<_> = (&mut lines)
        .map_while(|line| {
            all_consuming(parse_crate_line)(line)
                .finish()
                .ok()
                .map(|(_, line)| line)
        })
        .collect();

    let mut piles = Piles(
        transpose_rev(crate_lines)
            .into_iter()
            .map(RefCell::new)
            .collect(),
    );
    println!("{piles:?}");
    
        // then, later
    println!(
        "answer = {}",
        piles
        .0
        .iter()
        .map(|pile| *pile.borrow().last().unwrap())
        .join("")
    );
    println!("{piles:?}");

    // we've consumed the "numbers line" but not the separating line
    assert!(lines.next().unwrap().is_empty());

    AllocationRegistry::set_global_tracker(StdoutTracker)
        .expect("no other global tracker should be set yet");

    AllocationRegistry::enable_tracking();

    for ins in lines.map(|line| all_consuming(parse_instruction)(line).finish().unwrap().1) {
        println!("{ins:?}");
        piles.apply(ins);
        println!("{piles:?}");
    }

    AllocationRegistry::disable_tracking();
}