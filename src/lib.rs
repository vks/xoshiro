use std::arch::x86_64::*;

fn rotl(x: u32, k: u32) -> u32 {
    (x << k) | (x >> (32 - k))
}

fn shuffle(s: [u32; 4]) -> [u32; 4] {
    [s[3], s[2], s[0], s[1]]
}

fn xor(a: [u32; 4], b: [u32; 4]) -> [u32; 4] {
    [a[0] ^ b[0], a[1] ^ b[1], a[2] ^ b[2], a[3] ^ b[3]]
}

fn to_i32(x: u32) -> i32 {
    unsafe { std::mem::transmute(x) }
}

fn to_u32(x: i32) -> u32 {
    unsafe { std::mem::transmute(x) }
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
        let result_starstar = rotl(self.s[0].wrapping_mul(5), 7).wrapping_mul(9);
        let t = [0, 0, self.s[1] << 9, 0];
        self.s = xor(self.s, shuffle(self.s));
        self.s = xor(self.s, t);
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
            let s0 = to_u32(_mm_extract_epi32(self.s, 3));
            let result_starstar = rotl(s0.wrapping_mul(5), 7).wrapping_mul(9);
            let shifted = to_u32(_mm_extract_epi32(self.s, 2)) << 9;
            let t = _mm_set_epi32(0, 0, to_i32(shifted), 0);
            let shuffled = _mm_shuffle_epi32(self.s, 0b00_01_11_10);
            self.s = _mm_xor_si128(self.s, shuffled);
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
    use super::{Xoshiro128, Xoshiro128SIMD};

    #[test]
    fn test_xoshiro() {
        let mut rng = Xoshiro128::from_seed([1, 2, 3, 4]);
        assert_eq!(rng.next(), 5760);
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
