use itertools::Itertools;
use std::time::{Duration, Instant};

#[cfg(test)]
mod tests {
    use crate::find_marker;
    use test_case::test_case;
    
    #[test_case(7, "mjqjpqmgbljsphdztnvjfqwrcgsmlb")]
    #[test_case(5, "bvwbjplbgvbhsrlpgdmjqwftvncz")]
    #[test_case(6, "nppdvjthqldpwncqszvftbrmjlhg")]
    #[test_case(10, "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg")]
    #[test_case(11, "zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw")]
    fn test_find_marker(index: usize, input: &str) {
        assert_eq!(Some(index), find_marker(input));
    }
}

trait LowercaseLetter {
    fn to_u32_for_bitset(&self) -> u32;
}

impl LowercaseLetter for u8 {
    fn to_u32_for_bitset(&self) -> u32 {
        assert!(self.is_ascii_lowercase());
        1 << (*self as u32 - 'a' as u32)
    }
}

use std::collections::HashSet;

fn find_marker(input: &str) -> Option<usize> {
    input
        .as_bytes()
        .windows(4)
        .position(|window| {
            // this is allocated on the stack, it's fine
            let mut arr = [false; 256];
            for &item in window {
                if arr[item as usize] {
                    return false;
                }
                arr[item as usize] = true;
            }
            true
        })
        .map(|pos| pos + 4)
}

fn main() {
    let start = Instant::now();
    dbg!(find_marker(include_str!("input.txt")));
    let duration = start.elapsed();

    println!("Time elapsed in expensive_function() is: {:?}", duration);
}