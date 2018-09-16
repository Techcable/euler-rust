use std::{iter};
use std::str::FromStr;
use fixedbitset::FixedBitSet;
use std::fmt::{self, Debug, Formatter};
use std::hash::{Hash, Hasher};
use std::ops::{Index, Add, AddAssign};
use num::{PrimInt, Integer, Signed, Zero, ToPrimitive, FromPrimitive, NumCast, BigInt, BigUint};

use itertools::Itertools;
use itertools::EitherOrBoth::*;

mod digits;
mod integer_logarithm;

pub use self::digits::{Digits, BigDigits};
pub use self::integer_logarithm::IntegerLogarithm;

/// List of all primes less than the specified value.
///
/// Internally this uses the sieve of eratosthenes for simplicity,
/// as it's very fast for finding prime values.
pub fn primes(limit: u64) -> Vec<u64> {
    assert!(limit <= (usize::max_value() as u64));
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
    is_prime.ones().map(|i| i as u64).collect()
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