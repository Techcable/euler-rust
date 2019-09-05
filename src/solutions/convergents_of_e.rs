use utils::{ContinuedFraction, Digits, BigDigits};

pub fn solve() -> u64 {
    // NOTE: Project Euler counts from one here, so 99th convergenet is at index 100
    let numer = ContinuedFraction::e(99).eval_big_convergent(99).numer().clone();
    BigDigits::from_big_value(numer)
        .as_slice().iter().map(|&digit| digit as u64).sum()
}