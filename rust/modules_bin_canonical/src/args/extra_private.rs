// By being in a separate file we have created a new module, its
// namescope is args::extra_private.

// This 'pub' marks this as a function that is exported from this module.
// You need to read the 'mod' statements in mod.rs to get the whole picture.
pub fn extra_private_function() {
    println!("extra_private_function is running");
}