use std::arch::x86_64::*;

#[repr(C, align(16))]
struct Buf([u8; 64 + 32]);

pub unsafe fn part1_simd(input: &[u8]) -> u64 {
    let input_len = input.len();
    let ptr = input.as_ptr();
    assert!(input_len >= 16 && ptr as usize % 16 == 0);

    let mut buf = Buf([0u8; 64 + 32]);
    let buf_ptr = &mut buf as *mut Buf as *mut u8;
    let mut buf_len = 0;

    if input[1] == b'-' {
        buf.0[0] = b'0';
        buf_len += 1;
    }

    let ascii_zero = _mm_set1_epi8(b'0' as i8);
    let shuf_ctrl = _mm_set_epi8(14, 15, 12, 13, 6, 7, 4, 5, 10, 11, 8, 9, 2, 3, 0, 1);
    let one_i64x2 = _mm_set1_epi64x(1);

    let mut chunk = _mm_load_si128(ptr.cast());
    let mut sum1 = _mm_setzero_si128();

    // 24-91,80-92\n28-93,5-94\n
    let mut consume_buf_u16x8 = |offset| {
        // 2491809228930594
        let buf = _mm_load_si128(buf_ptr.add(offset).cast());
        // 4219823908295049
        let shuf1 = _mm_shuffle_epi8(buf, shuf_ctrl);
        // 0829504908295049
        let shuf2 = _mm_shuffle_epi32::<0b11101110>(shuf1);
        // [-1532, -1, 515, -1, 0, 0, 0, 0]
        let sub = _mm_sub_epi16(shuf1, shuf2);
        // [-1532, -1, 515, -1]
        let sign_ext = _mm_cvtepi16_epi32(sub);
        // [-1, -1, -1, -1]
        let shuf3 = _mm_shuffle_epi32::<0b11110101>(sign_ext);
        // [1532, -515]
        let mul = _mm_mul_epi32(sign_ext, shuf3);
        // [0, -1]
        let cmp = _mm_cmpgt_epi64(one_i64x2, mul);
        sum1 = _mm_sub_epi64(sum1, cmp);
    };

    let mut i = 0;
    while i + 32 <= input_len {
        let next_chunk = _mm_load_si128(ptr.add(i + 16).cast());
        let off_by_two = _mm_alignr_epi8::<2>(next_chunk, chunk);
        let lucky_or = _mm_or_si128(chunk, off_by_two);

        let to_fill = _mm_cmplt_epi8_mask(lucky_or, ascii_zero);
        let filled = _mm_mask_blend_epi8(to_fill, chunk, ascii_zero);

        let nums = _mm_cmpge_epi8_mask(filled, ascii_zero);
        _mm_mask_compressstoreu_epi8(buf_ptr.add(buf_len), nums, filled);

        chunk = next_chunk;

        buf_len += nums.count_ones() as usize;

        if buf_len >= 64 {
            consume_buf_u16x8(0);
            consume_buf_u16x8(16);
            consume_buf_u16x8(32);
            consume_buf_u16x8(48);

            let rem = _mm_load_si128(buf_ptr.add(64).cast());
            _mm_store_si128(buf_ptr.cast(), rem);
            buf_len -= 64;
        }
        i += 16;
    }

    while i < input_len {
        let mut x = input[i];
        let y = input.get(i + 2).copied().unwrap_or(0);
        if x | y < b'0' {
            x = b'0';
        }
        if x >= b'0' {
            *buf_ptr.add(buf_len) = x;
            buf_len += 1;
        }
        i += 1;
    }

    let mut buf_i = 0;
    while buf_i + 16 <= buf_len {
        consume_buf_u16x8(buf_i);
        buf_i += 16;
    }
    if buf_i + 8 <= buf_len {
        *buf_ptr.add(buf_i + 8).cast() = u64::from_le_bytes(*b"00001111");
        consume_buf_u16x8(buf_i);
    }

    hsum_i64x2(sum1) as u64
}

unsafe fn hsum_i64x2(x: __m128i) -> i64 {
    let hi64 = _mm_unpackhi_epi64(x, x);
    let sum64 = _mm_add_epi64(x, hi64);
    _mm_cvtsi128_si64(sum64)
}
