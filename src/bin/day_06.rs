#![feature(stdsimd)]

use std::{fs, io};

fn main() -> io::Result<()> {
    let input = fs::read_to_string("input/06.txt")?;
    let ans1 = aoc_simd::day_06::part1_simd(input.as_bytes());
    assert_eq!(ans1, Some(1794));
    println!("{ans1:?}");
    Ok(())
}
