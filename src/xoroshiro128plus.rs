use rand_core;
use rand_core::le::read_u64_into;
use rand_core::impls::fill_bytes_via_next;
use rand_core::{RngCore, SeedableRng};

use super::SplitMix64;

/// A Xoroshiro128Plus+ random number generator.
///
/// The Xoroshiro128Plus+ algorithm is not suitable for cryptographic purposes, but
/// is very fast and has better statistical properties than `XorShiftRng`.  If
/// you do not know for sure that it fits your requirements, use a more secure
/// one such as `IsaacRng` or `OsRng`.
///
/// The algorithm used here is translated from [the `Xoroshiro128Plusplus.c`
/// reference source code](http://xoshiro.di.unimi.it/Xoroshiro128Plusplus.c) by
/// David Blackman and Sebastiano Vigna.
#[allow(missing_copy_implementations)]
#[derive(Debug, Clone)]
pub struct Xoroshiro128Plus {
    s0: u64,
    s1: u64,
}

impl Xoroshiro128Plus {
    pub fn from_seed_u64(seed: u64) -> Xoroshiro128Plus {
        let mut rng = SplitMix64::from_seed_u64(seed);
        Xoroshiro128Plus::from_rng(&mut rng).unwrap()
    }

    /// Jump forward, equivalently to 2^64 calls to `next_u64()`.
    ///
    /// This can be used to generate 2^64 non-overlapping subsequences for
    /// parallel computations.
    ///
    /// ```
    /// # extern crate rand;
    /// # extern crate xoshiro;
    /// # fn main() {
    /// use rand::SeedableRng;
    /// use xoshiro::Xoroshiro128Plus;
    ///
    /// let rng1 = Xoroshiro128Plus::from_seed_u64(0);
    /// let mut rng2 = rng1.clone();
    /// rng2.jump();
    /// let mut rng3 = rng2.clone();
    /// rng3.jump();
    /// # }
    /// ```
    pub fn jump(&mut self) {
        const JUMP: [u64; 2] = [0xdf900294d8f554a5, 0x170865df4b3201fc];
        let mut s0 = 0;
        let mut s1 = 0;
        for j in &JUMP {
            for b in 0..64 {
                if (j & 1 << b) != 0 {
                    s0 ^= self.s0;
                    s1 ^= self.s1;
                }
                self.next_u64();
            }
        }
        self.s0 = s0;
        self.s1 = s1;
    }
}

impl RngCore for Xoroshiro128Plus {
    #[inline]
    fn next_u32(&mut self) -> u32 {
        // The two lowest bits have some linear dependencies, so we use the
        // upper bits instead.
        (self.next_u64() >> 32) as u32
    }

    #[inline]
    fn next_u64(&mut self) -> u64 {
        let r = self.s0.wrapping_add(self.s1);
        self.s1 ^= self.s0;
        self.s0 = self.s0.rotate_left(24) ^ self.s1 ^ (self.s1 << 16);
        self.s1 = self.s1.rotate_left(37);
        r
    }

    #[inline]
    fn fill_bytes(&mut self, dest: &mut [u8]) {
        fill_bytes_via_next(self, dest);
    }

    #[inline]
    fn try_fill_bytes(&mut self, dest: &mut [u8]) -> Result<(), rand_core::Error> {
        self.fill_bytes(dest);
        Ok(())
    }
}

impl SeedableRng for Xoroshiro128Plus {
    type Seed = [u8; 16];

    /// Create a new `Xoroshiro128Plus`.  This will panic if `seed` is entirely 0.
    fn from_seed(seed: [u8; 16]) -> Xoroshiro128Plus {
        assert!(seed != [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            "Xoroshiro128Plus::from_seed called with an all zero seed.");
        let mut s = [0; 2];
        read_u64_into(&seed, &mut s);

        Xoroshiro128Plus {
            s0: s[0],
            s1: s[1],
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reference() {
        let mut rng = Xoroshiro128Plus::from_seed(
            [1, 0, 0, 0, 0, 0, 0, 0, 2, 0, 0, 0, 0, 0, 0, 0]);
        // These values were produced with the reference implementation:
        // http://xoshiro.di.unimi.it/xoshiro128starstar.c
        let expected = [
            3, 412333834243, 2360170716294286339, 9295852285959843169,
            2797080929874688578, 6019711933173041966, 3076529664176959358,
            3521761819100106140, 7493067640054542992, 920801338098114767,
        ];
        for &e in &expected {
            assert_eq!(rng.next_u64(), e);
        }
    }
}
