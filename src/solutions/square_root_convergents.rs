
use std::fmt::{self, Write};
use std::ops::Add;

use failure::Error;
use num::rational::{Ratio, BigRational};
use num::integer::lcm;
use num::BigInt;

use solutions::EulerProblem;
use euler::EulerContext;
use utils::IntegerLogarithm;

type SimplifiedFraction = BigRational;
#[derive(Clone, Debug)]
pub enum Expansion {
    Integer(i64),
    Add(Box<Expansion>, Box<Expansion>),
    Fraction {
        numerator: Box<Expansion>,
        denominator: Box<Expansion>
    }
}
impl Expansion {
    #[inline]
    fn reciprocal(self) -> Expansion {
        Expansion::Integer(1).over(self)
    }
    #[inline]
    fn over(self, denominator: Expansion) -> Expansion {
        Expansion::Fraction {
            numerator: box self,
            denominator: box denominator
        }
    }
    fn simplify(&self) -> SimplifiedFraction {
        match *self {
            Expansion::Integer(i) => BigInt::from(i).into(),
            Expansion::Add(ref left, ref right) => {
                left.simplify() + right.simplify()
            },
            Expansion::Fraction { ref numerator, ref denominator } => {
                numerator.simplify() / denominator.simplify()
            }
        }
    }
    fn print(&self) -> String {
        let mut buffer = String::new();
        self.write_raw(&mut buffer, false).unwrap();
        buffer
    }
    fn write_raw(&self, out: &mut String, bracketed: bool) -> fmt::Result {
        match *self {
            Expansion::Integer(v) => {
                write!(out, "{}", v)?
            },
            Expansion::Add(ref first, ref second) => {
                if bracketed { out.push('(') }
                first.write_raw(out, true)?;
                out.push_str(" + ");
                second.write_raw(out, true)?;
                if bracketed { out.push(')') };
            },
            Expansion::Fraction { ref numerator, ref denominator } => {
                numerator.write_raw(out, true)?;
                out.push('/');
                denominator.write_raw(out, true)?;
            },
        }
        Ok(())
    }
}
impl From<i32> for Expansion {
    #[inline]
    fn from(a: i32) -> Self {
        Expansion::Integer(a as i64)
    }
}

impl Add for Expansion {
    type Output = Expansion;

    #[inline]
    fn add(self, rhs: Expansion) -> Expansion {
        Expansion::Add(box self, box rhs.into())
    }
}
impl Add<Expansion> for i32 {
    type Output = Expansion;

    #[inline]
    fn add(self, rhs: Expansion) -> Expansion {
        Expansion::Add(box self.into(), box rhs.into())
    }
}


#[derive(Default)]
pub struct SquareRootConvergentsProblem;
impl EulerProblem for SquareRootConvergentsProblem {
    fn name(&self) -> &'static str {
        "square_root_convergents"
    }

    fn solve(&self, _context: &EulerContext) -> Result<String, Error> {
        let mut count = 0;
        for i in 0..1000 {
            if (i + 1) % 50 == 0 {
                eprintln!("Computed {} expansions", i + 1);
            }
            let expansion = square_root_expansion(i);
            let frac = expansion.simplify();
            if numerator_has_more_digits(frac) {
                count += 1;
            }
        }
        Ok(format!("{}", count))
    }
}

fn numerator_has_more_digits(frac: SimplifiedFraction) -> bool {
    frac.numer().count_decimal_digits() > frac.denom().count_decimal_digits()
}

/// Expansions of the continued fraction representation of `sqrt(2)`
fn square_root_expansion(count: usize) -> Expansion {
    Expansion::Integer(1).add(inner_square_root_expansion(count).reciprocal())
}

fn inner_square_root_expansion(count: usize) -> Expansion {
    if count == 0 {
        Expansion::Integer(2)
    } else {
        Expansion::Integer(2).add(
            inner_square_root_expansion(count - 1).reciprocal())
    }
}

#[cfg(test)]
mod test {
    use super::*;
    fn examples() -> Vec<(Option<&'static str>, BigRational)> {
        vec![
            (Some("1 + 1/2"), From::from((3, 2))),
            (Some("1 + 1/(2 + 1/2)"), Ratio::new(7, 5)),
            (Some("1 + 1/(2 + 1/(2 + 1/2))"), Ratio::new(17, 12)),
            (Some("1 + 1/(2 + 1/(2 + 1/(2 + 1/2)))"), Ratio::new(41, 29)),
            (None, Ratio::new(99, 70)),
            (None, Ratio::new(239, 169)),
            (None, Ratio::new(577, 408)),
            (None, Ratio::new(1393, 985))
        ].into_iter().map(|(opt, frac): (_, Ratio<i32>)| {
            (opt, BigRational::new((*frac.numer()).into(), (*frac.denom()).into()))
        }).collect()
    }
    #[test]
    fn check_examples() {
        for (index, (text, expected_frac)) in examples().into_iter().enumerate() {
            let expansion = square_root_expansion(index);
            if let Some(text) = text {
                assert_eq!(text, expansion.print());
            }
            let frac = expansion.simplify();
            assert_eq!(frac, expected_frac);
            assert_eq!(numerator_has_more_digits(frac), index == 7)
        }
    }
    #[test]
    #[ignore] // too slow
    fn check_answer() {
        assert_eq!(
            SquareRootConvergentsProblem::solve_default(),
            "153"
        );
    }
}
