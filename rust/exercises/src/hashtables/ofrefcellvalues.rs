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

pub fn run() {
    println!("********* HashMap<i64, RefCell<Buffer>> examples *********");

    let mut fac = BufferFactory::new();
    let mut buffers = BufferCollection::new();

    let b1 = fac.new_empty_buffer();
    let b2 = fac.new_empty_buffer();

    buffers.insert(b1);
    buffers.insert(b2);

    let buffer_cell1 = buffers.get(0).unwrap();
    let mut bor1 = buffer_cell1.borrow_mut();

    let buffer_cell2 = buffers.get(1).unwrap();
    let mut bor2 = buffer_cell2.borrow_mut();

    bor1.set_title(String::from("phil"));
    bor2.set_title(String::from("daniels"));

    println!("bor1.Id = {}, bor1.title = {}", bor1.get_id(), bor1.title());
    println!("bor2.Id = {}, bor2.title = {}", bor2.get_id(), bor2.title());

    bor1.set_title(String::from("Philip"));
    bor2.set_title(String::from("Daniels"));

    println!("bor1.Id = {}, bor1.title = {}", bor1.get_id(), bor1.title());
    println!("bor2.Id = {}, bor2.title = {}", bor2.get_id(), bor2.title());
}
