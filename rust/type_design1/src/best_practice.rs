// This just summarizes what we have in the other classes, omitting
// the explanatory commentary and functions that do not compile.

#[allow(dead_code)]
pub struct BestPractice<'a> {
    a: i32,
    pub b: &'static str,
    c: String,
    d: &'a String,

    e: Option<&'static str>,
    f: Option<String>,
    g: Option<&'a String>
}

impl<'a> BestPractice<'a> {
    pub fn new(s: &'a String) -> BestPractice<'a> {
        BestPractice {
            a: 42,
            b: "A static string",
            c: String::from("An owned string"),
            d: s,
            e: Some("An optional static string"),
            f: Some(String::from("An optional owned string")),
            g: Some(s) // Just reuse this for brevity.
        }
    }

    // Immutable scalar fields (for types that implement Copy, which includes POD types).
    pub fn get_a(&self) -> i32 {
        self.a
    }

    // c cannot be changed by the caller.
    pub fn get_c(&self) -> &String {
        &self.c
    }

    pub fn get_d(&self) -> &String {
        self.d
    }

    pub fn get_e(&self) -> Option<&'static str> {
        self.e
    }

    pub fn get_f(&self) -> &Option<String> {
        &self.f
    }

    pub fn get_g(&self) -> &Option<&String> {
        &self.g
    }
}
