use std::collections::HashMap;
use std::collections::hash_map::Entry::{Occupied, Vacant};

pub fn run() {
    println!("********* int_to_string *********");

    let mut hash : HashMap<i32, String> = HashMap::new();
    hash.insert(0, String::from("hello"));
    hash.insert(1, String::from("world"));
    hash.insert(2, String::from("a"));
    hash.insert(3, String::from("z"));
    hash.insert(4, String::from("phil"));

    // Iterate the keys.
    // The iterator element type is &'a K. In this case, &String.
    for k in hash.keys() {
        println!("k = {}", k);
    }

    // Iterate the values.
    // The iterator element type is &'a V. In this case, &String.
    for v in hash.values() {
        println!("v = {}", v);
    }

    // Iterate the values mutably.
    // The iterator element type is &'a mut V. In this case, &mut String.
    // So we can append to the strings.
    for v in hash.values_mut() {
        println!("v = {}", v);
        v.push_str(" suffix");
    }

    // Dumping again shows we did append all the values.
    for v in hash.values() {
        println!("v = {}", v);
    }

    // Iterate the KVPs.
    // The iterator element type is a tuple, (&'a K, &'a V). In this case, (&'a i32, &'a String).
    for (k, v) in hash.iter() {
        println!("hash[{}] = {}", k, v);
    }

    // Iterate the KVPs mutably (just the value is mutable).
    // The iterator element type is a tuple, (&'a K, &'a mut V). In this case, (&'a i32, &'a mut String).
    for (k, v) in hash.iter_mut() {
        println!("hash[{}] = {}", k, v);
        v.push_str("/2");
        println!("hash[{}] = {}", k, v);
    }

    // Manipulate an entry in place.
    {
        let val = hash.entry(12).or_insert(String::from("new"));
        println!("We just inserted the value of {}", val);
    }

    {
        let val = hash.entry(12).or_insert(String::from("new2"));
        println!("Second time around, we got back the original value of {}", val);
    }

    {
        let val = hash.entry(13).or_insert_with(|| String::from("new 3"));
        println!("The thing that gets inserted can be a lambda {}", val);
    }

    {
        // There is not much you can do with a VacantEntry except insert something.
        match hash.entry(200) {
            Occupied(entry) => println!("The entry key is {}", entry.key()),
            Vacant(entry) => {
                println!("There is no entry, but we can insert one");
                entry.insert(String::from("insert for vacant entry"));
            }
        }

        // But an OccupiedEntry has some interesting methods. See the docs.
        match hash.entry(200) {
            Occupied(entry) => println!("The entry key is {}", entry.key()),
            Vacant(entry) => {
                println!("There is no entry, but we can insert one");
                entry.insert(String::from("insert for vacant entry"));
            }
        }
    }

    // v is of type &String.
    match hash.get(&3) {
        Some(v) => println!("Found value of {}", v),
        None => println!("No entry for key of 3")
    }

    // v is of type &mut String.
    match hash.get_mut(&3) {
        Some(v) => {
            let x = v.clone();
            v.push_str(&x);
            println!("Found value of {}", v)
        },
        None => println!("No entry for key of 3")
    }

    // retain allows us to remove items that we don't want.
    println!("Retaining entries where length is less than 20");
    hash.retain(|&k, ref mut v| v.len() < 20);
    for (k, v) in hash.iter() {
        println!("hash[{}] = {}", k, v);
    }

    // Drain gives us an iterator of tuples (like iter()).
    // After we use the iterator, the hash is empty.
    let d : Vec<_> = hash.drain().collect();
    println!("After drain, the hash has {} entries", hash.len());
}
