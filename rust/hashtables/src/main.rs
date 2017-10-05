use std::collections::HashMap;
use std::collections::hash_map::Entry::{Occupied, Vacant};

mod intmap;
mod stringmap;

use intmap::int_to_int;
use stringmap::int_to_string;

fn main() {
    int_to_int();
    int_to_string();
}
