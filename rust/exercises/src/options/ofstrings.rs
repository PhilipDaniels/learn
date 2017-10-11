pub fn run() {
    println!("********* Option<String> examples *********");

    demo_basic_matching();
    // demo_as_ref();
    // demo_as_mut();
    // demo_unwrap();
    // demo_take();
    // demo_logical_or_and_combinators();
    // demo_chaining();
    // demo_map();
}

fn demo_basic_matching() {
    println!(">>> demo_basic_matching");

    // Options are in the prelude, so a is automatically deduced as being of type Option<i32>.
    let a = Some("hello".to_string());
    assert!(a.is_some());
    assert!(!a.is_none());

    // Match is the most basic way of getting at an option.
    // Note the difference compared to ints. We need to use 'ref x' to avoid moving the String
    // out of the option.
    match a {
        Some(ref x) => println!("MATCH: x (from a) = {}. x is of type &String.", x),
        None => panic!("a should not be None")
    }

    // If let is equivalent to a match where we do nothing in the None.
    // Demo 'ref' again. If you remove this ref then this will no longer compile due to
    // an error in the 2nd if let.
    if let Some(ref x) = a {
        println!("IF LET: x (from a) = {}. x is of type &String.", x);
    }

    if let Some(x) = a {
        println!("IF LET: x (from a) = {}. x is of type String.", x);
    }
}


// fn demo_as_ref() {
//     println!(">>> demo_as_ref");
//     let a = Some(5);

//     // b is of type Option<&i32>.
//     // Now when we pattern match, we get an &i32 by default.
//     let b = a.as_ref();
//     if let Some(x) = b {
//         println!("IF LET: x (from b) = {}. x is of type &i32.", x);
//         //*x = 22;   Won't compile, *x is immutable.
//     }

//     // But we can "smash" the reference in the pattern too.
//     if let Some(&x) = b {
//         println!("IF LET: x (from b) = {}. x is of type i32 when & is used in the pattern.", x);
//         //x = 22;   // Won't compile, x is immutable.
//     }
// }


// fn demo_as_mut() {
//     println!(">>> demo_as_mut");
//     let mut a = Some(5);

//     {
//         // Again, but using as_mut();
//         // c is of type Option<&mut i32>.
//         // Now when we pattern match, we get an &i32 by default.
//         let c = a.as_mut();
//         if let Some(x) = c {
//             println!("IF LET: x (from c) = {}. x is of type &mut i32 and can be changed.", x);
//             *x = 22;    // Now this will compile.
//         }
//     }

//     {
//         // But we can "smash" the reference in the pattern too.
//         let c = a.as_mut();
//         if let Some(&mut x) = c {
//             println!("IF LET: x (from c) = {}. x is of type i32 when &mut is used in the pattern.", x);
//             // *x = 22;   // Won't compile, x is just plain old i32 because of the enhanced pattern.
//         }
//     }
// }

// fn demo_unwrap() {
//     println!(">>> demo_unwrap");
//     let a = Some(5);

//     let x = a.unwrap();
//     println!("x = {}", x);

//     // These are all no-ops because is a Some.
//     let x = a.unwrap_or(222);
//     assert!(x == 5);
//     let x = a.unwrap_or_default();
//     assert!(x == 5);
//     let x = a.unwrap_or_else(|| 222);
//     assert!(x == 5);

//     // Let's make a None.
//     let a : Option<i32> = None;
//     let x = a.unwrap_or(222);
//     assert!(x == 222);
//     let x = a.unwrap_or_default();      // Type needs to implement the Default trait.
//     assert!(x == 0);
//     let x = a.unwrap_or_else(|| 222);   // Result of a closure.
//     assert!(x == 222);
// }

// fn demo_take() {
//     // Take can be used to move a value from one option to another.
//     // TODO: Not sure what the point is.
//     let mut a = Some(5);
//     let b = a.take();

//     assert!(a == None);
//     assert!(b == Some(5));
// }

// fn demo_logical_or_and_combinators() {
//     // This is quite simple, but there is a wrinkle.
//     let a = Some(5);
//     let b : Option<i32> = None;

//     // For 'or', both options must be of the same type.
//     let c = a.or(b);
//     assert_eq!(c, Some(5));

//     let d = a.and(b);
//     assert_eq!(d, None);

//     // But for 'and', they can be different: you get back the second.
//     let a = Some(5);
//     let b = Some("b".to_string());
//     let d = a.and(b);
//     assert_eq!(d, Some("b".to_string()));
// }

// fn demo_chaining() {
//     println!(">>> demo_chaining");

//     // The closure (or function) passed to and_then() only gets called if 'a' is Some.
//     // Since it isn't, we never call the closure. x is of type i32 in this example.
//     // The closure must return an Option<T>.
//     let a : Option<i32> = None;
//     let result = a.and_then(|x| Some(x));
//     assert_eq!(result, None);

//     // This can be chained. None of these closures will get called.
//     let result = a.and_then(|x| Some(x)).and_then(|x| Some(x)).and_then(|x| Some(x));
//     assert_eq!(result, None);

//     // This will flow the whole way down.
//     let a = Some(2);
//     let result = a.and_then(|x| Some(x * x)).and_then(|x| Some(x * x)).and_then(|x| Some(x * x));
//     assert_eq!(result, Some(256));

//     // We can terminate in the middle, but the chain remains safe from panics.
//     let a = Some(2);
//     let result = a.and_then(|x| Some(x * x)).and_then(|x| None).and_then(|x : i32| Some(x * x));
//     assert_eq!(result, None);

//     // or_else is the other half of the pair.
//     let a = Some(2);
//     let result = a.and_then(|x| Some(x * x)).and_then(|x| None).or_else(|| Some(22));
//     assert_eq!(result, Some(22));

//     // and_then - call the closure if the value is Some
//     // or_else  - call the closure if the value is None

//     let a = Some(2);
//     let result = a.or_else(|| Some(999));
//     assert_eq!(result, Some(2));

//     // We can also use take() in chains to move the value from one place to another.
//     let mut a = Some(2);
//     let result = a.take().and_then(|x| Some(10 * x));
//     assert_eq!(a, None);
//     assert_eq!(result, Some(20));

//     // and_then can change the type of the Option.
//     let a = Some(22);
//     let result = a.and_then(|x| Some(x.to_string()));
//     assert_eq!(result, Some("22".to_string()));
// }

// fn demo_map() {
//     println!(">>> demo_map");
//     let a = Some(2);

//     // Maps an Option<T> to Option<U> by applying a function to a contained value.
//     // * Consumes the original value (see as_ref for a way around this)
//     // * and always applies a new wrapping.
//     let result = a.map(|x| x);

//     // TODO 'Consumes' is misleading. Since we are dealing with ints, they get copied and the original remains.
//     assert_eq!(a, Some(2));
//     // Here is demonstrated the property of automatically wrapping in a Some.
//     // Note that the closure does not have Some.
//     assert_eq!(result, Some(2));

//     // Like and_then(), map can change the type of the Option.
//     let result = a.map(|x| x.to_string());
//     assert_eq!(a, Some(2));
//     assert_eq!(result, Some("2".to_string()));

//     // You can call map() on a None. Nothing happens, the None is propagated and the closure is not called.
//     // TODO: The documentation is really bad here.
//     let a : Option<i32> = None;
//     let result = a.map(|x| x.to_string());
//     assert_eq!(result, None);


//     // The other two map variants basically allow us to deal with the None by
//     // supplying a default or a closure to compute a value.
//     // n.b. These do not return Options, they return values!
//     let a : Option<i32> = None;
//     let result = a.map_or("hello".to_string(), |x| x.to_string());
//     assert_eq!(result, "hello".to_string());

//     // Note that the default-calculating closure for map_or_else() takes no arguments.
//     let a : Option<i32> = None;
//     let x = 2222;
//     let result = a.map_or_else(|| x.to_string(), |x| x.to_string());
//     assert_eq!(result, "2222".to_string());
// }
