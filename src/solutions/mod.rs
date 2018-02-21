use failure::Error;

mod poker;
mod prime_digit_replacements;
mod lychrel_numbers;

use euler::EulerContext;

pub trait EulerProblem {
    fn name(&self) -> &'static str;
    fn solve(&self, context: &EulerContext) -> Result<String, Error>;
}

macro_rules! euler_problems {
    ($target:ident, { $($name:expr => $problem:path),* }) => {
        Ok(match $target {
            $ ( $name => Box::new(< $ problem as Default >::default()), ) *
            _ => return Err(format_err!("Unknown problem: {}", $target))
        })
    };
}
pub fn create_problem(name: &str) -> Result<Box<EulerProblem>, Error> {
    euler_problems!(name, {
        "poker" => self::poker::PokerProblem,
        "lychrel_numbers" => self::lychrel_numbers::LychrelNumbersProblem,
        "prime_digit_replacements" => self::prime_digit_replacements::PrimeDigitReplacementProblem
    })
}

