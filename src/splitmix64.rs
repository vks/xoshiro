use byteorder::{ByteOrder, LittleEndian};
use rand_core::le::read_u64_into;
use rand_core::impls::fill_bytes_via_next;
use rand_core::{RngCore, SeedableRng, Error};

/// A splitmix64 random number generator.
///
/// The splitmix algorithm is not suitable for cryptographic purposes, but is
/// very fast and has a 64 bit state.
///
/// The algorithm used here is translated from [the `splitmix64.c`
/// reference source code](http://xoshiro.di.unimi.it/splitmix64.c) by
/// Sebastiano Vigna.
#[allow(missing_copy_implementations)]
#[derive(Debug, Clone)]
pub struct SplitMix64 {
    x: u64,
}

impl SplitMix64 {
    pub fn from_seed_u64(seed: u64) -> SplitMix64 {
        let mut x = [0; 8];
        LittleEndian::write_u64(&mut x, seed);
        SplitMix64::from_seed(x)
    }
}

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reference() {
        let mut rng = SplitMix64::from_seed_u64(1477776061723855037);
        // These values were produced with the reference implementation:
        // http://xoshiro.di.unimi.it/splitmix64.c
        let expected = vec![
            1985237415132408290, 2979275885539914483, 13511426838097143398,
            8488337342461049707, 15141737807933549159, 17093170987380407015,
            16389528042912955399, 13177319091862933652, 10841969400225389492,
            17094824097954834098, 3336622647361835228, 9678412372263018368,
            11111587619974030187, 7882215801036322410, 5709234165213761869,
            7799681907651786826, 4616320717312661886, 4251077652075509767,
            7836757050122171900, 5054003328188417616, 12919285918354108358,
            16477564761813870717, 5124667218451240549, 18099554314556827626,
            7603784838804469118, 6358551455431362471, 3037176434532249502,
            3217550417701719149, 9958699920490216947, 5965803675992506258,
            12000828378049868312, 12720568162811471118, 245696019213873792,
            8351371993958923852, 14378754021282935786, 5655432093647472106,
            5508031680350692005, 8515198786865082103, 6287793597487164412,
            14963046237722101617, 3630795823534910476, 8422285279403485710,
            10554287778700714153, 10871906555720704584, 8659066966120258468,
            9420238805069527062, 10338115333623340156, 13514802760105037173,
            14635952304031724449, 15419692541594102413,
        ];
        for &e in &expected {
            assert_eq!(rng.next_u64(), e);
        }
    }
}
