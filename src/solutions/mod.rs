use failure::Error;

mod poker;
mod prime_digit_replacements;
mod lychrel_numbers;
mod powerful_digit_sum;
mod square_root_convergents;
mod spiral_primes;
mod xor_decryption;

pub trait EulerSolution: Sized + ::std::fmt::Display {
    #[inline]
    fn into_string(self) -> String {
        format!("{}", self)
    }
}
macro_rules! impl_solution {
    ($($target:ty),*) => {
        $(impl EulerSolution for $target {})*
    };
}
impl_solution!(u32, i32, u64, i64);
impl EulerSolution for String {
    #[inline]
    fn into_string(self) -> String {
        self
    }
}
pub trait EulerResult {
    fn into_result(self) -> Result<String, Error>;
}
impl<T: EulerSolution> EulerResult for Result<T, Error> {
    #[inline]
    fn into_result(self) -> Result<String, Error> {
        self.map(EulerSolution::into_string)
    }
}
impl<T: EulerSolution> EulerResult for T {
    #[inline]
    fn into_result(self) -> Result<String, Error> {
        Ok(self.into_string())
    }
}
pub struct EulerProblem {
    name: &'static str,
    func: Box<Fn() -> Result<String, Error> + Send + Sync + 'static>
}
impl EulerProblem {
    #[inline]
    pub fn new<R>(name: &'static str, func: fn() -> R) -> Self where  R: EulerResult + 'static {
        let func = Box::new(move || func().into_result());
        EulerProblem { name, func }
    }
    #[inline]
    pub fn solve(&self) -> Result<String, Error> {
        (self.func)()
    }
}



macro_rules! euler_problems {
    ($target:ident, { $($problem:ident),* }) => {
        Ok(match $target {
            $ ( stringify!($problem) => EulerProblem::new(stringify!($problem), self::$problem::solve), ) *
            _ => return Err(format_err!("Unknown problem: {}", $target))
        })
    };
}
pub fn create_problem(name: &str) -> Result<EulerProblem, Error> {
    euler_problems!(name, {
        poker, lychrel_numbers,
        prime_digit_replacements,
        powerful_digit_sum,
        square_root_convergents,
        spiral_primes, xor_decryption
    })
}

