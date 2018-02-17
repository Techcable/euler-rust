extern crate euler;
#[macro_use]
extern crate clap;
extern crate failure;

use std::process::exit;

fn app() -> ::clap::App<'static, 'static> {
    clap_app!(euler =>
        (version: crate_version!())
        (author: crate_authors!())
        (about: crate_description!())
        (@arg problem: +required "The name of the problem to solve")
    )
}

fn main() {
    let matches = app().get_matches();
    let name = matches.value_of("problem").unwrap();
    match ::euler::solve_problem(name) {
        Ok(result) => {
            println!("Solved {}: {}", name, result)
        },
        Err(error) => {
            eprintln!("Failed to solve {}: {}", name, error);
            exit(1)
        }
    }
}
