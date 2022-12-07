#![feature(test)]

extern crate test;

use std::{fs, io};

use aoc_simd::day_06::*;
use test::Bencher;

#[bench]
fn d6p1_simd(b: &mut Bencher) -> io::Result<()> {
    let input = fs::read_to_string("input/06.txt")?;
    b.iter(|| part1_simd(input.as_bytes()));
    Ok(())
}

#[bench]
fn d6p1_naive(b: &mut Bencher) -> io::Result<()> {
    let input = fs::read_to_string("input/06.txt")?;
    b.iter(|| part1_naive(input.as_bytes()));
    Ok(())
}

#[bench]
fn d6p1_memoized(b: &mut Bencher) -> io::Result<()> {
    let input = fs::read_to_string("input/06.txt")?;
    b.iter(|| part1_memoized(input.as_bytes()));
    Ok(())
}
