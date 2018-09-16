use std::ops::{Add, AddAssign, Index};
use std::str::FromStr;
use std::fmt::{self, Debug, Formatter};
use std::hash::{Hash, Hasher};

use fixedbitset::FixedBitSet;
use ndarray::{NdIndex, IxDyn};
use itertools::Itertools;
use itertools::EitherOrBoth::*;

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

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_palindrome() {
        assert!(is_palindrome(&[1]));
        assert!(is_palindrome(&[7, 3, 3, 7]));
        assert!(!is_palindrome(&[7, 3, 4, 7]));
        assert!(is_palindrome(&[7, 3, 1, 3, 7]));
        assert!(is_palindrome(&[1, 2, 1]));
    }
}
