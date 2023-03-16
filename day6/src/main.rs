// this is the feature we want:
#![feature(generic_const_exprs)]
// this is how we get rustc to stop shaming us for using it:
#![allow(incomplete_features)]

use std::time::{Duration, Instant};

// same as before
const SEQUENCE_SIZE: usize = 14;

// this defines a letter of the alphabet we can use with `State`
trait Letter {
    // the trait has an associated const value, which is the size of the
    // alphabet.
    const N: usize;

    fn to_usize(&self) -> usize;
}

// since it's our own trait, we can implement it on whatever type!
// (see "orphan rules" if you're unfamiliar)
impl Letter for u8 {
    const N: usize = 26;

    fn to_usize(&self) -> usize {
        assert!(self.is_ascii_lowercase());
        *self as usize - b'a' as usize
    }
}

struct State<L: Letter>
where
    // I learned this kind of bounds for this article! The compiler helped
    // me find half, and I had to do a web search for the other half
    [(); L::N]: Sized,
{
    data: [u8; L::N],
}

// I resent this impl, personally: `Default` is only implemented for [T; N] for
// small values of `N`. I suppose when more of the const generic stuff lands,
// we won't need this anymore
impl<L> Default for State<L>
where
    // and yes, we need to repeat those bounds on every impl block, that's
    // by design. it allows us to specify different impls with different bounds.
    // (I also find this annoying but so be it)
    L: Letter,
    [(); L::N]: Sized,
{
    fn default() -> Self {
        Self { data: [0; L::N] }
    }
}

impl<L> State<L>
where
    L: Letter,
    [(); L::N]: Sized,
{
    fn push(&mut self, c: u8) {
        // we check for overflow here! isn't that nice.
        self.data[c as usize] = self.data[c as usize].checked_add(1).unwrap();
    }

    fn pop(&mut self, c: u8) {
        // here too
        self.data[c as usize] = self.data[c as usize].checked_sub(1).unwrap();
    }

    fn is_unique(&self) -> bool {
        // I still really like this. it'll stop at the first one that's
        // > 1, too!
        self.data.iter().all(|&x| x <= 1)
    }
}

fn marker_pos(input: &str) -> Option<usize> {
    assert!(input.len() > SEQUENCE_SIZE);

    // we have to specify `u8` here to make rustc happy. oh well.
    let mut state = State::<u8>::default();
    input
        .bytes()
        .take(SEQUENCE_SIZE)
        .for_each(|c| state.push(c));
    if state.is_unique() {
        return Some(0);
    }

    // the logic itself hasn't changed, but thanks for reading all the comments
    for (index, window) in input.as_bytes().windows(SEQUENCE_SIZE + 1).enumerate() {
        let removed = window[0];
        let added = window[SEQUENCE_SIZE];

        state.pop(removed);
        state.push(added);

        if state.is_unique() {
            return Some(index + 1);
        }
    }
    None
}


fn main() {
    let start = Instant::now();
    dbg!(marker_pos(include_str!("input.txt")));
    let duration = start.elapsed();

    println!("Time elapsed in expensive_function() is: {:?}", duration);
}