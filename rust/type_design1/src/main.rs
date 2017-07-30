mod immutable_fields;
mod private_fields;

use immutable_fields::*;
use private_fields::*;

fn main() {
    /* Static strings are immutable. This won't compile.
    let ss = "A static string";
    ss[0] = 'A';
     */


    let p = PrivateFields::new();
    println!("p.a = {}", p.get_a());
    /* These are private and won't compile.
    p.a = 3;
    println!("p.a = {}", p.a);
    */
    //p.b[2] = 'a';


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
