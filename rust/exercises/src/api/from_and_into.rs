pub fn run() {
    println!("********* API Design Examples (The Into trait) *********");

    // If something is commented out, it won't compile.

    // Into is the reciprocal of From.
    // Guideline: DO NOT IMPLEMENT INTO!
    // Implement From instead, and you get Into for free.

    // `impl From<T> for TargetType` means a value of type T can be converted to a value of
    // type TargetType.

    // See https://github.com/rust-lang/rust/blob/1.10.0/src/libcore/convert.rs#L239-L243
    // for the definitions of From, Into, AsRef etc.
}


