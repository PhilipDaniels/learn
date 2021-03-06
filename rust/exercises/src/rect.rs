use std::fmt;

#[derive(Debug, Default, Clone)]
pub struct Rect {
    pub w: i32,
    pub h: i32,
    pub title: String
}

impl Rect {
    pub fn new<T>(w: i32, h: i32, t: T) -> Rect
        where T: AsRef<str> {
        Rect { w: w, h: h, title: t.as_ref().to_string() }
    }

    pub fn demo() -> Rect {
        Rect { w: 10, h: 20, title: String::from("first demo") }
    }

    pub fn demo2() -> Rect {
        Rect { w: 1000, h: 2000, title: String::from("second demo") }
    }

    pub fn app(&mut self, suffix: &str) {
        self.title.push_str(suffix);
    }
}

impl fmt::Display for Rect {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({},{},{})", self.title, self.w, self.h)
    }
}
