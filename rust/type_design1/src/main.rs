mod best_practice;
mod immutable_fields;
mod immutable_fields_with_references;
mod private_fields;
mod with_options;

use best_practice::*;
use immutable_fields::*;
use immutable_fields_with_references::*;
use private_fields::*;
use with_options::*;

fn main() {
    /* Static strings are immutable. This won't compile.
    let ss = "A static string";
    ss[0] = 'A';
     */

    test_private_fields();
    test_immutable_fields();
    test_immutable_fields_with_references();
    test_with_options();
    test_best_practice();
}

fn test_private_fields() {
    let p = PrivateFields::new();
    println!("p.a = {}", p.get_a());
    /* These are private and won't compile.
    p.a = 3;
    println!("p.a = {}", p.a);
    */
    //p.b[2] = 'a';
}

fn test_immutable_fields() {
    // Making b public is fine, it is a static string and hence immutable.
    let mut i = ImmutableFields::new();
    println!("i.b = {}", i.b);

    // This won't compile without 'pub c' in the ImmutableFields struct.
    // i.c.push_str(" - a change");
    println!("i.c = {}", i.get_c());

    /*
    // This won't compile. Will generate error "cannot borrow immutable borrowed content *x as mutable"
    // even though both i and x bindings are specified as mutable. It is because the return type
    // of the function is
    let mut x = i.get_c();
    x.push_str("A change");
    */

    // We can mutate i. We need to do it in a sub-scope for the subsequent print!
    // to work, otherwise we get warnings about mutable/immutable borrowings simultaneous.
    {
        let mut x = i.get_c_mutable();
        x.push_str(" - a change");
    }

    println!("i.c = {}", i.get_c());
}

fn test_immutable_fields_with_references() {
    let s = String::from("The string");
    let mut i = ImmutableFieldsWithReferences::new(&s);
    let e = i.get_e();
    let e2 = i.get_e();

    println!("i.e = {}", i.get_e());
}

fn test_with_options() {
    let s = String::from("The string");

    let wo = WithOptions::new(&s);
    let a = wo.get_a();
    let b = wo.get_b();
    println!("wo.a = {:?}, wo.b = {:?}, wo.c = {:?}", wo.get_a(), wo.get_b(), wo.get_c());
}

fn test_best_practice() {
    let s = String::from("A string from the surrounding scope");

    let bp = BestPractice::new(&s);
    println!("bp.a = {:?}, bp.b = {:?}, bp.c = {:?}, bp.d = {:?}", bp.get_a(), bp.b, bp.get_c(), bp.get_d());
}