#![feature(stdsimd)]
#![allow(unused)]

pub mod day_04;

use std::fmt::Debug;

use bytemuck::{bytes_of, AnyBitPattern, NoUninit};

fn print_bytes_of<T: NoUninit>(t: &T) {
    let bytes = bytemuck::bytes_of(t);
    print_bytes(&bytes);
}

fn print_as_slice<B: AnyBitPattern + Debug>(a: &impl NoUninit) {
    let bytes = bytemuck::bytes_of(a);
    let slice: &[B] = bytemuck::cast_slice(bytes);
    println!("{slice:?}");
}

fn print_bytes(bytes: &[u8]) {
    for &(mut b) in bytes {
        if b == b'\n' {
            b = b'\\';
        } else if b == 0 {
            b = b'/';
        }
        print!("{}", b as char)
    }
    println!();
}
