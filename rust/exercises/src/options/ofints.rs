pub fn run() {
    println!("********* Option<i32> examples *********");

    demo_basic_matching();
    demo_as_ref();
    demo_as_mut();
    demo_unwrap();
    demo_take();
    demo_logical_or_and_combinators();
    demo_and_then();
    demo_map();
}

fn demo_basic_matching() {
    println!(">>> demo_basic_matching");

    // Options are in the prelude, so a is automatically deduced as being of type Option<i32>.
    let a = Some(5);
    assert!(a.is_some());
    assert!(!a.is_none());

    // Match is the most basic way of getting at an option.
    match a {
        Some(x) => println!("MATCH: x (from a) = {}. x is of type i32.", x),
        None => panic!("a should not be None")
    }

    // If let is equivalent to a match where we do nothing in the None.
    if let Some(x) = a {
        println!("IF LET: x (from a) = {}. x is of type i32.", x);
    }
}

fn demo_as_ref() {
    println!(">>> demo_as_ref");
    let a = Some(5);

    // b is of type Option<&i32>.
    // Now when we pattern match, we get an &i32 by default.
    let b = a.as_ref();
    if let Some(x) = b {
        println!("IF LET: x (from b) = {}. x is of type &i32.", x);
        //*x = 22;   Won't compile, *x is immutable.
    }

    // But we can "smash" the reference in the pattern too.
    if let Some(&x) = b {
        println!("IF LET: x (from b) = {}. x is of type i32 when & is used in the pattern.", x);
        //x = 22;   // Won't compile, x is immutable.
    }
}

fn demo_as_mut() {
    println!(">>> demo_as_mut");
    let mut a = Some(5);

    {
        // Again, but using as_mut();
        // c is of type Option<&mut i32>.
        // Now when we pattern match, we get an &i32 by default.
        let c = a.as_mut();
        if let Some(x) = c {
            println!("IF LET: x (from c) = {}. x is of type &mut i32 and can be changed.", x);
            *x = 22;    // Now this will compile.
        }
    }

    {
        // But we can "smash" the reference in the pattern too.
        let c = a.as_mut();
        if let Some(&mut x) = c {
            println!("IF LET: x (from c) = {}. x is of type i32 when &mut is used in the pattern.", x);
            // *x = 22;   // Won't compile, x is just plain old i32 because of the enhanced pattern.
        }
    }
}

fn demo_unwrap() {
    println!(">>> demo_unwrap");
    let a = Some(5);

    let x = a.unwrap();
    println!("x = {}", x);

    // These are all no-ops because is a Some.
    let x = a.unwrap_or(222);
    assert!(x == 5);
    let x = a.unwrap_or_default();
    assert!(x == 5);
    let x = a.unwrap_or_else(|| 222);
    assert!(x == 5);

    // Let's make a None.
    let a : Option<i32> = None;
    let x = a.unwrap_or(222);
    assert!(x == 222);
    let x = a.unwrap_or_default();      // Type needs to implement the Default trait.
    assert!(x == 0);
    let x = a.unwrap_or_else(|| 222);   // Result of a closure.
    assert!(x == 222);
}

fn demo_take() {
    // Take can be used to move a value from one option to another.
    // TODO: Not sure what the point is.
    let mut a = Some(5);
    let b = a.take();

    assert!(a == None);
    assert!(b == Some(5));
}
