use std::str::FromStr;

use num::{FromPrimitive, Integer, Zero, Signed, BigUint, ToPrimitive, BigInt};

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
