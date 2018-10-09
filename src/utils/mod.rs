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

/// Make a bitset of all primes less than the specified value.
///
/// Internally this uses the sieve of eratosthenes for simplicity,
/// as it's very fast for finding prime values.
pub fn prime_set(limit: u64) -> FixedBitSet {
    assert!(limit <= (usize::max_value() as u64));
    let timer = DebugTimer::start();
    let mut is_prime = FixedBitSet::with_capacity(limit as usize);
    is_prime.set_range(2.., true);
    for i in 2..((limit as f64).sqrt().ceil() as usize) {
        if is_prime[i] {
            let mut j = i * i;
            while j < (limit as usize) {
                is_prime.set(j, false);
                j += i;
            }
        }
    }
    timer.finish_with(|| format!("Computed prime set of {}", limit));
    is_prime
}

/// List of all primes less than the specified value.
///
/// Internally this is just a simple wrapper around `prime_set`.
pub fn primes(limit: u64) -> Vec<u64> {
    prime_set(limit).ones().map(|i| i as u64).collect()
}

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

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_primes() {
        assert_eq!(primes(2), vec![]);
        assert_eq!(primes(14), vec![2, 3, 5, 7, 11, 13]);
        assert_eq!(primes(32), vec![2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31]);
    }
}