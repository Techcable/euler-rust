use std::{iter};
use fixedbitset::FixedBitSet;
use ndarray::{NdIndex, IxDyn};
use std::fmt::{self, Debug, Formatter};
use std::hash::{Hash, Hasher};
use std::ops::Index;

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
        result.as_mut_slice().reverse();
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