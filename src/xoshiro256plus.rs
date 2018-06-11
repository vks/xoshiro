use rand_core::impls::fill_bytes_via_next;
use rand_core::le::read_u64_into;
use rand_core::{SeedableRng, RngCore, Error};

#[derive(Debug, Clone)]
pub struct Xoshiro256Plus {
    s: [u64; 4],
}

impl Xoshiro256Plus {
    pub fn from_seed_u64(seed: u64) -> Xoshiro256Plus {
        from_splitmix!(seed)
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
    /// use xoshiro::Xoshiro256Plus;
    ///
    /// let rng1 = Xoshiro256Plus::from_seed_u64(0);
    /// let mut rng2 = rng1.clone();
    /// rng2.jump();
    /// let mut rng3 = rng2.clone();
    /// rng3.jump();
    /// # }
    /// ```
    pub fn jump(&mut self) {
        impl_jump!(u64, self, [
            0x180ec6d33cfd0aba, 0xd5a61266f0c9392c,
            0xa9582618e03fc9aa, 0x39abdc4529b1661c
        ]);
    }
}

impl SeedableRng for Xoshiro256Plus {
    type Seed = [u8; 32];

    #[inline]
    fn from_seed(seed: [u8; 32]) -> Xoshiro256Plus {
        let mut state = [0; 4];
        read_u64_into(&seed, &mut state);
        Xoshiro256Plus { s: state }
    }
}

impl RngCore for Xoshiro256Plus {
    #[inline]
    fn next_u32(&mut self) -> u32 {
        // The lowest bits have some linear dependencies, so we use the
        // upper bits instead.
        (self.next_u64() >> 32) as u32
    }

    #[inline]
    fn next_u64(&mut self) -> u64 {
        let result_plus = self.s[0].wrapping_add(self.s[3]);
        impl_xoshiro_u64!(self);
        result_plus
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
        let mut rng = Xoshiro256Plus::from_seed(
            [1, 0, 0, 0, 0, 0, 0, 0, 2, 0, 0, 0, 0, 0, 0, 0,
             3, 0, 0, 0, 0, 0, 0, 0, 4, 0, 0, 0, 0, 0, 0, 0]);
        // These values were produced with the reference implementation:
        // http://xoshiro.di.unimi.it/xoshiro256plus.c
        let expected = [
            5, 211106232532999, 211106635186183, 9223759065350669058,
            9250833439874351877, 13862484359527728515, 2346507365006083650,
            1168864526675804870, 34095955243042024, 3466914240207415127,
        ];
        for &e in &expected {
            assert_eq!(rng.next_u64(), e);
        }
    }
}
