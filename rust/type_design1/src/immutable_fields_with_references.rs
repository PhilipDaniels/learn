#[allow(dead_code)]
pub struct ImmutableFieldsWithReferences<'a> {
    // Read PrivateFields and ImmutableFields first.
    c: String,

    // Now we have a reference to a String that was allocated somewhere else.
    // We need a lifetime to indicate that e must live for at least as long
    // as the struct.
    e: &'a String
}

impl<'a> ImmutableFieldsWithReferences<'a> {
    pub fn new(s: &'a String) -> ImmutableFieldsWithReferences<'a> {
        ImmutableFieldsWithReferences {
            c: "an owned string".to_owned(),
            e: s
        }
    }

    /* ====================== BEST PRACTICE BELOW ====================== */
    pub fn get_c(&self) -> &String {
        // This is typically what we want, similar to returning a string in C#.
        // The caller can get a reference to c with 'let x = i.get_c()' but
        // 'let mut x = i.get_c()' will generate a compilation error:
        // "cannot borrow immutable borrowed content *x as mutable".
        &self.c
    }

    pub fn get_c_mutable(&mut self) -> &mut String {
        // This function allows callers to mutate c.
        // Note that self is now a mutable reference.

        // There is no real equivalent in C#, where strings are immutable. However,
        // it is similar to having a 'public string Foo { get; set; }' property.
        &mut self.c
    }

    pub fn get_e(&self) -> &String {
        // This is in some respects the same as get_c(). The return type of the function
        // is an immutable borrow, so the caller will not be able to change it.
        // They will get the same compilation error.

        // The main difference is that in the body of this function we can just
        // use 'self.e' instead of needing to borrow it.

        // This function can be called multiple times without moving e out of
        // the struct.
        self.e
    }
}
