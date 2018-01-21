use std::rc::Rc;

// Rc  = reference counted, cannot be passed between threads
// Arc = reference counted, can be passed between threads.
//
// Rc is faster, so use it if you can ("you only pay for what you need").

fn main() {
    let s = Rc::new("hello".to_string());
    let t = s.clone();
    let u = s.clone();

    // For an Rc<T>, you can call any method of T.
    if u.contains("el") {
        println!("Yes it does");
    }
}
