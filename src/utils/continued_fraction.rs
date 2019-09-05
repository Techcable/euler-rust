use num::rational::Ratio;
use num::{Zero, BigRational, BigInt, ToPrimitive};

pub struct ContinuedFraction {
    first_digit: u32,
    remaining: Vec<u32>
}

impl ContinuedFraction {
    pub fn new(first: u32, remaining: Vec<u32>) -> ContinuedFraction {
        ContinuedFraction { first_digit: first, remaining }
    }
    pub fn eval_convergent(&self, index: usize) -> Ratio<u64> {
        // TODO: Avoid using eval_big_convergent
        let ratio = self.eval_big_convergent(index);
        Ratio::new(
            ratio.numer().to_u64().unwrap(),
            ratio.denom().to_u64().unwrap()
        )
    }
    pub fn eval_big_convergent(&self, index: usize) -> BigRational {
        assert!(index <= self.remaining.len());
        let mut val: Option<BigRational> = None;
        for &value in self.remaining[..index].iter().rev() {
            let value = BigRational::from(BigInt::from(value));
            val = Some(match val {
                Some(existing) => {
                    existing.recip() + value
                },
                None => value
            });
        }
        BigRational::from_integer(self.first_digit.into())
            + val.map_or(BigRational::zero(), |v| v.recip())
    }
    pub fn e(len: usize) -> ContinuedFraction {
        let mut remaining = Vec::new();
        remaining.push(1);
        let mut k = 1;
        while remaining.len() < len {
            remaining.push(k * 2);
            remaining.push(1);
            remaining.push(1);
            k += 1;
        }
        ContinuedFraction {
            first_digit: 2,
            remaining
        }
    }
    pub fn sqrt2(len: usize) -> ContinuedFraction {
        ContinuedFraction {
            first_digit: 1,
            remaining: vec![2; len]
        }
    }
}

#[cfg(test)]
mod test {
    use utils::ContinuedFraction;
    use num::rational::Ratio;

    #[test]
    fn e() {
        let e = ContinuedFraction::e(20);
        assert_eq!(
            e.eval_convergent(0),
            Ratio::new(2, 1)
        );
        assert_eq!(
            e.eval_convergent(1),
            Ratio::new(3, 1)
        );
        assert_eq!(
            e.eval_convergent(2),
            Ratio::new(8, 3)
        );
        assert_eq!(
            e.eval_convergent(3),
            Ratio::new(11, 4)
        );
    }

    #[test]
    fn sqrt2() {
        let e = ContinuedFraction::sqrt2(20);
        assert_eq!(
            e.eval_convergent(0),
            Ratio::new(1, 1)
        );
        assert_eq!(
            e.eval_convergent(1),
            Ratio::new(3, 2)
        );
        assert_eq!(
            e.eval_convergent(2),
            Ratio::new(7, 5)
        );
        assert_eq!(
            e.eval_convergent(3),
            Ratio::new(17, 12)
        );
    }
}