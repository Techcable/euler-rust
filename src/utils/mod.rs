use std::{iter, mem};
use std::str::FromStr;
use fixedbitset::FixedBitSet;
use std::fmt::{self, Debug, Formatter};
use std::hash::{Hash, Hasher};
use std::ops::{Index, Add, AddAssign};
use num::{PrimInt, Integer, Signed, Zero, ToPrimitive, FromPrimitive, NumCast, BigInt, BigUint};
use std::time::Instant;

use itertools::Itertools;
use itertools::EitherOrBoth::*;

pub mod primes;
mod digits;
mod integer_logarithm;

pub use self::digits::{Digits, BigDigits};
pub use self::integer_logarithm::IntegerLogarithm;

pub fn permutations<T: Clone>(values: Vec<T>, k: usize) -> Vec<Vec<T>> {
    let timer = DebugTimer::start();
    let mut result = Vec::new();
    permutation_indexes(k, values.len(), |indexes| {
        result.push(indexes.iter().map(|&index| values[index].clone()).collect())
    });
    timer.finish_with(|| format!("Computed {} permutations of {} values", k, values.len()));
    result
}
fn permutation_indexes<F: FnMut(&[usize])>(k: usize, n: usize, mut func: F) {
    assert!(k <= n);
    // From https://alistairisrael.wordpress.com/2009/09/22/simple-efficient-pnk-algorithm/
    let mut a = (0..n).collect::<Vec<_>>();
    loop {
        func(&a[..k]);
        let edge = k - 1;
        // find j in (k…n-1) where a[j] > a[edge]
        let mut j = k;
        while j < n && a[edge] >= a[j] {
            j += 1;
        }
        if j < n {
            a.swap(edge, j);
        } else {
            a[k..].reverse();
            // find rightmost ascent to left of edge
            let mut i = edge - 1;
            while a[i] >= a[i + 1] {
                if i == 0 {
                    return // no more permutations
                }
                i -= 1;
            }
            // find j in (n-1…i+1) where a[j] > a[i]
            let mut j = n - 1;
            while j > i && a[i] >= a[j] {
                j -= 1;
            }
            a.swap(i, j);
            a[(i + 1)..].reverse();
        }
    }
}

pub struct DebugTimer {
    start: Option<Instant>
}
impl DebugTimer {
    #[inline]
    pub fn start() -> Self {
        let start = if log_enabled!(::log::Level::Debug) {
            // This is behind a flag since it may involve a system call
            Some(Instant::now())
        } else {
            None
        };
        DebugTimer { start }
    }
    #[inline]
    pub fn finish_with<F, T>(self, mut msg: F) where F: FnMut() -> T, T: ::std::fmt::Display {
        if self.start.is_some() {
            self.finish(&msg())
        }
    }
    pub fn finish(self, msg: &::std::fmt::Display) {
        if let Some(start) = self.start {
            let elapsed = start.elapsed();
            debug!(
                "{} in {:.2} ms",
                msg, elapsed.as_float_secs() * 1000.0
            );
        }
    }
}

pub use self::primes::{prime_set, primes};

pub fn modular_pow(mut base: u64, mut exponent: u64, modulus: u64) -> u64 {
    // NOTE: Taken from wikipedia
    assert_ne!(modulus, 0);
    if modulus == 1 { return 0 }
    let mut result = 1;
    base %= modulus;
    while exponent > 0 {
        if exponent % 2 == 1 {
            result = (result * base) % modulus;
        }
        exponent >>= 1;
        base = (base * base) % modulus;
    }
    result
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

#[cfg(test)]
mod test {
    use super::permutations;
    #[test]
    fn test_permutations() {
        assert_eq!(
            permutations(vec![0, 1, 2], 3),
            vec![
                vec![0, 1, 2],
                vec![0, 2, 1],
                vec![1, 0, 2],
                vec![1, 2, 0],
                vec![2, 0, 1],
                vec![2, 1, 0],
            ]
        );
        assert_eq!(
            permutations(vec![0, 1, 2], 2),
            vec![
                vec![0, 1],
                vec![0, 2],
                vec![1, 0],
                vec![1, 2],
                vec![2, 0],
                vec![2, 2],
                vec![2, 1],
            ]
        );
    }
}