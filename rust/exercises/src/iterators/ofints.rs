use std::slice::Iter;

fn make_vec() -> Vec<i32> {
    vec![4, 5, 6, -1, -100, 2, 400, 400, 500, 600, 11, 13, 15, 17, 19, 21, 23, 25]
}

pub fn run() {
    println!("********* Iterator<i32> examples *********");
    demo_skip_take();
    demo_partition();
    demo_find();
    demo_enumerate();
    demo_chain();
}

fn demo_skip_take() {
    println!(">>> demo_skip_take");

    let v = make_vec();
    let result : Vec<_> = v.iter().take(5).collect();
    println!("v.take(5) = {:?}", result);

    let v = make_vec();
    let result : Vec<_> = v.iter().skip(3).take(5).collect();
    println!("v.skip(3).take(5) = {:?}", result);

    let v = make_vec();
    let result : Vec<_> = v.iter().take_while(|&&x| x < 100).collect();
    println!("v.take_while(x < 100) = {:?}", result);

    let v = make_vec();
    let result : Vec<_> = v.iter().skip_while(|&&x| x < 100).collect();
    println!("v.skip_while(x < 100) = {:?}", result);
}

fn demo_partition() {
    println!(">>> demo_partition (like filter, but returns both sets of things)");

    let v = make_vec();
    let (a, b) : (Vec<i32>, Vec<i32>) = v.iter().partition(|&&x| x < 100);
    println!("v.partition(x < 100) (a,b) = ({:?}, {:?})", a, b);
}

fn demo_find() {
    println!(">>> demo_find");

    // find() gives us an Option<T> to the first match, if any.
    let v = make_vec();
    let result = v.iter().find(|&&x| x == 987);
    println!("v.find(x == 987) = {:?}", result);        // None
    let result = v.iter().find(|&&x| x > 400);
    println!("v.find(x > 400) = {:?}", result);         // Some(500)

    // position() and rposition() return an Option<usize> where the usize is the index of the element.
    let v = make_vec();
    let result = v.iter().position(|&x| x == 987);
    println!("v.position(x == 987) = {:?}", result);    // None
    let result = v.iter().position(|&x| x > 400);
    println!("v.position(x > 400) = {:?}", result);     // Some(8)

    assert!(v.iter().nth(result.unwrap()).unwrap() > &400);
}

fn demo_enumerate() {
    println!(">>> demo_enumerate");

    let v = make_vec();
    for (idx, elem) in v.iter().take(3).enumerate() {
        println!("v[{}] = {}", idx, elem);
    }
}

fn demo_chain() {
    println!(">>> demo_chain");

    let v = make_vec();
    let num_to_skip = v.len() - 3;
    let beginning = v.iter().take(3);
    let end = v.iter().skip(num_to_skip);

    for (idx, elem) in beginning.chain(end).enumerate() {
        println!("v[{}] = {}", idx, elem);
    }

    // Better, only a single pass. len() is O(1) because a Vec is a triple (ptr, capacity, length).
    let v = make_vec();
    let end_index = v.len() - 3;
    for (idx, elem) in v.iter().enumerate().filter(|&(idx, _)| idx < 3 || idx >= end_index) {
        println!("v[{}] = {}", idx, elem);
    }
}


// Returning an iterator from a function. e.g. make an iterator over the first 3 and last 3 elements.

