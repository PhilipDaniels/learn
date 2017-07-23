// 'mod' means bring in another module (remember that a file is somewhat
// synonymous with a module). However, to make the 'extra' sub-modules
// visible outside of this module we need to use 'pub mod' - which basically
// means 'bind to the module and rexport it publicly'.

mod extra_private;
pub mod extra_public;

pub fn args_function() -> i32 {
    // So I can call this private function from in this module, but not from main.rs
    extra_private::extra_private_function();
    42
}

fn private_function() -> i32 {
    24
}
