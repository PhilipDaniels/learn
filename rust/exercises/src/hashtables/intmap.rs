use std::collections::HashMap;
use std::collections::hash_map::Entry::{Occupied, Vacant};

pub fn run() {
    println!("********* HashMap<i32, i32> examples *********");

    let mut hash : HashMap<i32, i32> = HashMap::new();
    hash.insert(0, 100);
    hash.insert(1, 200);
    hash.insert(2, 23);
    hash.insert(3, 423);
    hash.insert(4, 5);

    // Iterate the keys.
    // The iterator element type is &'a K. In this case, &i32.
    for k in hash.keys() {
        println!("k = {}", k);
    }

    // Iterate the values.
    // The iterator element type is &'a V. In this case, &i32.
    for v in hash.values() {
        println!("v = {}", v);
    }

    // Iterate the values mutably.
    // The iterator element type is &'a mut V. In this case, &mut i32.
    // So we can multiply everything by 2.
    for v in hash.values_mut() {
        println!("v = {}", v);
        *v *= 2;
    }

    // Dumping again shows we did multiply all the values.
    for v in hash.values() {
        println!("v = {}", v);
    }

    // Iterate the KVPs.
    // The iterator element type is a tuple, (&'a K, &'a V). In this case, (&'a i32, &'a i32).
    for (k, v) in hash.iter() {
        println!("hash[{}] = {}", k, v);
    }

    // Iterate the KVPs mutably (just the value is mutable).
    // The iterator element type is a tuple, (&'a K, &'a mut V). In this case, (&'a i32, &'a mut i32).
    for (k, v) in hash.iter_mut() {
        println!("hash[{}] = {}", k, v);
        *v = *v / 2;
        println!("hash[{}] = {}", k, v);
    }

    // Manipulate an entry in place.
    {
        let val = hash.entry(12).or_insert(200);
        println!("We just inserted the value of {}", val);
    }

    {
        let val = hash.entry(12).or_insert(1200);
        println!("Second time around, we got back the original value of {}", val);
    }

    {
        let val = hash.entry(13).or_insert_with(|| 2 * 20);
        println!("The thing that gets inserted can be a lambda {}", val);
    }

    {
        // There is not much you can do with a VacantEntry except insert something.
        match hash.entry(200) {
            Occupied(entry) => println!("The entry key is {}", entry.key()),
            Vacant(entry) => {
                println!("There is no entry, but we can insert one");
                entry.insert(99);
            }
        }

        // But an OccupiedEntry has some interesting methods. See the docs.
        match hash.entry(200) {
            Occupied(entry) => println!("The entry key is {}", entry.key()),
            Vacant(entry) => {
                println!("There is no entry, but we can insert one");
                entry.insert(99);
            }
        }
    }

    // v is of type &i32.
    match hash.get(&3) {
        Some(v) => println!("Found value of {}", v),
        None => println!("No entry for key of 3")
    }

    // v is of type &mut i32.
    match hash.get_mut(&3) {
        Some(v) => { *v = 3 * *v; println!("Found value of {}", v) },
        None => println!("No entry for key of 3")
    }

    // retain allows us to remove items that we don't want.
    println!("Retaining entries where value is less than 50");
    hash.retain(|&k, &mut v| v < 50);
    for (k, v) in hash.iter() {
        println!("hash[{}] = {}", k, v);
    }

    // Drain gives us an iterator of tuples (like iter()).
    // After we use the iterator, the hash is empty.
    let d : Vec<_> = hash.drain().collect();
    println!("After drain, the hash has {} entries", hash.len());
}
