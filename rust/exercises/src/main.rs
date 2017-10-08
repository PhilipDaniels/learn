#![allow(dead_code)]
#![allow(warnings)]

mod hashtables;
mod vectors;

fn main() {
    hashtables::intmap::run();
    hashtables::stringmap::run();
    hashtables::ofrefcellvalues::run();
    vectors::ofints::run();
    vectors::ofstrings::run();
}
