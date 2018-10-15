use std::iter;

use failure::Error;
use ndarray::{Array2, ArrayView2};
use num::Zero;
use num::rational::Ratio;
use utils::primes::IncrementalPrimeSet;

use euler::EulerContext;
use super::EulerProblem;

pub struct Corner([u64; 4]);
/// An infinite iterator over the diagonals of the spiral
pub struct SpiralCornerIter {
    side_length: u32,
    last_value: u64
}
impl SpiralCornerIter {
    #[inline]
    pub fn new() -> SpiralCornerIter {
        SpiralCornerIter {
            side_length: 2,
            last_value: 1,
        }
    }
    pub fn take_until_length(self, limit: u32) -> impl Iterator<Item=u64> {
        self.take_while(move |&(length, _)| length <= limit)
            .flat_map(|(_, value)| ::arrayvec::ArrayVec::from(value.0).into_iter())
    }
}
impl Iterator for SpiralCornerIter {
    type Item = (u32, Corner);

    #[inline]
    fn next(&mut self) -> Option<(u32, Corner)> {
        debug_assert!(self.side_length >= 2);
        let side_length = self.side_length;
        let last_value = self.last_value;
        let offset = side_length as u64;
        let corner_table = [
            last_value + offset,
            last_value + offset * 2,
            last_value + offset * 3,
            last_value + offset * 4,
        ];
        self.last_value = corner_table[3];
        self.side_length += 2;
        return Some((side_length, Corner(corner_table)))
    }
}

#[inline]
pub fn corners() -> SpiralCornerIter {
    SpiralCornerIter::new()
}

/*
fn diagonal_ratios() -> DiagonalPrimeRatios {
    DiagonalPrimeRatios {
        diagonals: diagonals().enumerate(),
        prime_set: IncrementalPrimeSet::new(),
        prime_count: 0
    }
}
struct DiagonalPrimeRatios {
    diagonals: iter::Enumerate<SpiralCornerIter>,
    prime_set: IncrementalPrimeSet,
    prime_count: usize
}
impl DiagonalPrimeRatios {
}
impl Iterator for DiagonalPrimeRatios {
    type Item = (Ratio<usize>, u32, u64);

    fn next(&mut self) -> Option<Self::Item> {
        match self.diagonals.next() {
            Some((index, (level, value))) => {
                if self.check_prime(value) {
                    self.prime_count += 1;
                }
                Some((Ratio::new(self.prime_count, index + 1), level, value))
            },
            None => None
        }
    }
}
*/

/// Solutions which the website says are incorrect
const INCORRECT_SOLUTIONS: &[u32] = &[
    13120,
    13119,
    26239,
    26237,
    2763,
    25981
];
#[derive(Default)]
pub struct SpiralPrimeProblem;
impl EulerProblem for SpiralPrimeProblem {
    fn name(&self) -> &'static str {
        "spiral_primes"
    }

    fn solve(&self, _context: &EulerContext) -> Result<String, Error> {
        let mut prime_count = 0;
        let mut prime_set = IncrementalPrimeSet::new();
        for (side_length, corner) in corners() {
            for &value in corner.0.iter() {
                if prime_set.check_prime(value) {
                    prime_count += 1;
                }
            }
            let total_count = (2 * side_length) + 1;
            let ratio = prime_count as f64 / total_count as f64;
            if side_length < 10 || (side_length % 1000) == 0 {
                debug!(
                    "Found side length {} with ratio {}/{} ({:.2}%)",
                    side_length, prime_count, total_count, ratio
                );
            }
            if ratio < 0.1 {
                debug_assert!(!INCORRECT_SOLUTIONS.contains(&side_length), "Incorrect solution: {}", side_length);
                return Ok(format!("{}", side_length));
            }
        }
        unreachable!() // diagonals are infinite
    }
}

#[cfg(test)]
mod test {
    use super::corners;
    use itertools::Itertools;
    #[test]
    fn test_diagonals() {
        assert_eq!(
            corners().take_until_length(3).collect_vec(),
            vec![3, 5, 7, 9]
        );
        assert_eq!(
            corners().take_until_length(5).collect_vec(),
            vec![3, 5, 7, 9, 13, 17, 21, 25]
        );
        assert_eq!(
            corners().take_until_length(7).collect_vec(),
            vec![3, 5, 7, 9, 13, 17, 21, 25, 31, 37, 43, 49]
        );
    }
}