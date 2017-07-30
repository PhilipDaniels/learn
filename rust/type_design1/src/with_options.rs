#[allow(dead_code)]
pub struct WithOptions<'a> {
    // Option<> implements Copy.
    a: Option<&'static str>,
    b: Option<String>,
    c: Option<&'a String>
}

impl<'a> WithOptions<'a> {
    pub fn new(s: &'a String) -> WithOptions<'a> {
        WithOptions {
            a: Some("A static string"),
            b: Some("A heap string".to_owned()),
            c: Some(s)
        }
    }

    pub fn get_a(&self) -> Option<&'static str> {
        self.a
    }

    /* This won't compile: attempt to move out of borrowed content
    pub fn get_b(&self) -> Option<String> {
        self.b
    }
    */

    pub fn get_b(&self) -> &Option<String> {
        &self.b
    }

    pub fn get_c(&self) -> &Option<&String> {
        &self.c
    }
}
