use rand_core::impls::fill_bytes_via_next;
use rand_core::le::read_u64_into;
use rand_core::{SeedableRng, RngCore, Error};

use super::SplitMix64;

fn rotl(x: u64, k: u64) -> u64 {
    (x << k) | (x >> (64 - k))
}

fn starstar(s0: u64) -> u64 {
    rotl(s0.wrapping_mul(5), 7).wrapping_mul(9)
}

#[derive(Debug, Clone)]
pub struct Xoshiro256StarStar {
    s: [u64; 4],
}

impl Xoshiro256StarStar {
    pub fn from_seed_u64(seed: u64) -> Xoshiro256StarStar {
        let mut rng = SplitMix64::from_seed_u64(seed);
        Xoshiro256StarStar::from_rng(&mut rng).unwrap()
    }
}

impl SeedableRng for Xoshiro256StarStar {
    type Seed = [u8; 32];

    #[inline]
    fn from_seed(seed: [u8; 32]) -> Xoshiro256StarStar {
        let mut state = [0; 4];
        read_u64_into(&seed, &mut state);
        Xoshiro256StarStar { s: state }
    }
}

impl RngCore for Xoshiro256StarStar {
    #[inline]
    fn next_u32(&mut self) -> u32 {
        self.next_u64() as u32
    }

    #[inline]
    fn next_u64(&mut self) -> u64 {
        let result_starstar = starstar(self.s[1]);

        let t = self.s[1] << 17;

        self.s[2] ^= self.s[0];
        self.s[3] ^= self.s[1];
        self.s[1] ^= self.s[2];
        self.s[0] ^= self.s[3];

        self.s[2] ^= t;

        self.s[3] = rotl(self.s[3], 45);

        result_starstar
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
        let mut rng = Xoshiro256StarStar::from_seed(
            [1, 0, 0, 0, 0, 0, 0, 0, 2, 0, 0, 0, 0, 0, 0, 0,
             3, 0, 0, 0, 0, 0, 0, 0, 4, 0, 0, 0, 0, 0, 0, 0]);
        // These values were produced with the reference implementation:
        // http://xoshiro.di.unimi.it/xoshiro128starstar.c
        let expected = [
            11520, 0, 1509978240, 1215971899390074240, 1216172134540287360,
            607988272756665600, 16172922978634559625, 8476171486693032832,
            10595114339597558777, 2904607092377533576,
        ];
        for &e in &expected {
            assert_eq!(rng.next_u64(), e);
        }
    }
}
