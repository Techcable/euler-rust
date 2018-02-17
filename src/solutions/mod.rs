use failure::Error;

mod poker;

use euler::EulerContext;
use self::poker::PokerProblem;

pub trait EulerProblem {
    fn name(&self) -> &'static str;
    fn solve(&self, context: &EulerContext) -> Result<String, Error>;
}

macro_rules! euler_problems {
    ($target:ident, { $($name:expr => $problem:ident),* }) => {
        Ok(match $target {
            $ ( $name => Box::new(< $ problem as Default >::default()), ) *
            _ => return Err(format_err!("Unknown problem: {}", $target))
        })
    };
}
pub fn create_problem(name: &str) -> Result<Box<EulerProblem>, Error> {
    euler_problems!(name, {
        "poker" => PokerProblem
    })
}

