extern crate byteorder;
extern crate rand_core;

#[macro_use]
mod common;
mod splitmix64;
mod xoshiro128starstar;
mod xoshiro128plus;
mod xoshiro256starstar;
mod xoshiro256plus;
mod xoshiro512starstar;
mod xoshiro512plus;
mod xoroshiro128plus;
mod xoroshiro128starstar;
mod xoroshiro64starstar;
mod xoroshiro64star;

pub use splitmix64::SplitMix64;
pub use xoshiro128starstar::Xoshiro128StarStar;
pub use xoshiro128plus::Xoshiro128Plus;
pub use xoshiro256starstar::Xoshiro256StarStar;
pub use xoshiro256plus::Xoshiro256Plus;
pub use xoshiro512starstar::{Xoshiro512StarStar, Xoshiro512StarStarSeed};
pub use xoshiro512plus::{Xoshiro512Plus, Xoshiro512PlusSeed};
pub use xoroshiro128plus::Xoroshiro128Plus;
pub use xoroshiro128starstar::Xoroshiro128StarStar;
pub use xoroshiro64starstar::Xoroshiro64StarStar;
pub use xoroshiro64star::Xoroshiro64Star;
