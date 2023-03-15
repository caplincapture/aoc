use std::ops::RangeInclusive;

use itertools::Itertools;

use std::time::{Duration, Instant};

trait InclusiveRangeExt {
    fn contains_range(&self, other: &Self) -> bool;

    // 👋 new! we can have trait methods with default implementations
    fn contains_or_is_contained(&self, other: &Self) -> bool {
        self.contains_range(other) || other.contains_range(self)
    }
}

impl<T> InclusiveRangeExt for RangeInclusive<T>
where
    T: PartialOrd,
{
    fn contains_range(&self, other: &Self) -> bool {
        self.contains(other.start()) && self.contains(other.end())
    }
}

fn main() {
    let start = Instant::now();
    let redundant = include_str!("input.txt")
        .lines()
        .map(|l| {
            l.split(',')
                .map(|range| {
                    range
                        .split('-')
                        .map(|n| n.parse().expect("range start/end should be u32"))
                        .collect_tuple::<(u32, u32)>()
                        .map(|(start, end)| start..=end)
                        .expect("each range should have a start and end")
                })
                .collect_tuple::<(_, _)>()
                .expect("each line must have a pair of ranges")
        })
        .filter(|(a, b)| a.contains_or_is_contained(b))
        .count();
    let duration = start.elapsed();

    println!("Time elapsed in expensive_function() is: {:?}", duration);
    dbg!(redundant);
}