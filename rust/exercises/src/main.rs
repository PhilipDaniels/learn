#![allow(dead_code)]
#![allow(warnings)]

mod hashtables;

fn main() {
    hashtables::intmap::run();
    hashtables::stringmap::run();
    hashtables::ofrefcellvalues::run();
}
