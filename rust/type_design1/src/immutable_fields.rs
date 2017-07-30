#[allow(dead_code)]
pub struct ImmutableFields {
    // Read PrivateFields first.

    // 'a' must be private and we must provide a getter, otherwise any client could change it.
    a: i32,
    // 'b' can be public, because static strings are immutable.
    pub b: &'static str,
    // For strings:
    //   pub c: String - it's mutable. Clients that have used 'mut' in their variable bindings
    //          for ImmutableFields can change it.
    //   c: String - it's private, cannot be directly mutated. Need a function to get at it.
    c: String,
}

impl ImmutableFields {
    pub fn new() -> ImmutableFields {
        ImmutableFields {
            a: 42,
            b: "hello",
            c: "an owned string".to_owned()
        }
    }

    pub fn get_a(&self) -> i32 {
        self.a
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


    /* ====================== PEDAGOGICAL EXAMPLES BELOW ====================== */
    /*
    pub fn get_c(&self) -> String {
        // This will give an error 'cannot move out of borrowed content'.
        // We have borrowed self, and we can't move a part of it to somewhere else.
        self.c
    }
    */

    pub fn get_c_clone_inefficient(&self) -> String {
        // This will work, but is inefficient. We are cloning (and hence allocating)
        // a new string everytime we call this function.
        self.c.clone()
    }

    /*
    pub fn get_c(&self) -> &String {
        // This will give an error 'mismatched types'. 'self.c' says 'move a string'
        // but the return type of the function is expecting a string reference.
        self.c
    }
    */

    pub fn get_c_immutable(&mut self) -> &String {
        // This function behaves the same as get_c() as far as the caller is concerned.
        // The 'mut' is irrelevant since we do not mutate c.
        // You would normally never write this.
        &self.c
    }
}
