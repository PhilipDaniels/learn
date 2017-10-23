pub fn run() {
    println!("********* Option<String> examples *********");

    demo_basic_matching();
    demo_as_ref();
    demo_as_mut();
    demo_unwrap();
    demo_take();
    demo_logical_or_and_combinators();
    demo_chaining();
    demo_map();
    // demo_collection_of_options();
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


fn demo_as_ref() {
    println!(">>> demo_as_ref");
    let a = Some("hello".to_string());

    // b is of type Option<&String>.
    // Now when we pattern match, we get an &String by default.
    let b = a.as_ref();
    if let Some(x) = b {
        println!("IF LET: x (from b) = {}. x is of type &String.", x);
        // *x = "world".to_string();   // Won't compile, *x is immutable.
    }

    // Trying to use & in this case (compare to ints) won't work because we cannot move the String.
    // let b = a.as_ref();
    // if let Some(&x) = b {
    // }
}

fn demo_as_mut() {
    println!(">>> demo_as_mut");
    let mut a = Some("hello".to_string());

    {
        // Again, but using as_mut();
        // c is of type Option<&mut String>.
        // Now when we pattern match, we get an &mut String by default.
        let c = a.as_mut();
        if let Some(x) = c {
            println!("IF LET: x (from c) = {}. x is of type &mut i32 and can be changed.", x);
            x.push_str(" world");   // Now this will compile.
        }
    }

    {
        // Similarly to the as_ref() case, we can't use the & in the pattern match because
        // it will try to move the content out into x.
        // let c = a.as_mut();
        // if let Some(&mut x) = c {
        // }
    }
}

fn demo_unwrap() {
    println!(">>> demo_unwrap");
    let a = Some("hello".to_string());

    let x = a.unwrap();
    println!("x = {}", x);

    // These are all no-ops because is a Some.
    let a = Some("hello".to_string());
    let x = a.unwrap_or("unk".to_string());
    assert!(x == "hello".to_string());

    let a = Some("hello".to_string());
    let x = a.unwrap_or_default();
    assert!(x == "hello".to_string());

    let a = Some("hello".to_string());
    let x = a.unwrap_or_else(|| "unk".to_string());
    assert!(x == "hello".to_string());

    // Let's make a None.
    let a : Option<String> = None;
    let x = a.unwrap_or("unk".to_string());
    assert!(x == "unk".to_string());

    let a : Option<String> = None;
    let x = a.unwrap_or_default();      // Type needs to implement the Default trait.
    assert!(x == String::new());

    let a : Option<String> = None;
    let x = a.unwrap_or_else(|| "unk".to_string());   // Result of a closure.
    assert!(x == "unk".to_string());
}

fn demo_take() {
    // Take can be used to move a value from one option to another.
    // TODO: Not sure what the point is.
    let mut a = Some("hello".to_string());
    let b = a.take();

    assert!(a == None);
    assert!(b == Some("hello".to_string()));
}

fn demo_logical_or_and_combinators() {
    // This is quite simple, but there is a wrinkle.
    let a = Some("hello".to_string());
    let b : Option<String> = None;

    // For 'or', both options must be of the same type.
    let c = a.or(b);
    assert_eq!(c, Some("hello".to_string()));


    // Reset.
    let a = Some("hello".to_string());
    let b : Option<String> = None;

    let d = a.and(b);
    assert_eq!(d, None);

    // But for 'and', they can be different: you get back the second.
    let a = Some(5);
    let b = Some("b".to_string());
    let d = a.and(b);
    assert_eq!(d, Some("b".to_string()));
}

fn demo_chaining() {
    println!(">>> demo_chaining");

    // The closure (or function) passed to and_then() only gets called if 'a' is Some.
    // Since it isn't, we never call the closure. x is of type i32 in this example.
    // The closure must return an Option<T>.
    let a : Option<String> = None;
    let result = a.and_then(|x| Some(x));
    assert_eq!(result, None);

    // This can be chained. None of these closures will get called.
    let a : Option<String> = None;
    let result = a.and_then(|x| Some(x)).and_then(|x| Some(x)).and_then(|x| Some(x));
    assert_eq!(result, None);

    // This will flow the whole way down. Notice that we have to specify mut x for the closure
    // parameter in order to be able to call push_str().
    let a = Some("hello".to_string());
    let result = a.and_then(|mut x| { x.push_str(" world"); Some(x) })
        .and_then(|mut x| { x.push_str(" world"); Some(x) })
        .and_then(|mut x| { x.push_str(" world"); Some(x) });
    assert_eq!(result, Some("hello world world world".to_string()));

    // We can terminate in the middle, but the chain remains safe from panics.
    // Notice that we can even bugger up the types in the last closure. This is probably because
    // rustc deduces the type of the None as being of Option<i32>, hence the last closure compiles.
    let a = Some("hello".to_string());
    let result = a.and_then(|mut x| { x.push_str(" world"); Some(x) })
        .and_then(|x| None)
        .and_then(|x : i32| Some(x * x));
    assert_eq!(result, None);

    // or_else is the other half of the pair.
    let a = Some("hello".to_string());
    let result = a.and_then(|mut x| { x.push_str(" world"); Some(x) })
        .and_then(|x| None)
        .or_else(|| Some(22));
    assert_eq!(result, Some(22));

    // and_then - call the closure if the value is Some
    // or_else  - call the closure if the value is None

    let a = Some("hello".to_string());
    let result = a.or_else(|| Some("unk".to_string()));
    assert_eq!(result, Some("hello".to_string()));

    // We can also use take() in chains to move the value from one place to another.
    let mut a = Some("hello".to_string());
    let result = a.take().and_then(|mut x| { x.push_str(" world"); Some(x) });
    assert_eq!(a, None);
    assert_eq!(result, Some("hello world".to_string()));

    // // and_then can change the type of the Option.
    let a = Some("hello".to_string());
    let result = a.and_then(|x| Some(x.len()));
    assert_eq!(result, Some(5));
}

fn demo_map() {
    println!(">>> demo_map");
    let a = Some("hello".to_string());

    // Maps an Option<T> to Option<U> by applying a function to a contained value.
    // * Consumes the original value (see as_ref for a way around this)
    // * and always applies a new wrapping.
    let result = a.map(|x| x);

    // TODO 'Consumes' is NOT misleading in the case of Strings.
    // This won't compile because a has been moved.
    // assert_eq!(a, None);
    // Here is demonstrated the property of automatically wrapping in a Some.
    // Note that the closure does not have Some.
    assert_eq!(result, Some("hello".to_string()));

    // Like and_then(), map can change the type of the Option.
    let a = Some("hello".to_string());
    let result = a.map(|x| x.len());
    // assert_eq!(a, Some("hello".to_string()));     Again, won't compile because a is moved.
    assert_eq!(result, Some(5));

    // You can call map() on a None. Nothing happens, the None is propagated and the closure is not called.
    // TODO: The documentation is really bad here.
    let a : Option<String> = None;
    let result = a.map(|x| x.len());
    assert_eq!(result, None);


    // The other two map variants basically allow us to deal with the None by
    // supplying a default or a closure to compute a value.
    // n.b. These do not return Options, they return values!
    let a : Option<String> = None;
    let result = a.map_or("hello".len(), |x| x.len());
    assert_eq!(result, 5);

    // Note that the default-calculating closure for map_or_else() takes no arguments.
    let a : Option<String> = None;
    let x = "hello".to_string();
    let result = a.map_or_else(|| x.len(), |y| y.len());
    assert_eq!(result, 5);
}
