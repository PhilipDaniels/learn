#![allow(dead_code)]
#![allow(warnings)]
#![feature(iterator_step_by)]

mod hashtables;
mod vectors;
mod options;
mod results;
mod iterators;

fn main() {
    hashtables::intmap::run();
    hashtables::stringmap::run();
    hashtables::ofrefcellvalues::run();
    vectors::ofints::run();
    vectors::ofstrings::run();
    options::ofints::run();
    options::ofstrings::run();
    options::ofrefsofthings::run();
    iterators::ofints::run();
}
