use std::path::{Path, PathBuf};
use std::ffi::{OsString, OsStr};

pub fn run() {
    println!("********* API Design Examples (String-like parameters) *********");

    // If something is commented out, it won't compile.
    string_examples();
    path_examples();
}


fn string_by_value(s: String) {
}

fn string_by_ref(s: &String) {
}

fn string_slice(s: &str) {
    // This can be called with a String (using &) and a &str.
}

fn string_borrow_or_move<T: AsRef<str>>(s: T) {
    // This can be called with a String (using &) and a &str PLUS a move of a String.
    // But no Path types.

    // By declaring that we accept AsRef<str>, we are saying that we will accept anything
    // that can be converted to &str. Sometimes this conversion is free, sometimes it isn't.
    // The conversion is guaranteed to work (not panic).
    // We have to make that conversion like this (x is of type &str):

    let x = s.as_ref();

    // The original 's' parameter has no useful methods, other than as_ref().
    // So we have to call it, and we have to call it manually, as shown above.
}

fn string_examples() {
    let s = "A String".to_string();
    string_by_value(s);
    // string_by_value(&s);
    // string_by_value("hello");

    let s = "A String".to_string();
    // string_by_ref(s);
    string_by_ref(&s);
    // string_by_ref("hello");

    let s = "A String".to_string();
    //string_slice(s);
    string_slice(&s);
    string_slice("hello");

    let s = "A String".to_string();
    string_borrow_or_move(&s);
    string_borrow_or_move("hello");
    string_borrow_or_move(s);
    //string_borrow_or_move(s);
}


fn path_or_string<T: AsRef<Path>>(s : T) {
    // x is of type &Path.
    let x = s.as_ref();
}

fn path_examples() {
    let s = "A String".to_string();
    let p = PathBuf::from("A PathBuf");
    let p_temp = PathBuf::from("A Path");
    let p2 = p_temp.as_path();

    // None of the string functions will accept a PathBuf.
    //string_by_value(p);
    //string_by_ref(&p);
    //string_slice(p);
    //string_slice(&p);

    //string_borrow_or_move(p);
    //string_borrow_or_move(&p);

    // So we can call this with &str, &String, &Path and &PathBuf (all borrows)
    // or moves of String or PathBuf.
    path_or_string("hello");
    path_or_string(&p);
    path_or_string(p);
    path_or_string(&s);
    path_or_string(s);
    path_or_string(&p2);
    path_or_string(p2);

    // If you look at https://doc.rust-lang.org/std/convert/trait.AsRef.html
    // You can see everything that can be converted to, say, &Path. It includes
    // things like OsStr, OsString and Cow<'a, OsStr>. One more demo.

    let c = OsString::from("heee");
    path_or_string(&c);
}

