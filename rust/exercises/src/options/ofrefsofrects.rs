use rect::Rect;

pub fn run() {
    println!("********* Option<&Rect> examples *********");

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

    // We need to keep r around for the lifetime checker.
    let r = Rect::demo();
    let a = Some(&r);
    assert!(a.is_some());
    assert!(!a.is_none());

    // Match is the most basic way of getting at an option.
    // Note that x is now &Rect automatically, so we do not need 'ref x'.
    match a {
        Some(x) => println!("MATCH: x (from a) = {}. x is of type &Rect.", x),
        None => panic!("a should not be None")
    }

    // If let is equivalent to a match where we do nothing in the None.
    // Because x is automatically a borrow, both of these compile fine.
    if let Some(x) = a {
        println!("IF LET: x (from a) = {}. x is of type &Rect.", x);
    }

    if let Some(x) = a {
        println!("IF LET: x (from a) = {}. x is of type &Rect.", x);
    }
}

fn demo_as_ref() {
    println!(">>> demo_as_ref");
    let r = Rect::demo();
    let a = Some(&r);

    // b is of type Option<&&Rect>.
    // Now when we pattern match, we get an &&Rect by default.
    let b = a.as_ref();
    if let Some(x) = b {
        println!("IF LET: x (from b) = {}. x is of type &&Rect.", x);
    }

    // Trying to use & in this case works, we get rid of one of the references.
    let b = a.as_ref();
    if let Some(&x) = b {
    }
}

fn demo_as_mut() {
    println!(">>> demo_as_mut (not sure what is going on here, to be honest)");
    let r = Rect::demo();
    let mut a = Some(&r);

    {
        // Again, but using as_mut();
        // c is of type Option<&mut &Rect>.
        // Now when we pattern match, we get an &mut &Rect by default.
        let c = a.as_mut();
        if let Some(x) = c {
            println!("IF LET: x (from c) = {}. x is of type &mut i32 and can be changed.", x);
            // x.w = 200;           // Cannot assign to immutable field x.w
        }
    }

    {
        // We can now pattern match using &mut x.
        let c = a.as_mut();
        if let Some(&mut x) = c {
        }
    }
}

fn demo_unwrap() {
    println!(">>> demo_unwrap");
    let r = Rect::demo();
    let r2 = Rect::demo2();
    let a = Some(&r);

    let x = a.unwrap();
    println!("x = {}", x);

    // These are all no-ops because is a Some.
    let a = Some(&r);
    let x = a.unwrap_or(&r2);
    assert!(x.title == "first demo");

    // Won't compile, T is &Rect, not Rect.
    // let a = Some(&r);
    // let x = a.unwrap_or_default();
    // assert!(x == "hello".to_string());

    let a = Some(&r);
    let x = a.unwrap_or_else(|| &r2);
    assert!(x.title == "first demo");

    // Let's make a None.
    let a : Option<&Rect> = None;
    let x = a.unwrap_or(&r2);
    assert!(x.title == "second demo");

    // let a : Option<&Rect> = None;
    // let x = a.unwrap_or_default();      // Type needs to implement the Default trait, &Rect doesn't.
    // assert!(x == String::new());

    let a : Option<&Rect> = None;
    let x = a.unwrap_or_else(|| &r2);   // Result of a closure.
    assert!(x.title == "second demo");
}

fn demo_take() {
    println!(">>> demo_take");

    // Take can be used to move a value from one option to another.
    let r = Rect::demo();
    let mut a = Some(&r);

    let b = a.take();

    assert!(a.is_none());
    match b {
        Some(x) => assert!(x.title == "first demo"),
        None => panic!("a should not be None")
    }
}

fn demo_logical_or_and_combinators() {
    // This is quite simple, but there is a wrinkle.
    let r = Rect::demo();
    let a = Some(&r);
    let b : Option<&Rect> = None;

    // For 'or', both options must be of the same type.
    let c = a.or(b).unwrap();
    assert_eq!(c.title, "first demo");


    // Reset.
    let a = Some(&r);
    let b : Option<&Rect> = None;

    let d = a.and(b);
    assert!(d.is_none());

    // But for 'and', they can be different: you get back the second.
    let a = Some(&r);
    let b = Some("b".to_string());
    let d = a.and(b);
    assert_eq!(d, Some("b".to_string()));
}

fn demo_chaining() {
    println!(">>> demo_chaining");

    // The closure (or function) passed to and_then() only gets called if 'a' is Some.
    // Since it isn't, we never call the closure. x is of type i32 in this example.
    // The closure must return an Option<T>.
    let a : Option<&Rect> = None;
    let result = a.and_then(|x| Some(x));
    assert!(result.is_none());

    // This can be chained. None of these closures will get called.
    let a : Option<&Rect> = None;
    let result = a.and_then(|x| Some(x)).and_then(|x| Some(x)).and_then(|x| Some(x));
    assert!(result.is_none());

    // This will flow the whole way down. Notice that we have to specify mut x for the closure
    // parameter in order to be able to call app(), and note also that we create the Some
    // with a '&mut r'!! It won't compile without it.
    let mut r = Rect::demo();
    let a = Some(&mut r);
    let result = a.and_then(|mut x| { x.app(" world"); Some(x) })
        .and_then(|mut x| { x.app(" world"); Some(x) })
        .and_then(|mut x| { x.app(" world"); Some(x) });
    assert_eq!(result.unwrap().title, "first demo world world world");

    // We can terminate in the middle, but the chain remains safe from panics.
    // Notice that we can even bugger up the types in the last closure. This is probably because
    // rustc deduces the type of the None as being of Option<i32>, hence the last closure compiles.
    let mut r = Rect::demo();
    let a = Some(&mut r);
    let result = a.and_then(|mut x| { x.app(" world"); Some(x) })
        .and_then(|x| None)
        .and_then(|x : i32| Some(x * x));
    assert_eq!(result, None);

    // or_else is the other half of the pair.
    let mut r = Rect::demo();
    let a = Some(&mut r);
    let result = a.and_then(|mut x| { x.app(" world"); Some(x) })
        .and_then(|x| None)
        .or_else(|| Some(22));
    assert_eq!(result, Some(22));

    // and_then - call the closure if the value is Some
    // or_else  - call the closure if the value is None

    let mut r = Rect::demo();
    let mut r2 = Rect::demo2();
    let a = Some(&mut r);
    let result = a.or_else(|| Some(&mut r2)).unwrap();
    assert_eq!(result.title, "first demo");

    // We can also use take() in chains to move the value from one place to another.
    let mut r = Rect::demo();
    let mut a = Some(&mut r);
    let result = a.take().and_then(|mut x| { x.app(" world"); Some(x) }).unwrap();
    assert!(a.is_none());
    assert_eq!(result.title, "first demo world");

    // and_then can change the type of the Option.
    let mut r = Rect::demo();
    let mut a = Some(&mut r);
    let result = a.and_then(|x| Some(x.title.len()));
    assert_eq!(result, Some(10));
}

fn demo_map() {
    println!(">>> demo_map");
    let mut r = Rect::demo();
    let mut a = Some(&mut r);

    // Maps an Option<T> to Option<U> by applying a function to a contained value.
    // * Consumes the original value (see as_ref for a way around this)
    // * and always applies a new wrapping.
    let result = a.map(|x| x);

    // TODO 'Consumes' is NOT misleading in the case of references to things.
    // This won't compile because a has been moved.
    //assert!(a.is_none());
    // Here is demonstrated the property of automatically wrapping in a Some.
    // Note that the closure does not have Some.
    let t = result.unwrap();
    assert_eq!(t.title, "first demo");

    // Like and_then(), map can change the type of the Option.
    let mut r = Rect::demo();
    let mut a = Some(&mut r);
    let result = a.map(|x| x.title.len());
    // assert_eq!(a, ...);     Again, won't compile because a is moved.
    assert_eq!(result, Some(10));

    // You can call map() on a None. Nothing happens, the None is propagated and the closure is not called.
    // TODO: The documentation is really bad here.
    let a : Option<&Rect> = None;
    let result = a.map(|x| x.title.len());
    assert!(result.is_none());


    // The other two map variants basically allow us to deal with the None by
    // supplying a default or a closure to compute a value.
    // n.b. These do not return Options, they return values!
    let a : Option<&Rect> = None;
    let result = a.map_or("hello".len(), |x| x.title.len());
    assert_eq!(result, 5);

    // Note that the default-calculating closure for map_or_else() takes no arguments.
    let a : Option<&Rect> = None;
    let x = "hello".to_string();
    let result = a.map_or_else(|| x.len(), |y| y.title.len());
    assert_eq!(result, 5);
}
