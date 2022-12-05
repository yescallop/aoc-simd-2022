#![feature(stdsimd)]

use std::{fs, io, time::Instant};

fn main() -> io::Result<()> {
    let input = fs::read_to_string("input/04.txt")?;
    let start = Instant::now();
    let ans1 = unsafe { aoc_simd::day_04::part1_simd(input.as_bytes()) };
    assert_eq!(ans1, 547);
    println!("{ans1}: {:?}", start.elapsed());
    Ok(())
}
