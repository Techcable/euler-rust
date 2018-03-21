#![feature(const_fn)]
#![allow(dead_code, unused_imports)]
#[macro_use]
extern crate failure;
extern crate fixedbitset;
extern crate ndarray;
extern crate itertools;
extern crate num;
#[macro_use]
extern crate lazy_static;

use failure::Error;

pub mod euler;
pub mod solutions;
pub mod utils;

pub fn solve_problem(name: &str) -> Result<String, Error> {
    let context = ::euler::EulerContext::new(name.to_owned());
    ::solutions::create_problem(name)?
        .solve(&context)
}
