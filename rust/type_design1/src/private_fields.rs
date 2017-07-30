#[allow(dead_code)]
pub struct PrivateFields {
    // All these fields are private (since no pub), however private means "private to this module",
    // not "private to this type". If we moved this struct into main.rs the main() function would
    // be able to access a, b and c because everything would be in the same module (main).
    // By placing things in this file, we have moved the struct into a new module called 'private_fields',
    // and that has the side effect of hiding a, b and c from the code in main().

    // They are also all OWNED by this struct. The lack of a reference is the sign that this struct
    // owns them. The exception to this rule is b, but that is a static string and it is idiomatic,
    // and means that we have a reference to a string that is compiled statically into the exe (like in C).
    // In that case, the string will live as long as the program, so we do not need to add a lifetime
    // specifier such as 'a to PrivateFields - we can just use the built-in 'static lifetime specifier.
    a: i32,
    b: &'static str,
    c: String
}

impl PrivateFields {
    pub fn new() -> PrivateFields {
        PrivateFields {
            a: 42,
            b: "hello",
            c: "an owned string".to_owned()
        }
    }

    pub fn get_a(&self) -> i32 {
        self.a
    }
}
