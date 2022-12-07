use std::arch::x86_64::*;

pub fn part1_avx512(input: &[u8]) -> Option<usize> {
    assert!(input.len() >= 64);
    unsafe { solve_avx512::<4>(input) }
}

pub fn part2_avx512(input: &[u8]) -> Option<usize> {
    assert!(input.len() >= 64);
    unsafe { solve_avx512::<14>(input) }
}

unsafe fn solve_avx512<const N: usize>(input: &[u8]) -> Option<usize> {
    assert!(N >= 4);
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

        let mut neq = _mm512_cmpneq_epi8_mask(chunk, off_by_8);
        neq &= _mm512_cmpneq_epi8_mask(chunk, off_by_16);
        neq &= _mm512_cmpneq_epi8_mask(chunk, off_by_24);
        neq &= _mm512_cmpneq_epi8_mask(off_by_8, off_by_16);
        neq &= _mm512_cmpneq_epi8_mask(off_by_8, off_by_24);
        neq &= _mm512_cmpneq_epi8_mask(off_by_16, off_by_24);

        if neq != 0 {
            break;
        }

        chunk = next_chunk;
        i += 64;
    }

    let mut chunk = _mm512_castsi512_si128(chunk);

    while i + 32 <= len {
        let next_chunk = _mm_loadu_si128(ptr.add(i + 16).cast());

        let off_by_8 = _mm_alignr_epi8::<1>(next_chunk, chunk);
        let off_by_16 = _mm_alignr_epi8::<2>(next_chunk, chunk);
        let off_by_24 = _mm_alignr_epi8::<3>(next_chunk, chunk);

        let mut neq = _mm_cmpneq_epi8_mask(chunk, off_by_8);
        neq &= _mm_cmpneq_epi8_mask(chunk, off_by_16);
        neq &= _mm_cmpneq_epi8_mask(chunk, off_by_24);
        neq &= _mm_cmpneq_epi8_mask(off_by_8, off_by_16);
        neq &= _mm_cmpneq_epi8_mask(off_by_8, off_by_24);
        neq &= _mm_cmpneq_epi8_mask(off_by_16, off_by_24);

        if neq != 0 {
            break;
        }

        chunk = next_chunk;
        i += 16;
    }

    Some(solve_xor::<N>(&input[i..])? + i)
}

pub fn part1_xor(input: &[u8]) -> Option<usize> {
    solve_xor::<4>(input)
}

pub fn part2_xor(input: &[u8]) -> Option<usize> {
    solve_xor::<14>(input)
}

fn solve_xor<const N: usize>(bytes: &[u8]) -> Option<usize> {
    if bytes.len() < N {
        return None;
    }

    // flag = number of a byte value in the current window
    //    0 = zero or even
    //    1 = one or odd
    // current window contains no duplicates iff `flags.count_ones() == N`
    let mut flags = 0u32;
    for i in 0..N {
        // Subtracting with 96 (b'a' - 1) is no-op.
        // See also: comments in Day 3.
        flags ^= 1 << (bytes[i] - 96);
    }
    if flags.count_ones() == N as u32 {
        return Some(4);
    }

    let pos = bytes.windows(N + 1).position(|one_larger_window| {
        let &[out_byte, .., in_byte] = one_larger_window else {
            unreachable!();
        };
        flags ^= 1 << (out_byte - 96);
        flags ^= 1 << (in_byte - 96);
        flags.count_ones() == N as u32
    });
    pos.map(|i| i + N + 1)
}

pub fn part1_naive(input: &[u8]) -> Option<usize> {
    Some(input.windows(4).position(all_distinct)? + 4)
}

pub fn part2_naive(input: &[u8]) -> Option<usize> {
    Some(input.windows(14).position(all_distinct)? + 14)
}

fn all_distinct(bytes: &[u8]) -> bool {
    let mut flags = 0u32;
    for x in bytes {
        flags |= 1 << (x - 96);
    }
    flags.count_ones() == bytes.len() as u32
}

pub fn part1_naive_short_circuit(input: &[u8]) -> Option<usize> {
    Some(input.windows(4).position(all_distinct_short_circuit)? + 4)
}

pub fn part2_naive_short_circuit(input: &[u8]) -> Option<usize> {
    Some(input.windows(14).position(all_distinct_short_circuit)? + 14)
}

fn all_distinct_short_circuit(bytes: &[u8]) -> bool {
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
