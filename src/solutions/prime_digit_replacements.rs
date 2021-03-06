use std::collections::HashMap;

use fixedbitset::FixedBitSet;
use failure::Error;
use ndarray::prelude::*;
use itertools::Itertools;

use utils::Digits;

/// The solution to the prime digit replacement problem,
/// originally solved here `https://gist.github.com/Techcable/965341b217ae82defe1f541b3118c328`.
///
/// My key breakthrough was recognizing that I could use a n-dimensional matrix
/// in order to cache whether or not a certain number was prime.
/// Once you have the digit representation of a single prime number,
/// you can quickly determine whether the entire family is prime.
pub fn solve() -> Result<u64, Error> {
    let result = digit_replacement_prime_families(6, 8)
        .ok_or_else(|| format_err!("Unable to solve prime_digit_replacmeents"))?;
    Ok(result.0)
}

fn digit_replacement_prime_families(bound_digits: usize, minimum_size: usize) -> Option<(u64, Vec<u64>)> {
    assert!(bound_digits > 1, "Invalid bound digits: {}", bound_digits);
    let matrix = PrimeDigitMatrix::new(bound_digits);
    assert_eq!(matrix.primes.len(), matrix.prime_digits.len());
    let mut digit_replacement_combinations = Vec::new();
    for num_replaced in 1..bound_digits {
        digit_replacement_combinations.extend((0..bound_digits).combinations(num_replaced))
    }
    assert!(bound_digits != 5 || digit_replacement_combinations.contains(&vec![2, 3]));
    let prime_digit_map = matrix.prime_digits.iter().cloned()
        .zip_eq(matrix.primes.iter().cloned())
        .collect::<HashMap<Digits, u64>>();
    for prime_digits in matrix.prime_digits.iter() {
        if prime_digits[0] == 0 { continue }
        // Try replacing parts of the digits
        for replacement_indexes in &digit_replacement_combinations {
            let mut digits = prime_digits.clone();
            let mut prime_family = Vec::with_capacity(minimum_size);
            for value in 0u8..10 {
                for &index in replacement_indexes {
                    digits.insert(index, value)
                }
                if matrix.matrix[digits] && digits[0] != 0 {
                    prime_family.push(prime_digit_map[&digits])
                }
            }
            if prime_family.len() >= minimum_size {
                assert!(prime_family.iter().all(|prime| matrix.primes.contains(prime)));
                return Some((*prime_family.iter().min().unwrap(), prime_family))
            }
        }
    }
    None
}

pub struct PrimeDigitMatrix {
    primes: Vec<u64>,
    prime_digits: Vec<Digits>,
    matrix: Array<bool, IxDyn>
}

impl PrimeDigitMatrix {
    pub fn new(amount: usize) -> PrimeDigitMatrix {
        let primes = ::utils::primes(10u64.pow(amount as u32));
        let mut prime_digits = Vec::new();
        let mut matrix = Array::<bool, _>::default(IxDyn(&vec![10; amount]));
        for &prime in &primes {
            let digits = Digits::from_value(prime).padded(amount);
            matrix[digits] = true;
            prime_digits.push(digits);
        }
        PrimeDigitMatrix { primes, prime_digits, matrix }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    #[ignore]
    fn check_solutions() {
        // This is the example solution
        assert_eq!(digit_replacement_prime_families(5, 7).unwrap().0, 56003);
        // This is the solution to the primary problem
        assert_eq!(digit_replacement_prime_families(6, 8).unwrap().0, 121313);
    }

}
