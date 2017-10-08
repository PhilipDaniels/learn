// print_vec takes an immutable_slice. The elements in the slice are not changeable.
fn pass_immutable_slice(slice: &[i32]) {
    // Won't compile.
    // slice[0] = 20;
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
    // Remove the last element. Returns Option<T>: None if the vec is empty.
    let elem = vec.pop().unwrap();

    // Create using a macro.
    let mut vec = vec![10, 20, 30, 40, 50];

    // If a function does not need to add, remove or change elements, then pass an immutable slice.
    pass_immutable_slice(&vec);
    pass_immutable_slice(&vec[0..4]);

    vec.truncate(2);
    println!("vec after truncating to length of 2: {:?}", &vec);

    // These two calls are equivalent.
    pass_mutable_slice(&mut vec);
    pass_mutable_slice(vec.as_mut_slice());

    println!("vec after mutating a slice: {:?}", &vec);

    // Summary
    //     &vec is equivalent to vec.as_slice() and yields an immutable slice
    //     &mut vec is equivalent to vec.as_mut_slice() and yields a mutable slice.

    vec.insert(0, 50);              // Insert an element at index = 0.
    let elem = vec.remove(1);       // Remove the element at index = 1.
    println!("After insert and remove: {:?}", &vec);

    // Reinitialize.
    let mut vec = vec![10, 20, 30, 40, 50];
    vec.retain(|&elem| elem == 20 || elem == 40);

    println!("After retaining only elements matching a predicate: {:?}", &vec);

    // Try that again, using a lambda for the predicate.
    // The two predicates are the same. Not sure why.
    let mut vec = vec![10, 20, 30, 40, 50];
    let pred = |elem: &i32| *elem == 20 || *elem == 40;
    let pred = |&elem: &i32| elem == 20 || elem == 40;
    vec.retain(&pred);
    println!("After retaining only elements matching a predicate (lambda): {:?}", &vec);

    // Negating the predicate, not sure if this is the best way.
    vec.retain(|&elem| !pred(&elem));

    // We can append a vec, but it does a 'move' of all the elements from the vec2 we are appending.
    let mut vec = vec![10, 20, 30, 40, 50];
    println!("vec before append of vec2: {:?}", &vec);
    let mut vec2 = vec![-10, -20, -30];
    vec.append(&mut vec2);
    println!("vec after append of vec2: {:?}", &vec);

    println!("vec2 after append to vec (it's empty): {:?}", &vec2);

    // We have a lot of methods from come from slices, because vectors implement "Deref<Target = [T]>"
    vec.reverse();
    vec.reverse();

    // drain allows us to remove slices of vectors.
    let removed: Vec<_> = vec.drain(2..5).collect();
    println!("vec after draining elements 2..5: {:?}", &vec);
    println!("The items drained: {:?}", &removed);

    // Clear is easy enough.
    vec.clear();

    // split_off allows us to split a vector into two. Half-empty range, as usual.
    let mut vec = vec![10, 20, 30, 40, 50];
    let vec2 = vec.split_off(2);
    assert_eq!(vec, [10, 20]);
    assert_eq!(vec2, [30, 40, 50]);

    // Change length to 5, fill with value of 4.
    vec.resize(5, 4);
    println!("vec after resizing and filling with a value of 4: {:?}", &vec);

    vec.extend_from_slice(&[100, 200]);
    println!("vec after extend_from_slice: {:?}", &vec);

    let mut vec = vec![1, 2, 2, 3, 2];
    println!("vec before dedup: {:?}", &vec);
    vec.dedup();
    println!("vec after dedup (note there is dedup_by_key as well): {:?}", &vec);



    // Get an element.
    let elem = vec[2];

    // Get a shared reference to an element. The assignment won't compile.
    let elem = &vec[2];
    //*elem = 22;

    // Get a mutable reference to an element.
    {
        let mut vec = vec![1, 2, 2, 3, 2];
        let elem = &mut vec[2];
        *elem = 22;
    }

    {
        let mut vec = vec![1, 2, 2, 3, 2];
        // Set an element.
        vec[2] = 3;
    }

    // How to append a clone.
    let mut vec = vec![1, 2, 3];
    let mut vec2 = vec![4, 5, 6];

    vec.extend_from_slice(&vec2);
    println!("vec after append_from_slice: {:?}", &vec);
    println!("vec2 after append_from_slice: {:?}", &vec2);
}
