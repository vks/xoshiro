use rand_core::le::read_u64_into;
use rand_core::impls::fill_bytes_via_next;
use rand_core::{RngCore, SeedableRng, Error};

/// A splitmix random number generator.
///
/// The splitmix algorithm is not suitable for cryptographic purposes, but is
/// very fast and has a 64 bit state.  Usually `XoroShiro128` should be prefered.
/// If you do not know for sure that it fits your requirements, use a more
/// secure one such as `IsaacRng` or `OsRng`.
///
/// The algorithm used here is translated from [the `splitmix64.c`
/// reference source code](http://xoshiro.di.unimi.it/splitmix64.c) by
/// Sebastiano Vigna.
#[allow(missing_copy_implementations)]
#[derive(Debug, Clone)]
pub struct SplitMix64 {
    x: u64,
}

/*
impl SplitMix64 {
    pub fn from_seed_u64(seed: u64) -> SplitMix64 {
        let mut x = [0; 8];
        LittleEndian::write_u64(&mut x, seed);
        SplitMix64::from_seed(x)
    }
}
*/

impl RngCore for SplitMix64 {
    #[inline]
    fn next_u32(&mut self) -> u32 {
        self.next_u64() as u32
    }

    #[inline]
    fn next_u64(&mut self) -> u64 {
        self.x = self.x.wrapping_add(0x9e3779b97f4a7c15);
        let mut z = self.x;
        z = (z ^ (z >> 30)).wrapping_mul(0xbf58476d1ce4e5b9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94d049bb133111eb);
        z ^ (z >> 31)
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

impl SeedableRng for SplitMix64 {
    type Seed = [u8; 8];

    /// Create a new `SplitMix64`.
    fn from_seed(seed: [u8; 8]) -> SplitMix64 {
        let mut state = [0; 1];
        read_u64_into(&seed, &mut state);
        SplitMix64 {
            x: state[0],
        }
    }
}
