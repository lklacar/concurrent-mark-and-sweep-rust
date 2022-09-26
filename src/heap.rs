use crate::stack::SizedValue;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

#[derive(Debug)]
pub enum UnsizedValue {
    String(String),
    List(Vec<SizedValue>),
    Object(HashMap<String, SizedValue>),
    Empty,
}

#[derive(Debug, Clone)]
pub struct Heap {
    pub(crate) values: Arc<Mutex<Vec<UnsizedValue>>>,
}

impl Heap {
    pub fn new() -> Heap {
        Heap {
            values: Arc::new(Mutex::new(Vec::new())),
        }
    }

    pub fn push(&mut self, value: UnsizedValue) {
        self.values.lock().unwrap().push(value);
    }

    pub fn pop(&mut self) -> UnsizedValue {
        self.values.lock().unwrap().pop().unwrap()
    }
}
