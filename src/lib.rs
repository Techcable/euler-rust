#![feature(inclusive_range_syntax, const_fn)]
#![allow(dead_code, unused_imports)]
#[macro_use]
extern crate failure;

use failure::Error;

pub mod euler;
pub mod solutions;

pub fn solve_problem(name: &str) -> Result<String, Error> {
    let context = ::euler::EulerContext::new(name.to_owned());
    ::solutions::create_problem(name)?
        .solve(&context)
}
