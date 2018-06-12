use byteorder::{ByteOrder, LittleEndian};
use rand_core;
use rand_core::le::read_u32_into;
use rand_core::impls::{fill_bytes_via_next, next_u64_via_u32};
use rand_core::{RngCore, SeedableRng};

/// A Xoroshiro64** random number generator.
///
/// The Xoroshiro64** algorithm is not suitable for cryptographic purposes, but
/// is very fast and has better statistical properties than `XorShiftRng`.  If
/// you do not know for sure that it fits your requirements, use a more secure
/// one such as `IsaacRng` or `OsRng`.
///
/// The algorithm used here is translated from [the `xoroshiro64starstar.c`
/// reference source code](http://xoshiro.di.unimi.it/xoroshiro64starstar.c) by
/// David Blackman and Sebastiano Vigna.
#[allow(missing_copy_implementations)]
#[derive(Debug, Clone)]
pub struct Xoroshiro64StarStar {
    s0: u32,
    s1: u32,
}

impl Xoroshiro64StarStar {
    /// Seed a `Xoroshiro64StarStar` from a `u64`.
    pub fn from_seed_u64(seed: u64) -> Xoroshiro64StarStar {
        let mut s = [0; 8];
        LittleEndian::write_u64(&mut s, seed);
        Xoroshiro64StarStar::from_seed(s)
    }
}

impl RngCore for Xoroshiro64StarStar {
    #[inline]
    fn next_u32(&mut self) -> u32 {
        let r = starstar_u32!(self.s0);
        impl_xoroshiro_u32!(self);
        r
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
    fn try_fill_bytes(&mut self, dest: &mut [u8]) -> Result<(), rand_core::Error> {
        self.fill_bytes(dest);
        Ok(())
    }
}

impl SeedableRng for Xoroshiro64StarStar {
    type Seed = [u8; 8];

    /// Create a new `Xoroshiro64StarStar`.  This will panic if `seed` is entirely 0.
    fn from_seed(seed: [u8; 8]) -> Xoroshiro64StarStar {
        assert!(seed != [0, 0, 0, 0, 0, 0, 0, 0],
            "Xoroshiro64StarStar::from_seed called with an all zero seed.");
        let mut s = [0; 2];
        read_u32_into(&seed, &mut s);

        Xoroshiro64StarStar {
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
        let mut rng = Xoroshiro64StarStar::from_seed([1, 0, 0, 0, 2, 0, 0, 0]);
        // These values were produced with the reference implementation:
        // http://xoshiro.di.unimi.it/xoshiro64starstar.c
        let expected = [
            3802928447, 813792938, 1618621494, 2955957307, 3252880261,
            1129983909, 2539651700, 1327610908, 1757650787, 2763843748,
        ];
        for &e in &expected {
            assert_eq!(rng.next_u32(), e);
        }
    }
}
