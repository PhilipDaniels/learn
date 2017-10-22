use std::cell::{RefCell, RefMut};
use std::collections::HashMap;
use std::collections::hash_map::Entry::{Occupied, Vacant};

pub struct Buffer {
    id: i64,
    title: String
}

impl Buffer {
    pub fn get_id(&self) -> i64 {
        self.id
    }

    pub fn title(&self) -> &String {
        &self.title
    }

    pub fn set_title(&mut self, title: String) {
        self.title = title
    }
}

pub struct BufferFactory {
    next_buffer_id: i64
}

impl BufferFactory {
    pub fn new() -> BufferFactory {
        Self { next_buffer_id: -1 }
    }

    pub fn new_empty_buffer(&mut self) -> Buffer {
        self.next_buffer_id += 1;
        Buffer { id: self.next_buffer_id, title: String::default() }
    }
}

pub struct BufferCollection {
    data: HashMap<i64, RefCell<Buffer>>
}

impl BufferCollection {
    pub fn new() -> BufferCollection {
        BufferCollection {
            data: HashMap::with_capacity(20)
        }
    }

    pub fn insert(&mut self, s: Buffer) {
         self.data.insert(s.get_id(), RefCell::from(s));
     }

    pub fn get(&self, buffer_id: i64) -> Option<&RefCell<Buffer>> {
         self.data.get(&buffer_id)
    }
}
