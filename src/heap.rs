use crate::stack::SizedValue;
use std::collections::HashMap;

use crate::OpCode;

#[derive(Debug, Clone)]
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

#[derive(Debug)]
pub struct Heap {
    pub(crate) values: Vec<UnsizedValue>,
}


impl Heap {
    pub fn new() -> Heap {
        Heap { values: Vec::new() }
    }

    pub fn alloc(&mut self, value: UnsizedValue) -> usize {
        self.values.push(value);
        self.values.len() - 1
    }

    pub fn push(&mut self, value: UnsizedValue) {
        self.values.push(value);
    }

    pub fn pop(&mut self) -> UnsizedValue {
        self.values.pop().unwrap()
    }

    pub fn get(&self, address: usize) -> &UnsizedValue {
        &self.values[address]
    }
}
