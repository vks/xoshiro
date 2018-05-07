use std::arch::x86_64::*;

fn rotl(x: u32, k: u32) -> u32 {
    (x << k) | (x >> (32 - k))
}


fn to_i32(x: u32) -> i32 {
    unsafe { std::mem::transmute(x) }
}

fn to_u32(x: i32) -> u32 {
    unsafe { std::mem::transmute(x) }
}

fn starstar(s0: u32) -> u32 {
    rotl(s0.wrapping_mul(5), 7).wrapping_mul(9)
}

pub struct Xoshiro128 {
    s: [u32; 4],
}

impl Xoshiro128 {
    #[inline]
    pub fn from_seed(s: [u32; 4]) -> Xoshiro128 {
        Xoshiro128 { s }
    }

    #[inline]
    pub fn next(&mut self) -> u32 {
        let result_starstar = starstar(self.s[0]);
        let t = self.s[1] << 9;

        /*
        // This code is closer to the SIMD implementation.
        // It is slightly inefficient, because reordering the assignments
        // yields the same results with less instructions. However, this is
        // closer to the SIMD algorithm and might be optimized away by the
        // compiler.
        fn shuffle(s: [u32; 4]) -> [u32; 4] { [s[3], s[2], s[0], s[1]] }

        fn xor(a: [u32; 4], b: [u32; 4]) -> [u32; 4] {
            [a[0] ^ b[0], a[1] ^ b[1], a[2] ^ b[2], a[3] ^ b[3]]
        }

        let s0 = self.s[0];
        let s1 = self.s[1];
        self.s = xor(self.s, shuffle(self.s));
        self.s[0] ^= s1;
        self.s[1] ^= s0;
        */

        // This is closer to the reference implementation.
        self.s[2] ^= self.s[0];
        self.s[3] ^= self.s[1];
        self.s[1] ^= self.s[2];
        self.s[0] ^= self.s[3];

        self.s[2] ^= t;

        self.s[3] = rotl(self.s[3], 11);

        result_starstar
    }
}

pub struct Xoshiro128SIMD {
    s: __m128i,
}

impl Xoshiro128SIMD {
    #[inline]
    pub fn from_seed(s: [u32; 4]) -> Xoshiro128SIMD {
        unsafe {
            Xoshiro128SIMD {
                s: _mm_set_epi32(to_i32(s[0]), to_i32(s[1]),
                                 to_i32(s[2]), to_i32(s[3]))
            }
        }
    }

    #[inline]
    pub fn next(&mut self) -> u32 {
        unsafe {
            let s0 = _mm_extract_epi32(self.s, 3);
            let s1 = _mm_extract_epi32(self.s, 2);
            let result_starstar = starstar(to_u32(s0));
            let shifted = to_u32(s1) << 9;
            let t = _mm_set_epi32(0, 0, to_i32(shifted), 0);
            let shuffled = _mm_shuffle_epi32(self.s, 0b00_01_11_10);
            self.s = _mm_xor_si128(self.s, shuffled);
            self.s = _mm_xor_si128(self.s, _mm_set_epi32(s1, s0, 0, 0));
            self.s = _mm_xor_si128(self.s, t);
            let s3 = to_u32(_mm_extract_epi32(self.s, 0));
            let u = _mm_set_epi32(0, 0, 0, to_i32(rotl(s3, 11)));
            self.s = _mm_blend_epi32(self.s, u, 0b0001); // this requires avx2 :(

            result_starstar
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_xoshiro() {
        let mut rng = Xoshiro128::from_seed([1, 2, 3, 4]);
        // These values were produced with the reference implementation:
        // http://xoshiro.di.unimi.it/xoshiro128starstar.c
        let expected = [
            5760, 40320, 70819200, 3297914139, 2480851620, 1792823698,
            4118739149, 1251203317, 1581886583, 1721184582,
        ];
        for &e in &expected {
            assert_eq!(rng.next(), e);
        }
    }

    #[test]
    fn test_xoshiro_simd() {
        let mut rng1 = Xoshiro128::from_seed([1, 2, 3, 4]);
        let mut rng2 = Xoshiro128SIMD::from_seed([1, 2, 3, 4]);
        for _ in 0..100 {
            assert_eq!(rng1.next(), rng2.next());
        }
    }
}
