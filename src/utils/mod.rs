use std::{iter};
use fixedbitset::FixedBitSet;

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

pub fn digits_of(mut num: u64, target_size: usize) -> Vec<u8> {
    if num == 0 {
        return vec![0]
    }
    let mut result = Vec::with_capacity(target_size);
    while num > 0 {
        let digit = num % 10;
        num /= 10;
        result.push(digit as u8);
    }
    let remaining_digits = target_size - result.len();
    if remaining_digits > 0 {
        result.extend(iter::repeat(0).take(remaining_digits));
    }
    result.reverse();
    result
}
pub fn from_digits(digits: &[u8]) -> u64 {
    let mut result = 0u64;
    for &digit in digits {
        debug_assert!(digit < 10, "Invalid digit: {}", digit);
        result *= 10;
        result += digit as u64;
    }
    result
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