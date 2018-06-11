use rand_core::impls::{next_u64_via_u32, fill_bytes_via_next};
use rand_core::le::read_u32_into;
use rand_core::{SeedableRng, RngCore, Error};

#[derive(Debug, Clone)]
pub struct Xoshiro128StarStar {
    s: [u32; 4],
}

impl Xoshiro128StarStar {
    pub fn from_seed_u64(seed: u64) -> Xoshiro128StarStar {
        from_splitmix!(seed)
    }

    /// Jump forward, equivalently to 2^64 calls to `next_u32()`.
    ///
    /// This can be used to generate 2^64 non-overlapping subsequences for
    /// parallel computations.
    ///
    /// ```
    /// # extern crate rand;
    /// # extern crate xoshiro;
    /// # fn main() {
    /// use rand::SeedableRng;
    /// use xoshiro::Xoroshiro128StarStar;
    ///
    /// let rng1 = Xoroshiro128StarStar::from_seed_u64(0);
    /// let mut rng2 = rng1.clone();
    /// rng2.jump();
    /// let mut rng3 = rng2.clone();
    /// rng3.jump();
    /// # }
    /// ```
    pub fn jump(&mut self) {
        const JUMP: [u32; 4] = [
            0x8764000b, 0xf542d2d3, 0x6fa035c3, 0x77f2db5b,
        ];
        let mut s0 = 0;
        let mut s1 = 0;
        let mut s2 = 0;
        let mut s3 = 0;
        for j in &JUMP {
            for b in 0..64 {
                if (j & 1 << b) != 0 {
                    s0 ^= self.s[0];
                    s1 ^= self.s[1];
                    s2 ^= self.s[2];
                    s3 ^= self.s[3];
                }
                self.next_u32();
            }
        }
        self.s[0] = s0;
        self.s[1] = s1;
        self.s[2] = s2;
        self.s[3] = s3;
    }
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
        let result_starstar = starstar!(self.s[0]);
        let t = self.s[1] << 9;

        self.s[2] ^= self.s[0];
        self.s[3] ^= self.s[1];
        self.s[1] ^= self.s[2];
        self.s[0] ^= self.s[3];

        self.s[2] ^= t;

        self.s[3] = self.s[3].rotate_left(11);

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
    fn reference() {
        let mut rng = Xoshiro128StarStar::from_seed(
            [1, 0, 0, 0, 2, 0, 0, 0, 3, 0, 0, 0, 4, 0, 0, 0]);
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
