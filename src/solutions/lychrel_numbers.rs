use super::{EulerContext, EulerProblem};

use utils::BigDigits;

use failure::Error;

#[derive(Default)]
pub struct LychrelNumbersProblem;
impl EulerProblem for LychrelNumbersProblem {
    #[inline]
    fn name(&self) -> &'static str {
        "lychrel_numbers"
    }

    #[inline]
    fn solve(&self, _: &EulerContext) -> Result<String, Error> {
        let mut count = 0;
        for value in 0..10_000 {
            count += is_lycrell_number(value, 50) as u32;
        }
        Ok(format!("{}", count))
    }
}

#[inline]
pub fn is_lycrell_number(value: u64, max_iterations: u32) -> bool {
    let mut iterations = 0;
    let mut digits = BigDigits::from_value(value);
    loop {
        digits += digits.reversed();
        if digits.is_palindrome() {
            return false;
        }
        iterations += 1;
        if iterations >= max_iterations {
            return true;
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn check_examples() {
        assert!(!is_lycrell_number(47, 1));
        assert!(!is_lycrell_number(349, 3));
        assert!(!is_lycrell_number(10677, 53));
        assert!(is_lycrell_number(4994, 50));
        assert!(is_lycrell_number(196, 50));
    }
}

