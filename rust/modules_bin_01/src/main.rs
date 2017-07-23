// Tell the compiler to look for the module args.
// It will find it in the file called args.rs in the same folder
// as this main.rs file.
mod args;
mod foo;

// Bring a symbol into scope.
use foo::foo_function;

fn main() {
    println!("Hello, world! {}, {}", args::args_function(), foo_function());
}
