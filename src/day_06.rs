use std::arch::x86_64::*;

pub fn part1_simd(input: &[u8]) -> Option<usize> {
    assert!(input.len() >= 64);
    unsafe { _part1_simd(input) }
}

unsafe fn _part1_simd(input: &[u8]) -> Option<usize> {
    let len = input.len();
    let ptr = input.as_ptr();

    let mut chunk = _mm512_loadu_si512(ptr.cast());

    let mut i = 0;
    while i + 128 <= len {
        let next_chunk = _mm512_loadu_si512(ptr.add(i + 64).cast());
        let off_by_128 = _mm512_alignr_epi64::<2>(next_chunk, chunk);

        let off_by_8 = _mm512_alignr_epi8::<1>(off_by_128, chunk);
        let off_by_16 = _mm512_alignr_epi8::<2>(off_by_128, chunk);
        let off_by_24 = _mm512_alignr_epi8::<3>(off_by_128, chunk);

        let mut mask = 0;
        mask |= _mm512_cmpeq_epi8_mask(chunk, off_by_8);
        mask |= _mm512_cmpeq_epi8_mask(chunk, off_by_16);
        mask |= _mm512_cmpeq_epi8_mask(chunk, off_by_24);
        mask |= _mm512_cmpeq_epi8_mask(off_by_8, off_by_16);
        mask |= _mm512_cmpeq_epi8_mask(off_by_8, off_by_24);
        mask |= _mm512_cmpeq_epi8_mask(off_by_16, off_by_24);

        if mask != !0 {
            break;
        }

        chunk = next_chunk;
        i += 64;
    }
    Some(input[i..].windows(4).position(all_distinct)? + i + 4)
}

pub fn all_distinct(bytes: &[u8]) -> bool {
    let mut flags = 0u32;
    for x in bytes {
        let mask = 1 << (x - 96);
        if flags & mask != 0 {
            return false;
        }
        flags |= mask;
    }
    true
}

pub fn part1_naive(input: &[u8]) -> Option<usize> {
    Some(input.windows(4).position(all_distinct)? + 4)
}

pub fn part1_memoized(input: &[u8]) -> Option<usize> {
    solve_memoized::<4>(input)
}

// Original: https://github.com/orlp/aoc2022/blob/master/src/bin/day06.rs
fn solve_memoized<const N: usize>(bytes: &[u8]) -> Option<usize> {
    if bytes.len() < N {
        return None;
    }
    // In the current N-byte window:
    // The number of bytes with a certain value;
    let mut byte_cnt = [0u8; 256];
    // The number of distinct byte values.
    let mut value_cnt = 0;

    for i in 0..N {
        let in_cnt = &mut byte_cnt[bytes[i] as usize];
        value_cnt += (*in_cnt == 0) as usize;
        *in_cnt += 1;
    }

    let pos = bytes.windows(N + 1).position(|window| {
        let &[out_byte, .., in_byte] = window else {
            unreachable!();
        };

        let out_cnt = &mut byte_cnt[out_byte as usize];
        *out_cnt -= 1;
        value_cnt -= (*out_cnt == 0) as usize;

        let in_cnt = &mut byte_cnt[in_byte as usize];
        value_cnt += (*in_cnt == 0) as usize;
        *in_cnt += 1;

        value_cnt == N
    });
    pos.map(|i| i + N + 1)
}
