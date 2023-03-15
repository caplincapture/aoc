pub fn main() {
    use std::time::{Duration, Instant};
    let start = Instant::now();
    println!(
        "{}",
        include_bytes!("input.txt")
            .split(|b| *b == b'\n')
            .map(|l| ((l[0] - b'A') as i16, (l[2] - b'X') as i16,))
            .map(|(a, b)| 1 + b * 3 + (2 + a + b) % 3)
            .sum::<i16>(),
    );
    
    let duration = start.elapsed();

    println!("Time elapsed in expensive_function() is: {:?}", duration);
}