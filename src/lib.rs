#![feature(const_fn, box_syntax, extra_log_consts, duration_float)]
#![allow(dead_code, unused_imports)]
#[macro_use]
extern crate failure;
extern crate fixedbitset;
extern crate ndarray;
extern crate itertools;
extern crate num;
#[macro_use]
extern crate lazy_static;
extern crate num_traits;
#[macro_use]
extern crate log;
extern crate arrayvec;
extern crate env_logger;

use failure::Error;

pub mod solutions;
pub mod utils;

pub fn solve_problem(name: &str) -> Result<String, Error> {
    ::solutions::create_problem(name)?.solve()
}
