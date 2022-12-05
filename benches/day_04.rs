#![feature(test)]

extern crate test;

use std::{fs, io};

use aoc_simd::day_04::*;
use test::Bencher;

#[bench]
fn d4p1_simd(b: &mut Bencher) -> io::Result<()> {
    let input = fs::read_to_string("input/04.txt")?;
    b.iter(|| unsafe { part1_simd(input.as_bytes()) });
    Ok(())
}

#[bench]
fn d4p1_naive(b: &mut Bencher) -> io::Result<()> {
    let input = fs::read_to_string("input/04.txt")?;
    b.iter(|| part1_naive(&input));
    Ok(())
}

pub fn part1_naive(input: &str) -> u64 {
    fn range(s: &str) -> Option<(u32, u32)> {
        let (l, r) = s.split_once('-')?;
        l.parse().ok().zip(r.parse().ok())
    }
    fn contains(a: (u32, u32), b: (u32, u32)) -> bool {
        let (x, y) = (b.0 as i32 - a.0 as i32, b.1 as i32 - a.1 as i32);
        x * y <= 0
    }

    let mut cnt = 0;
    for line in input.lines() {
        let (l, r) = line.split_once(',').unwrap();
        let (l, r) = range(l).zip(range(r)).unwrap();
        cnt += contains(l, r) as u64;
    }

    cnt
}
