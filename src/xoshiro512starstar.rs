use rand_core::impls::fill_bytes_via_next;
use rand_core::le::read_u64_into;
use rand_core::{SeedableRng, RngCore, Error};

#[derive(Debug, Clone)]
pub struct Xoshiro512StarStar {
    s: [u64; 8],
}

impl Xoshiro512StarStar {
    pub fn from_seed_u64(seed: u64) -> Xoshiro512StarStar {
        from_splitmix!(seed)
    }

    /// Jump forward, equivalently to 2^256 calls to `next_u64()`.
    ///
    /// This can be used to generate 2^256 non-overlapping subsequences for
    /// parallel computations.
    ///
    /// ```
    /// # extern crate rand;
    /// # extern crate xoshiro;
    /// # fn main() {
    /// use rand::SeedableRng;
    /// use xoshiro::Xoshiro512StarStar;
    ///
    /// let rng1 = Xoshiro512StarStar::from_seed_u64(0);
    /// let mut rng2 = rng1.clone();
    /// rng2.jump();
    /// let mut rng3 = rng2.clone();
    /// rng3.jump();
    /// # }
    /// ```
    pub fn jump(&mut self) {
        impl_jump!(u64, self, [
            0x33ed89b6e7a353f9, 0x760083d7955323be, 0x2837f2fbb5f22fae,
            0x4b8c5674d309511c, 0xb11ac47a7ba28c25, 0xf1be7667092bcc1c,
            0x53851efdb6df0aaf, 0x1ebbc8b23eaf25db
        ]);
    }
}

#[derive(Clone)]
pub struct Xoshiro512StarStarSeed(pub [u8; 64]);

impl ::std::fmt::Debug for Xoshiro512StarStarSeed {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        self.0[..].fmt(f)
    }
}

impl Default for Xoshiro512StarStarSeed {
    fn default() -> Xoshiro512StarStarSeed {
        Xoshiro512StarStarSeed([0; 64])
    }
}

impl AsMut<[u8]> for Xoshiro512StarStarSeed {
    fn as_mut(&mut self) -> &mut [u8] {
        &mut self.0
    }
}

impl SeedableRng for Xoshiro512StarStar {
    type Seed = Xoshiro512StarStarSeed;

    #[inline]
    fn from_seed(seed: Xoshiro512StarStarSeed) -> Xoshiro512StarStar {
        let mut state = [0; 8];
        read_u64_into(&seed.0, &mut state);
        Xoshiro512StarStar { s: state }
    }
}

impl RngCore for Xoshiro512StarStar {
    #[inline]
    fn next_u32(&mut self) -> u32 {
        self.next_u64() as u32
    }

    #[inline]
    fn next_u64(&mut self) -> u64 {
        let result_starstar = starstar_u64!(self.s[1]);
        impl_xoshiro_large!(self);
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
    fn reference() {
        let mut rng = Xoshiro512StarStar::from_seed(Xoshiro512StarStarSeed(
            [1, 0, 0, 0, 0, 0, 0, 0, 2, 0, 0, 0, 0, 0, 0, 0,
             3, 0, 0, 0, 0, 0, 0, 0, 4, 0, 0, 0, 0, 0, 0, 0,
             5, 0, 0, 0, 0, 0, 0, 0, 6, 0, 0, 0, 0, 0, 0, 0,
             7, 0, 0, 0, 0, 0, 0, 0, 8, 0, 0, 0, 0, 0, 0, 0]));
        // These values were produced with the reference implementation:
        // http://xoshiro.di.unimi.it/xoshiro512starstar.c
        let expected = [
            11520, 0, 23040, 23667840, 144955163520, 303992986974289920,
            25332796375735680, 296904390158016, 13911081092387501979,
            15304787717237593024,
        ];
        for &e in &expected {
            assert_eq!(rng.next_u64(), e);
        }
    }
}
