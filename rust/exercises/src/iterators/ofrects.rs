use std::slice::Iter;
use std::iter::Filter;

fn make_vec() -> Vec<i32> {
    vec![4, 5, 6, -1, -100, 2, 400, 400, 500, 600, 11, 13, 15, 17, 19, 21, 23, 25]
}

pub fn run() {
    println!("********* Iterator<Rect> examples *********");
    // demo_skip_take();
    // demo_partition();
    // demo_find();
    // demo_enumerate();
    // demo_chain();
    // demo_filter();
    // demo_reduction();
    // demo_map();
}

// fn demo_skip_take() {
//     println!(">>> demo_skip_take");

//     let v = make_vec();
//     let result : Vec<_> = v.iter().take(5).collect();
//     println!("v.take(5) = {:?}", result);

//     let v = make_vec();
//     let result : Vec<_> = v.iter().skip(3).take(5).collect();
//     println!("v.skip(3).take(5) = {:?}", result);

//     let v = make_vec();
//     let result : Vec<_> = v.iter().take_while(|&&x| x < 100).collect();
//     println!("v.take_while(x < 100) = {:?}", result);

//     let v = make_vec();
//     let result : Vec<_> = v.iter().skip_while(|&&x| x < 100).collect();
//     println!("v.skip_while(x < 100) = {:?}", result);
// }

// fn demo_partition() {
//     println!(">>> demo_partition (like filter, but returns both sets of things)");

//     let v = make_vec();
//     let (a, b) : (Vec<i32>, Vec<i32>) = v.iter().partition(|&&x| x < 100);
//     println!("v.partition(x < 100) (a,b) = ({:?}, {:?})", a, b);
// }

// fn demo_find() {
//     println!(">>> demo_find");

//     // find() gives us an Option<T> to the first match, if any.
//     let v = make_vec();
//     let result = v.iter().find(|&&x| x == 987);
//     println!("v.find(x == 987) = {:?}", result);        // None
//     let result = v.iter().find(|&&x| x > 400);
//     println!("v.find(x > 400) = {:?}", result);         // Some(500)

//     // position() and rposition() return an Option<usize> where the usize is the index of the element.
//     let v = make_vec();
//     let result = v.iter().position(|&x| x == 987);
//     println!("v.position(x == 987) = {:?}", result);    // None
//     let result = v.iter().position(|&x| x > 400);
//     println!("v.position(x > 400) = {:?}", result);     // Some(8)

//     assert!(v.iter().nth(result.unwrap()).unwrap() > &400);
// }

// fn demo_enumerate() {
//     println!(">>> demo_enumerate");

//     let v = make_vec();
//     for (idx, elem) in v.iter().take(3).enumerate() {
//         println!("v[{}] = {}", idx, elem);
//     }
// }

// fn demo_chain() {
//     println!(">>> demo_chain");

//     let v = make_vec();
//     let num_to_skip = v.len() - 3;
//     let beginning = v.iter().take(3);
//     let end = v.iter().skip(num_to_skip);

//     for (idx, elem) in beginning.chain(end).enumerate() {
//         println!("v[{}] = {}", idx, elem);
//     }

//     // Better, only a single pass. len() is O(1) because a Vec is a triple (ptr, capacity, length).
//     let v = make_vec();
//     let end_index = v.len() - 3;
//     for (idx, elem) in v.iter().enumerate().filter(|&(idx, _)| idx < 3 || idx >= end_index) {
//         println!("v[{}] = {}", idx, elem);
//     }
// }


// /*
// // This is HARD: see https://stackoverflow.com/questions/27535289/what-is-the-correct-way-to-return-an-iterator
// fn return_iterator_from_function() -> Filter<Iter<i32>, fn(&i32) -> bool> {
//     let v = make_vec();
//     v.iter().filter(|&x| x % 2 == 0)
// }
// */

// fn demo_filter() {
//     println!(">>> demo_filter");

//     let v = make_vec();
//     let result : Vec<_> = v.iter().filter(|&x| x %2 == 0).collect();
//     println!("v.filter(x %2 == 0) = {:?}", result);
// }

// fn demo_reduction() {
//     println!(">>> demo_reduction");

//     let v = make_vec();
//     let result : i32 = v.iter().filter(|&x| x %2 == 0).sum();
//     println!("v.sum = {:?}", result);

//     let v = make_vec();
//     let result = v.iter().fold(100000, |mut agg, &x| { agg += x; agg });
//     println!("v.fold = {:?}", result);
// }

// fn demo_map() {
//     println!(">>> demo_map");

//     let v = make_vec();
//     let result : Vec<_> = v.iter().map(|&x| x + 1000).collect();
//     println!("v.map (original vector is unchanged) = {:?}", result);

//     let mut v = make_vec();
//     v.iter_mut().map(|x| *x = *x + 1000).count();
//     println!("v.map (using iter_mut) = {:?}", v);

//     // step_by() requires `#![feature(iterator_step_by)]` in main.rs.
//     let mut v = make_vec();
//     v.iter_mut().step_by(3).map(|x| *x = *x + 5000).count();
//     println!("v.map (using iter_mut and step_by) = {:?}", v);
// }



