// This is the canonical way to organize modules, as it allows
// an individual module to be split into multiple files.

// Tell the compiler to look for the module args.
// It will find it in the file called args\mod.rs.
mod args;

fn main() {
    // This will not compile because we used 'mod' and not 'pub mod'
    // when we imported extra_private in args\mod.rs.
    // args::extra_private::extra_private_function();

    args::extra_public::extra_public_function();

    println!("Hello, world! {}", args::args_function());
}
