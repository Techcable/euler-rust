use std::{iter};
use std::str::FromStr;
use fixedbitset::FixedBitSet;
use ndarray::{NdIndex, IxDyn};
use std::fmt::{self, Debug, Formatter};
use std::hash::{Hash, Hasher};
use std::ops::{Index, Add, AddAssign};
use num::{PrimInt, Integer, Signed, Zero, ToPrimitive, FromPrimitive, NumCast, BigInt, BigUint};

use itertools::Itertools;
use itertools::EitherOrBoth::*;

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

#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Digits {
    len: u8,
    values: [u8; 20]
}
impl Digits {
    #[inline]
    pub fn new() -> Digits {
        Digits { len: 0, values: [0; 20] }
    }
    pub fn from_digits(digits: &[u8]) -> Digits {
        assert!(digits.len() <= 20, "Invalid digits: {:?}", digits);
        let mut values = [0u8; 20];
        for (&digit, result) in digits.iter().zip(values.iter_mut()) {
            assert!(digit < 10, "Invalid digit: {}", digit);
            *result = digit;
        }
        Digits { values, len: digits.len() as u8 }
    }
    pub fn from_value(mut num: u64) -> Digits {
        if num == 0 {
            return Digits { values: [0; 20], len: 1 }
        }
        let mut result = Digits::new();
        while num > 0 {
            let digit = num % 10;
            num /= 10;
            result.push(digit as u8);
        }
        result.reverse();
        result
    }
    #[inline]
    pub fn padded(mut self, amount: usize) -> Self {
        self.pad(amount);
        self
    }
    #[inline]
    pub fn pad(&mut self, amount: usize) {
        assert!(amount <= 20, "Invalid amount: {}", amount);
        let len = self.len as usize;
        if len < amount {
            let padding = amount - len;
            for index in (len..amount).rev() {
                let value = self.values[index];
                self.values[index + padding] = value;
            }
            for i in &mut self.values[..padding] {
                *i = 0;
            }
            self.len = amount as u8;
        }
    }
    #[inline]
    pub fn is_palindrome(&self) -> bool {
        is_palindrome(self.as_slice())
    }
    #[inline]
    pub fn len(&self) -> u8 {
        self.len
    }
    pub fn value(&self) -> u64 {
        let mut result = 0u64;
        for &digit in self.as_slice() {
            debug_assert!(digit < 10, "Invalid digit: {}", digit);
            result *= 10;
            result += digit as u64;
        }
        result
    }
    pub fn checked_value(&self) -> Option<u64> {
        let mut result = 0u64;
        for &digit in self.as_slice() {
            debug_assert!(digit < 10, "Invalid digit: {}", digit);
            result = result.checked_mul(10)?;
            result = result.checked_add(digit as u64)?;
        }
        Some(result)
    }
    #[inline]
    pub fn push(&mut self, digit: u8) {
        assert!(digit < 10, "Invalid digit: {}", digit);
        assert!(self.len < 20, "Capacity overflow adding {} to {:?}", digit, self);
        self.values[self.len as usize] = digit;
        self.len += 1;
    }
    #[inline]
    pub fn insert(&mut self, index: usize, digit: u8) {
        assert!(digit < 10, "Invalid digit: {}", digit);
        self.as_mut_slice()[index] = digit;
    }
    #[inline]
    pub fn reversed(mut self) -> Digits {
        self.reverse();
        self
    }
    #[inline]
    pub fn reverse(&mut self) {
        self.as_mut_slice().reverse();
    }
    #[inline]
    pub fn as_slice(&self) -> &[u8] {
        &self.values[..(self.len as usize)]
    }
    #[inline]
    fn as_mut_slice(&mut self) -> &mut [u8] {
        &mut self.values[..(self.len as usize)]
    }
    #[inline]
    fn usize_array(&self) -> [usize; 20] {
        let mut indexes = [0usize; 20];
        for (&digit, result) in self.as_slice().iter().zip(indexes.iter_mut()) {
            *result = digit as usize;
        }
        indexes
    }
}
#[inline]
fn is_palindrome(digits: &[u8]) -> bool {
    let half = digits.len() / 2;
    digits[..half].iter().eq(digits[(digits.len() - half)..].iter().rev())
}
impl Debug for Digits {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        f.debug_list().entries(self.as_slice()).finish()
    }
}
impl Hash for Digits {
    #[inline]
    fn hash<H: Hasher>(&self, state: &mut H) {
        state.write_u8(self.len);
        state.write(self.as_slice());
    }
}
impl Index<usize> for Digits {
    type Output = u8;

    #[inline]
    fn index(&self, index: usize) -> &Self::Output {
        &self.as_slice()[index]
    }
}
unsafe impl NdIndex<IxDyn> for Digits {
    #[inline]
    fn index_checked(&self, dim: &IxDyn, strides: &IxDyn) -> Option<isize> {
        (&self.usize_array()[..(self.len as usize)]).index_checked(dim, strides)
    }
    #[inline]
    fn index_unchecked(&self, strides: &IxDyn) -> isize {
        (&self.usize_array()[..(self.len as usize)]).index_unchecked(strides)
    }
}
#[derive(Clone, Eq, PartialEq, Debug)]
pub struct BigDigits(Vec<u8>);
impl BigDigits {
    #[inline]
    pub fn from_digits(digits: &[u8]) -> BigDigits {
        BigDigits(digits.iter().map(|&digit| {
            assert!(digit < 10, "Invalid digit: {:?}", digit);
            digit
        }).collect())
    }
    #[inline]
    pub fn from_value(value: u64) -> BigDigits {
        BigDigits::from(Digits::from_value(value))
    }
    #[inline]
    pub fn reverse(&mut self) {
        self.0.reverse();
    }
    pub fn checked_value(&self) -> Option<u64> {
        let mut result = 0u64;
        for &digit in &self.0 {
            result = result.checked_mul(10)?;
            result = result.checked_add(digit as u64)?;
        }
        Some(result)
    }
    #[inline]
    pub fn reversed(&self) -> BigDigits {
        let mut result = self.0.clone();
        result.reverse();
        BigDigits(result)
    }
    #[inline]
    pub fn is_palindrome(&self) -> bool {
        is_palindrome(&self.0)
    }
}
impl From<Digits> for BigDigits {
    #[inline]
    fn from(digits: Digits) -> Self {
        BigDigits(Vec::from(digits.as_slice()))
    }
}
impl AddAssign for BigDigits {
    #[inline]
    fn add_assign(&mut self, rhs: BigDigits) {
        let result = self.clone() + rhs;
        *self = result;
    }
}
impl Add for BigDigits {
    type Output = BigDigits;
    fn add(self, rhs: BigDigits) -> BigDigits {
        let mut carry = false;
        let mut result = Vec::with_capacity(self.0.len().max(rhs.0.len()) + 1);
        for either in self.0.iter().rev().zip_longest(rhs.0.iter().rev()) {
            let (left, right) = match either {
                Left(&left) => (left, 0),
                Right(&right) => (0, right),
                Both(&left, &right) => (left, right)
            };
            let (digit, overflow) = add_digit(left, right, carry);
            carry = overflow;
            result.push(digit);
        }
        if carry {
            result.push(1);
        }
        result.reverse();
        let result = BigDigits(result);
        debug_assert_eq!(result.checked_value(), self.checked_value().and_then(|left| {
            rhs.checked_value().and_then(|right| left.checked_add(right))
        }));
        result
    }
}
#[inline]
fn add_digit(left: u8, right: u8, mut carry: bool) -> (u8, bool) {
    let mut result = left + right + (carry as u8);
    carry = result >= 10;
    if carry {
        debug_assert!(result < 20);
        result -= 10;
    }
    (result, carry)
}

pub trait IntegerLogarithm: Clone + FromPrimitive + Integer + ::std::fmt::Debug + ::num_traits::CheckedMul {
    #[inline]
    fn exp2(amount: usize) -> Self {
        Self::one().shl(amount)
    }
    fn exp10(amount: usize) -> Self {
        assert!(amount <= usize::max_value());
        ::num::pow::pow(
            Self::from_u8(10).unwrap(),
            amount
        )
    }
    fn nlz(&self) -> usize;
    fn size(&self) -> usize;
    #[inline]
    fn floor_log2(&self) -> u64 {
        assert!(!self.is_zero());
        debug_assert!(self.size() > 0, "Invalid size: {:?}", self);
        debug_assert!(
            self.nlz() <= self.size(),
            "Invalid nlz {:?} for size {:?}: {:?}",
            self.nlz(), self.size(), self
        );
        ((self.size() - 1) - self.nlz()) as u64
    }
    #[inline]
    fn ceil_log2(&self) -> u64 {
        assert!(!self.is_zero());
        (self.size() - (self.clone() - Self::one()).nlz()) as u64
    }
    fn shl(&self, amount: usize) -> Self;
    fn ceil_log10(&self) -> u64 {
        let mut log = self.floor_log10();
        let ten = Self::from_u8(10).unwrap();
        let mut power = Self::exp10(log.to_usize().unwrap());
        debug_assert!(power <= *self);
        while power < *self {

            log += 1;
            power = match power.checked_mul(&ten) {
                Some(increased) => increased,
                None => {
                    /*
                     * We overflowed and can't get actually represent the next power (the current log).
                     * However, since we can't represent the next power
                     * it also means that it's the power of self.
                     */
                    debug_assert!(self.clone() / power.clone() < Self::from_u8(100).unwrap());
                    break
                }
            }
        }
        log
    }
    fn floor_log10(&self) -> u64 {
        /*
         * Using the change of base formula we can have a
         * crude approximation of floor(log10(self)).
         * Even the integer logarithm may not be correct,
         * since we have to do possibly inexact floating point division.
         * By rounding down, we should guarantee that
         * 10**approx is always less than 10**real_floor.
         * Then we can keep correcting as long as the factor is greater than ten.
         */
        let guess = ((self.floor_log2() as f64) / ::std::f64::consts::LOG2_10).floor() as u64;
        let ten = Self::from_u8(10).unwrap();
        let mut real_log = guess;
        let mut factor = self.clone() / Self::exp10(real_log.to_usize().unwrap());
        assert!(!factor.is_zero());
        while factor >= ten {
            factor = factor / Self::from_u8(10).unwrap();
            real_log += 1;
        }
        real_log
    }
    fn abs(&self) -> Self {
        let result = match Self::from_i8(-1) {
            Some(neg_1) => {
                if *self < Self::zero() {
                    neg_1 * self.clone()
                } else {
                    self.clone()
                }
            },
            None => self.clone()
        };
        debug_assert!(result >= Self::zero());
        result
    }
    fn count_decimal_digits(&self) -> u64 {
        if self.is_zero() {
            1
        } else {
            self.abs().ceil_log10()
        }
    }
}
#[inline]
fn nlz_bytes(bytes: &[u8]) -> usize {
    let mut nlz = 0;
    for &b in bytes.iter().rev() {
        if b == 0 {
            nlz += 8;
        } else {
            nlz += b.leading_zeros() as usize;
            break
        }
    }
    nlz
}
impl IntegerLogarithm for BigInt {
    #[inline]
    fn nlz(&self) -> usize {
        nlz_bytes(&self.to_signed_bytes_le())
    }

    fn size(&self) -> usize {
        self.to_signed_bytes_le().len() * 8
    }

    #[inline]
    fn shl(&self, amount: usize) -> Self {
        self << amount
    }

    #[inline]
    fn abs(&self) -> Self {
        Signed::abs(self)
    }
}
impl IntegerLogarithm for BigUint {
    fn nlz(&self) -> usize {
        nlz_bytes(&self.to_bytes_le())
    }

    fn size(&self) -> usize {
        self.to_bytes_le().len() * 8
    }

    #[inline]
    fn shl(&self, amount: usize) -> Self {
        self << amount
    }

    #[inline]
    fn abs(&self) -> Self {
        self.clone()
    }
}
macro_rules! prim_int_lograithm {
    ($($target:ty),*) => {
        $(impl IntegerLogarithm for $target {
            #[inline]
            fn nlz(&self) -> usize {
                self.leading_zeros() as usize
            }

            #[inline]
            fn size(&self) -> usize {
                ::std::mem::size_of::<Self>() * 8
            }

            #[inline]
            fn shl(&self, amount: usize) -> Self {
                *self << (amount as u32)
            }
        })*
    }
}
prim_int_lograithm!(i32, u32, i64, u64);

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_primes() {
        assert_eq!(primes(2), vec![]);
        assert_eq!(primes(14), vec![2, 3, 5, 7, 11, 13]);
        assert_eq!(primes(32), vec![2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31]);
    }
    #[test]
    fn test_palindrome() {
        assert!(is_palindrome(&[1]));
        assert!(is_palindrome(&[7, 3, 3, 7]));
        assert!(!is_palindrome(&[7, 3, 4, 7]));
        assert!(is_palindrome(&[7, 3, 1, 3, 7]));
        assert!(is_palindrome(&[1, 2, 1]));
    }
    #[test]
    fn test_count_decimal_digits() {
        assert_eq!(IntegerLogarithm::count_decimal_digits(&-922337203685477580i64), 18);
        assert_eq!(IntegerLogarithm::count_decimal_digits(&922337203685477580i64), 18);
        assert_eq!(IntegerLogarithm::count_decimal_digits(&-12), 2);
        assert_eq!(IntegerLogarithm::count_decimal_digits(&12), 2);
        assert_eq!(IntegerLogarithm::count_decimal_digits(&-12345), 5);
        assert_eq!(IntegerLogarithm::count_decimal_digits(&12345), 5);
    }
    #[test]
    fn integer_exp2() {
        assert_eq!(i32::exp2(0), 1);
        assert_eq!(i32::exp2(1), 2);
        assert_eq!(i32::exp2(4), 16);
        assert_eq!(i32::exp2(8), 256);
        assert_eq!(u32::exp2(31) - 1, i32::max_value() as u32);
    }
    #[test]
    fn integer_exp10() {
        assert_eq!(i32::exp10(0), 1);
        assert_eq!(i32::exp10(1), 10);
        assert_eq!(i32::exp10(2), 100);
        assert_eq!(i32::exp10(3), 1000);
        assert_eq!(i32::exp10(4), 10_000);
        assert_eq!(i32::exp10(6), 1_000_000);
        assert_eq!(i32::exp10(9), 1_000_000_000);
    }
    #[test]
    fn integer_log2() {
        assert_eq!(1.floor_log2(), 0);
        assert_eq!(1.floor_log2(), 0);
        assert_eq!(2.floor_log2(), 1);
        assert_eq!(2.floor_log2(), 1);
        assert_eq!(256.floor_log2(), 8);
        assert_eq!(256.ceil_log2(), 8);
        assert_eq!(255.floor_log2(), 7);
        assert_eq!(255.ceil_log2(), 8);
        let i = 2384908324i64;
        assert_eq!(i.floor_log2(), 31);
        assert_eq!(i.ceil_log2(), 32);
        let i = BigInt::from_str("8091834908109384091283094").unwrap();
        assert_eq!(i.floor_log2(), 82);
        assert_eq!(i.ceil_log2(), 83);
        let i = BigInt::exp2(4898);
        assert_eq!(i.floor_log2(), 4898);
        assert_eq!(i.ceil_log2(), 4898);
    }

    #[test]
    fn integer_log10() {
        assert_eq!(1.floor_log10(), 0);
        assert_eq!(1.ceil_log10(), 0);
        assert_eq!(2.floor_log2(), 1);
        assert_eq!(2.ceil_log10(), 1);
        assert_eq!(256.floor_log2(), 8);
        assert_eq!(256.ceil_log2(), 8);
        assert_eq!(255.floor_log2(), 7);
        assert_eq!(255.ceil_log2(), 8);
        let i = 2384908324i64;
        assert_eq!(i.floor_log2(), 31);
        assert_eq!(i.ceil_log2(), 32);
        let i = BigInt::from_str("8091834908109384091283094").unwrap();
        assert_eq!(i.floor_log2(), 82);
        assert_eq!(i.ceil_log2(), 83);
        let i = BigInt::exp2(4898);
        assert_eq!(i.floor_log2(), 4898);
        assert_eq!(i.ceil_log2(), 4898);
    }
}