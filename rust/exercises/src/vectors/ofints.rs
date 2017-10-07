// print_vec takes an immutable_slice. The elements in the slice are not changeable.
fn print_vec(slice: &[i32]) {
    // Won't compile.
    // slice[0] = 20;

    for (idx, elem) in slice.iter().enumerate() {
        println!("vec[{}] = {}", idx, elem);
    }
}

// We can also get a mutable slice, where we can change the elements.
fn pass_mutable_slice(slice: &mut [i32]) {
    slice[0] *= 100;
}

pub fn run() {
    println!("********* vec<i32> examples *********");

    // Create using ctor and type inference.
    let mut vec = Vec::new();
    vec.push(1);

    // Create using a macro.
    let mut vec = vec![10, 20, 30, 40, 50];

    // If a function does not need to add, remove or change elements, then pass an immutable slice.
    println!("Slicing the entire vector...");
    print_vec(&vec);
    println!("Slicing the first four elements...");
    print_vec(&vec[0..4]);

    vec.truncate(2);
    println!("Truncating to length of 2...");
    print_vec(&vec);

    // These two calls are equivalent.
    pass_mutable_slice(&mut vec);
    pass_mutable_slice(vec.as_mut_slice());

    println!("After mutating a slice...");
    print_vec(&vec);

    // Summary
    //     &vec is equivalent to vec.as_slice() and yields an immutable slice
    //     &mut vec is equivalent to vec.as_mut_slice() and yields a mutable slice.

    vec.insert(0, 50);              // Insert an element at index = 0.
    let elem = vec.remove(1);       // Remove the element at index = 1.
    println!("After insert and remove...");
    print_vec(&vec);

    // Reinitialize.
    let mut vec = vec![10, 20, 30, 40, 50];

    println!("After retaining only elements matching a predicate...");
    vec.retain(|&elem| elem == 20 || elem == 40);
    print_vec(&vec);

    // Try that again, using a lambda for the predicate.
    // The two predicates are the same. Not sure why.
    let mut vec = vec![10, 20, 30, 40, 50];
    let pred = |elem: &i32| *elem == 20 || *elem == 40;
    let pred = |&elem: &i32| elem == 20 || elem == 40;
    vec.retain(&pred);
    println!("After retaining only elements matching a predicate (lambda)...");
    print_vec(&vec);

    // Negating the predicate, not sure if this is the best way.
    vec.retain(|&elem| !pred(&elem));

    // Add an element to the end.
    vec.push(555);

    // Remove the last element. Returns Option<T>: None if the vec is empty.
    let elem = vec.pop().unwrap();

    // We can append a vec, but it does a 'move' of all the elements from the vec2 we are appending.
    let mut vec = vec![10, 20, 30, 40, 50];
    println!("vec before append of vec2...");
    print_vec(&vec);

    let mut vec2 = vec![-10, -20, -30];
    vec.append(&mut vec2);

    println!("vec after append of vec2...");
    print_vec(&vec);

    println!("vec2 after append to vec (it's empty)...");
    print_vec(&vec2);
    println!("See!");

    // We have a lot of methods from come from slices, because vectors implement "Deref<Target = [T]>"
    vec.reverse();
    vec.reverse();

    // drain allows us to remove slices of vectors.
    let removed: Vec<_> = vec.drain(2..5).collect();
    println!("vec after draining 2..5 ...");
    print_vec(&vec);
    println!("The items drained...");
    print_vec(&removed);

    // Clear is easy enough.
    vec.clear();

    // split_off allows us to split a vector into two. Half-empty range, as usual.
    let mut vec = vec![10, 20, 30, 40, 50];
    let vec2 = vec.split_off(2);
    assert_eq!(vec, [10, 20]);
    assert_eq!(vec2, [30, 40, 50]);

    // Change length to 5, fill with value of 4.
    vec.resize(5, 4);
    println!("vec after resizing and filling with a value of 4 ...");
    print_vec(&vec);

    vec.extend_from_slice(&[100, 200]);
    println!("vec after extend_from_slice ...");
    print_vec(&vec);


    let mut vec = vec![1, 2, 2, 3, 2];
    println!("vec before dedup ...");
    print_vec(&vec);
    vec.dedup();
    println!("vec after dedup (note there is dedup_by_key as well)...");
    print_vec(&vec);

    println!("vec = {:?}", &vec);;
}
