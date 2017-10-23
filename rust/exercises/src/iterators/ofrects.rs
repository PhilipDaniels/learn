use std::slice::Iter;
use std::iter::Filter;
use rect::*;

fn make_vec() -> Vec<Rect> {
    vec![
        Rect::new(1, 2, "first"),
        Rect::new(10, 20, "second"),
        Rect::new(3, 5, "third"),
        Rect::new(12, 222, "fourth")
    ]
}

pub fn run() {
    println!("********* Iterator<Rect> examples *********");
    demo_skip_take();
    demo_partition();
    demo_find();
    demo_enumerate();
    demo_chain();
    demo_filter();
    demo_reduction();
    demo_map();
    demo_max();
}

fn demo_skip_take() {
    println!(">>> demo_skip_take");

    let v = make_vec();
    let result : Vec<_> = v.iter().take(2).collect();
    println!("v.take(2) = {:?}", result);

    let v = make_vec();
    let result : Vec<_> = v.iter().skip(3).take(5).collect();
    println!("v.skip(3).take(5) = {:?}", result);

    // Note that when using structs (rather than ints) we don't need to add &&
    // because the compiler can infer what we want automatically.
    let v = make_vec();
    let result : Vec<_> = v.iter().take_while(|x| x.w < 10).collect();
    println!("v.take_while(x.w < 10) = {:?}", result);

    let v = make_vec();
    let result : Vec<_> = v.iter().skip_while(|x| x.w < 10).collect();
    println!("v.skip_while(x.w < 10) = {:?}", result);
}

// Note we are returning Vec<&Rect> rather than Vec<Rect>. This is to avoid copying the Rects,
// which is actually impossible because they do not implement the Copy trait.
fn demo_partition() {
    println!(">>> demo_partition (like filter, but returns both sets of things)");

    let v = make_vec();
    let (a, b) : (Vec<&Rect>, Vec<&Rect>) = v.iter().partition(|x| x.w < 10);
    println!("v.partition(x < 100) (a,b) = ({:?}, {:?})", a, b);
}

fn demo_find() {
    println!(">>> demo_find");

    // find() gives us an Option<T> to the first match, if any.
    let v = make_vec();
    let result = v.iter().find(|x| x.w == 1000);
    println!("v.find(x.w == 1000) = {:?}", result);        // None
    let result = v.iter().find(|x| x.w > 10);
    println!("v.find(x.w > 10) = {:?}", result);         // Some(Rect...)

    // position() and rposition() return an Option<usize> where the usize is the index of the element.
    let v = make_vec();
    let result = v.iter().position(|x| x.w == 987);
    println!("v.position(x.w == 987) = {:?}", result);    // None
    let result = v.iter().position(|x| x.w > 10);
    println!("v.position(x.w > 10) = {:?}", result);     // Some(3)
}

fn demo_enumerate() {
    println!(">>> demo_enumerate");

    // elem is &Rect.
    let v = make_vec();
    for (idx, elem) in v.iter().take(3).enumerate() {
        println!("v[{}] = {}", idx, elem);
    }
}

fn demo_chain() {
    println!(">>> demo_chain");

    let v = make_vec();
    let num_to_skip = v.len() - 1;
    let beginning = v.iter().take(1);
    let end = v.iter().skip(num_to_skip);

    for (idx, elem) in beginning.chain(end).enumerate() {
        println!("(1st chain): v[{}] = {}", idx, elem);
    }

    // Better, only a single pass. len() is O(1) because a Vec is a triple (ptr, capacity, length).
    let v = make_vec();
    let end_index = v.len() - 1;
    for (idx, elem) in v.iter().enumerate().filter(|&(idx, _)| idx < 1 || idx >= end_index) {
        println!("(2nd chain): v[{}] = {}", idx, elem);
    }
}


// /*
// // This is HARD: see https://stackoverflow.com/questions/27535289/what-is-the-correct-way-to-return-an-iterator
// fn return_iterator_from_function() -> Filter<Iter<i32>, fn(&i32) -> bool> {
//     let v = make_vec();
//     v.iter().filter(|&x| x % 2 == 0)
// }
// */

fn demo_filter() {
    println!(">>> demo_filter");

    let v = make_vec();
    let result : Vec<_> = v.iter().filter(|x| x.title.contains("i")).collect();
    println!("v.filter(x.title.contains(\"i\")) = {:?}", result);
}

fn demo_reduction() {
    println!(">>> demo_reduction");

    let v = make_vec();
    let result : i32 = v.iter().map(|x| x.w).sum();
    println!("v.sum(of w) = {:?}", result);

    let v = make_vec();
    let result = v.iter().fold(100000, |mut agg, x| { agg += x.w; agg });
    println!("v.fold(100000 + w) = {:?}", result);
}

fn demo_map() {
    println!(">>> demo_map");

    let v = make_vec();
    let result : Vec<_> = v.iter().map(|x| x.w + 1000).collect();
    println!("v.map (original vector is unchanged) = {:?}", result);

    let mut v = make_vec();
    v.iter_mut().map(|x| x.w += 1000).count();
    println!("v.map (using iter_mut) = {:?}", v);

    // step_by() requires `#![feature(iterator_step_by)]` in main.rs.
    let mut v = make_vec();
    v.iter_mut().step_by(2).map(|x| x.w += 5000).count();
    println!("v.map (using iter_mut and step_by) = {:?}", v);
}

fn demo_max() {
    println!(">>> demo_max");

    let v = make_vec();
    let result = v.iter().max_by_key(|x| x.h);
    println!("v.max_by_key(by x.h) = {:?}", result);

    // max_by is a little harder to use than max_by_key, which will probably do what you
    // want in most circumstances, unless you need to bind F as a variable.
    let result = v.iter().max_by(|a, b| a.title.cmp(&b.title));
    println!("v.max_by = {:?}", result);
}