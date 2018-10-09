use std::{iter};
use std::str::FromStr;
use fixedbitset::FixedBitSet;
use std::fmt::{self, Debug, Formatter};
use std::hash::{Hash, Hasher};
use std::ops::{Index, Add, AddAssign};
use num::{PrimInt, Integer, Signed, Zero, ToPrimitive, FromPrimitive, NumCast, BigInt, BigUint};
use std::time::Instant;

use itertools::Itertools;
use itertools::EitherOrBoth::*;

mod sieve;
mod digits;
mod integer_logarithm;

pub use self::digits::{Digits, BigDigits};
pub use self::integer_logarithm::IntegerLogarithm;

pub struct DebugTimer {
    start: Option<Instant>
}
impl DebugTimer {
    pub fn start() -> Self {
        let start = if log_enabled!(::log::Level::Debug) {
            // This is behind a flag since it may involve a system call
            Some(Instant::now())
        } else {
            None
        };
        DebugTimer { start }
    }
    pub fn finish_with<F, T>(self, mut msg: F) where F: FnMut() -> T, T: ::std::fmt::Display {
        if let Some(start) = self.start {
            let elapsed = start.elapsed();
            debug!(
                "{} in {:.2} ms",
                msg(), elapsed.as_float_secs() * 1000.0
            );
        }
    }
}

pub use self::sieve::{prime_set, primes};

/// Find a reasonable approximation of the first input
/// where the function returns true.
pub fn guess_first_match<F, T>(mut func: F) -> T
    where F: FnMut(T) -> bool, T: Ord + ::num::PrimInt + ::std::ops::MulAssign {
    if func(T::zero()) { return T::zero() }
    let mut guess = T::one();
    let two = T::from(2).unwrap();
    while !func(guess) {
        guess *= two;
    }
    guess
}

pub unsafe trait ArbitraryBytes {}
unsafe impl ArbitraryBytes for u64 {}
unsafe impl ArbitraryBytes for u32 {}
unsafe impl ArbitraryBytes for usize {}

#[inline]
pub fn clear_slice<T: ArbitraryBytes>(slice: &mut [T]) {
    // Nothing is faster than memset
    write_bytes_slice(slice, 0)
}
#[inline]
pub fn write_bytes_slice<T: ArbitraryBytes>(slice: &mut [T], value: u8) {
    unsafe {
        let len = slice.len();
        slice.as_mut_ptr().write_bytes(value, len)
    }
}