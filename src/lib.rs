extern crate rand_core;

use rand_core::impls::{next_u64_via_u32, fill_bytes_via_next};
use rand_core::le::read_u32_into;
use rand_core::{SeedableRng, RngCore, Error};

fn rotl(x: u32, k: u32) -> u32 {
    (x << k) | (x >> (32 - k))
}

fn starstar(s0: u32) -> u32 {
    rotl(s0.wrapping_mul(5), 7).wrapping_mul(9)
}

#[derive(Debug, Clone)]
pub struct Xoshiro128StarStar {
    s: [u32; 4],
}

impl SeedableRng for Xoshiro128StarStar {
    type Seed = [u8; 16];

    #[inline]
    fn from_seed(seed: [u8; 16]) -> Xoshiro128StarStar {
        let mut state = [0; 4];
        read_u32_into(&seed, &mut state);
        Xoshiro128StarStar { s: state }
    }
}

impl RngCore for Xoshiro128StarStar {
    #[inline]
    fn next_u32(&mut self) -> u32 {
        let result_starstar = starstar(self.s[0]);
        let t = self.s[1] << 9;

        self.s[2] ^= self.s[0];
        self.s[3] ^= self.s[1];
        self.s[1] ^= self.s[2];
        self.s[0] ^= self.s[3];

        self.s[2] ^= t;

        self.s[3] = rotl(self.s[3], 11);

        result_starstar
    }

    #[inline]
    fn next_u64(&mut self) -> u64 {
        next_u64_via_u32(self)
    }

    #[inline]
    fn fill_bytes(&mut self, dest: &mut [u8]) {
        fill_bytes_via_next(self, dest);
    }

    #[inline]
    fn try_fill_bytes(&mut self, dest: &mut [u8]) -> Result<(), Error> {
        self.fill_bytes(dest);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_xoshiro() {
        let mut rng = Xoshiro128StarStar::from_seed([1, 0, 0, 0,
                                             2, 0, 0, 0,
                                             3, 0, 0, 0,
                                             4, 0, 0, 0]);
        // These values were produced with the reference implementation:
        // http://xoshiro.di.unimi.it/xoshiro128starstar.c
        let expected = [
            5760, 40320, 70819200, 3297914139, 2480851620, 1792823698,
            4118739149, 1251203317, 1581886583, 1721184582,
        ];
        for &e in &expected {
            assert_eq!(rng.next_u32(), e);
        }
    }
}
