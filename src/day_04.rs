use std::arch::x86_64::*;

#[repr(C, align(64))]
struct Buf([u8; 64 * 6]);

pub fn part1_simd(input: &[u8]) -> u64 {
    assert!(input.as_ptr() as usize % 2 == 0);
    unsafe { _part1_simd(input) }
}

unsafe fn _part1_simd(input: &[u8]) -> u64 {
    let input_len = input.len();
    let ptr = input.as_ptr();

    let mut buf = Buf([0u8; 64 * 6]);
    let buf_ptr = &mut buf as *mut Buf as *mut u8;
    let mut buf_len = 0;

    if input[1] == b'-' {
        buf.0[0] = b'0';
        buf_len += 1;
    }

    let ascii_zero = _mm512_set1_epi8(b'0' as i8);
    let shuf_ctrl = _mm_set_epi8(14, 15, 12, 13, 6, 7, 4, 5, 10, 11, 8, 9, 2, 3, 0, 1);
    let shuf_ctrl512 = _mm512_broadcast_i32x4(shuf_ctrl);
    let one_i64x2 = _mm_set1_epi64x(1);
    let one_i64x8 = _mm512_set1_epi64(1);

    let mut sum1 = _mm512_setzero_si512();

    // 24-91,80-92\n28-93,5-94\n
    let consume_buf_u16x8 = |offset, sum1: &mut __m512i| {
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
        let cmp = _mm_cmpgt_epi64_mask(one_i64x2, mul);
        *sum1 = _mm512_mask_add_epi64(*sum1, cmp, *sum1, one_i64x8);
    };

    let consume_buf_u16x32 = |offset, sum1: &mut __m512i| {
        let buf = _mm512_load_si512(buf_ptr.add(offset).cast());
        let shuf1 = _mm512_shuffle_epi8(buf, shuf_ctrl512);
        let shuf2 = _mm512_shuffle_epi32::<0b11101110>(shuf1);
        let sub = _mm512_sub_epi16(shuf1, shuf2);
        let perm = _mm512_permutex_epi64::<0b11011000>(sub);
        let shuf3 = _mm512_shuffle_i64x2::<0b11011000>(perm, perm);
        let sign_ext = _mm512_cvtepi16_epi32(_mm512_castsi512_si256(shuf3));
        let shuf4 = _mm512_shuffle_epi32::<0b11110101>(sign_ext);
        let mul = _mm512_mul_epi32(sign_ext, shuf4);
        let cmp = _mm512_cmpgt_epi64_mask(one_i64x8, mul);
        *sum1 = _mm512_mask_add_epi64(*sum1, cmp, *sum1, one_i64x8);
    };

    let mut i = 0;
    while i + 64 + 2 <= input_len {
        let sup: u16 = *ptr.add(i + 64).cast();
        let sup = _mm512_castsi128_si512(_mm_cvtsi32_si128(sup as i32));

        let chunk = _mm512_loadu_si512(ptr.add(i).cast());
        let off_by_128 = _mm512_alignr_epi64::<2>(sup, chunk);
        let off_by_16 = _mm512_alignr_epi8::<2>(off_by_128, chunk);
        let lucky_or = _mm512_or_si512(chunk, off_by_16);

        let to_fill = _mm512_cmplt_epi8_mask(lucky_or, ascii_zero);
        let filled = _mm512_mask_blend_epi8(to_fill, chunk, ascii_zero);

        let nums = _mm512_cmpge_epi8_mask(filled, ascii_zero);
        _mm512_mask_compressstoreu_epi8(buf_ptr.add(buf_len), nums, filled);

        buf_len += nums.count_ones() as usize;

        if buf_len >= 256 {
            consume_buf_u16x32(0, &mut sum1);
            consume_buf_u16x32(64, &mut sum1);
            consume_buf_u16x32(128, &mut sum1);
            consume_buf_u16x32(192, &mut sum1);

            let rem = _mm512_load_si512(buf_ptr.add(256).cast());
            _mm512_store_si512(buf_ptr.cast(), rem);
            buf_len -= 256;
        }
        i += 64;
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
        consume_buf_u16x8(buf_i, &mut sum1);
        buf_i += 16;
    }
    if buf_i + 8 <= buf_len {
        *buf_ptr.add(buf_i + 8).cast() = u64::from_le_bytes(*b"00001111");
        consume_buf_u16x8(buf_i, &mut sum1);
    }

    _mm512_reduce_add_epi64(sum1) as u64
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
