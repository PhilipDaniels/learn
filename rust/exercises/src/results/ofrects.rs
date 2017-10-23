use std::slice::Iter;
use std::iter::Filter;
use rect::*;

fn make_result<'a>() -> Result<Rect, &'a str> {
    Ok(Rect::new(10, 20, "rec"))
}

fn make_err<'a>() -> Result<Rect, &'a str> {
    Err("oops")
}

pub fn run() {
    println!("********* Option<Rect> examples *********");

    demo_basic_matching();
    demo_as_ref();
    demo_as_mut();
    demo_unwrap();
    demo_logical_or_and_combinators();
    demo_chaining();
    // demo_map();
    // demo_collection_of_options();
}

fn demo_basic_matching() {
    println!(">>> demo_basic_matching");

    // Options are in the prelude, so a is automatically deduced as being of type Result<>.
    let a = make_result();
    assert!(a.is_ok());
    assert!(!a.is_err());

    // Match is the most basic way of getting at a Result.
    // Note the difference compared to ints. We need to use 'ref x' to avoid moving the String
    // out of the option.
    match a {
        Ok(ref x) => println!("MATCH: x (from a) = {}. x is of type &Rect.", x),
        Err(e) => panic!("a should not be Err")
    }

    // If let is equivalent to a match where we do nothing in the Err.
    // Demo 'ref' again. If you remove this ref then this will no longer compile due to
    // an error in the 2nd if let.
    if let Ok(ref x) = a {
        println!("IF LET: x (from a) = {}. x is of type &Rect.", x);
    }

    if let Ok(x) = a {
        println!("IF LET: x (from a) = {}. x is of type Rect.", x);
    }
}

fn demo_as_ref() {
    println!(">>> demo_as_ref");
    let a = make_result();

    // b is of type Result<&Rect, &&str>.
    // Now when we pattern match, we get an &Rect by default.
    let b = a.as_ref();
    if let Ok(x) = b {
        println!("IF LET: x (from b) = {}. x is of type &String.", x);
        // x.w = 20;   // Won't compile, *x is immutable.
    }

    // Trying to use & in this case (compare to ints) won't work because we cannot move the Rect.
    // let b = a.as_ref();
    // if let Ok(&x) = b {
    // }
}

fn demo_as_mut() {
    println!(">>> demo_as_mut");
    let mut a = make_result();

    {
        // Again, but using as_mut();
        // b is of type Result<&mut Rect, &mut &str>.
        // Now when we pattern match, we get an &mut String by default.
        let c = a.as_mut();
        if let Ok(x) = c {
            println!("IF LET: x (from c) = {}. x is of type &mut Rect and can be changed.", x);
            x.w = 22;   // Now this will compile.
        }
    }

    {
        // Similarly to the as_ref() case, we can't use the & in the pattern match because
        // it will try to move the content out into x.
        // let c = a.as_mut();
        // if let Ok(&mut x) = c {
        // }
    }
}

fn demo_unwrap() {
    println!(">>> demo_unwrap");
    let a = make_result();

    let x = a.unwrap();
    println!("x = {}", x);

    // These are all no-ops because is an Ok.
    let a = make_result();
    let x = a.unwrap_or(Rect::new(1,1,"a"));
    assert!(x.title == "rec");

    let a = make_result();
    let x = a.unwrap_or_default();
    assert!(x.title == "rec");

    // In contrast to Option<>, for unwrap_or_else we get passed an argument, the err.
    let a = make_result();
    let x = a.unwrap_or_else(|err| Rect::new(1,1,"a"));
    assert!(x.title == "rec");

    // Let's make an Err.
    let a = make_err();
    let x = a.unwrap_or(Rect::new(1,1,"a"));
    assert!(x.title == "a");

    let a = make_err();
    let x : Rect = a.unwrap_or_default();      // Type needs to implement the Default trait.
    assert!(x.w == 0 && x.h == 0 && x.title == "");

    // With Results, we can unwrap the Err if we want. x is of type &str (same as the Err).
    let a = make_err();
    let x = a.unwrap_err();
    assert!(x == "oops");
}

fn demo_logical_or_and_combinators() {
    // This is quite simple, but there is a wrinkle.
    let a = make_result();
    let b = make_err();

    // For 'or', both Results must be of the same type.
    let c = a.or(b);
    assert_eq!(c.unwrap().title, "rec");


    // Reset.
    // But for 'and', they can be different: you get back the second.
    // Hence we need type annotations here.
    let a = make_result();
    let b = make_err();

    let d = a.and(b);
    assert_eq!(d.unwrap_err(), "oops");

    // With different types.
    let a = Ok(5);
    let b = make_err();
    let d = a.and(b);
    assert_eq!(d.unwrap_err(), "oops");
}

fn demo_chaining() {
    println!(">>> demo_chaining");

    // The closure (or function) passed to and_then() only gets called if 'a' is Ok.
    // Since it isn't, we never call the closure. x is of type Rect in this example.
    // The closure must return a Result<T>.
    let a = make_err();
    let result = a.and_then(|x| Ok(x));
    assert!(result.is_err());

    // This can be chained. None of these closures will get called.
    let a = make_err();
    let result = a.and_then(|x| Ok(x)).and_then(|x| Ok(x)).and_then(|x| Ok(x));
    assert!(result.is_err());

    // This will flow the whole way down. Notice that we have to specify mut x for the closure
    // parameter in order to be able to modify the Rect, x.
    let a = make_result();
    let result = a.and_then(|mut x| { x.w += 50; Ok(x) })
        .and_then(|mut x| { x.w += 50; Ok(x) })
        .and_then(|mut x| { x.w += 50; Ok(x) });
    assert_eq!(result.unwrap().w, 160);

    // We can terminate in the middle, but the chain remains safe from panics.
    // Notice that we can even bugger up the types in the last closure.
    let a = make_result();
    let result = a.and_then(|mut x| { x.w += 50; Ok(x) })
        .and_then(|x| Err("oops"))
        .and_then(|x : i32| Ok(x * x));
    assert!(result.is_err());

    // or_else is the other half of the pair.
    let a = make_result();
    let result : Result<Rect, &str> = a.and_then(|mut x| { x.w += 50; Ok(x) })
        .and_then(|x| Err("oops"))
        .or_else(|x| Ok(Rect::new(9,9,"temp")));
    assert_eq!(result.unwrap().title, "temp");

    // and_then - call the closure if the value is Ok
    // or_else  - call the closure if the value is Err

    let a = make_result();
    let result : Result<Rect, &str> = a.or_else(|x| Ok(Rect::new(9,9,"temp")));
    assert_eq!(result.unwrap().title, "rec");

    // and_then can change the type of the Result.
    let a = make_result();
    let result : Result<i32, &str> = a.and_then(|x| Ok(42));
    assert_eq!(result, Ok(42));
}

// fn demo_map() {
//     println!(">>> demo_map");
//     let a = Some("hello".to_string());

//     // Maps an Option<T> to Option<U> by applying a function to a contained value.
//     // * Consumes the original value (see as_ref for a way around this)
//     // * and always applies a new wrapping.
//     let result = a.map(|x| x);

//     // TODO 'Consumes' is NOT misleading in the case of Strings.
//     // This won't compile because a has been moved.
//     // assert_eq!(a, None);
//     // Here is demonstrated the property of automatically wrapping in a Some.
//     // Note that the closure does not have Some.
//     assert_eq!(result, Some("hello".to_string()));

//     // Like and_then(), map can change the type of the Option.
//     let a = Some("hello".to_string());
//     let result = a.map(|x| x.len());
//     // assert_eq!(a, Some("hello".to_string()));     Again, won't compile because a is moved.
//     assert_eq!(result, Some(5));

//     // You can call map() on a None. Nothing happens, the None is propagated and the closure is not called.
//     // TODO: The documentation is really bad here.
//     let a : Option<String> = None;
//     let result = a.map(|x| x.len());
//     assert_eq!(result, None);


//     // The other two map variants basically allow us to deal with the None by
//     // supplying a default or a closure to compute a value.
//     // n.b. These do not return Options, they return values!
//     let a : Option<String> = None;
//     let result = a.map_or("hello".len(), |x| x.len());
//     assert_eq!(result, 5);

//     // Note that the default-calculating closure for map_or_else() takes no arguments.
//     let a : Option<String> = None;
//     let x = "hello".to_string();
//     let result = a.map_or_else(|| x.len(), |y| y.len());
//     assert_eq!(result, 5);
// }
