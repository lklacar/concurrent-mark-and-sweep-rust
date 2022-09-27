use crate::stack::SizedValue;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use crate::OpCode;

#[derive(Debug)]
pub enum UnsizedValue {
    String(String),
    List(Vec<SizedValue>),
    Object(HashMap<String, SizedValue>),
    Function(Vec<OpCode>),
    Empty,
}

impl UnsizedValue {
    pub fn as_function(&self) -> &Vec<OpCode> {
        match self {
            UnsizedValue::Function(f) => f,
            _ => panic!("Expected function, got {:?}", self),
        }
    }

    pub fn as_string(&self) -> &String {
        match self {
            UnsizedValue::String(s) => s,
            _ => panic!("Expected string, got {:?}", self),
        }
    }
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

    pub fn alloc(&self, value: UnsizedValue) -> usize {
        let mut values = self.values.lock().unwrap();
        values.push(value);
        values.len() - 1
    }

    pub fn push(&mut self, value: UnsizedValue) {
        self.values.lock().unwrap().push(value);
    }

    pub fn pop(&mut self) -> UnsizedValue {
        self.values.lock().unwrap().pop().unwrap()
    }
}
