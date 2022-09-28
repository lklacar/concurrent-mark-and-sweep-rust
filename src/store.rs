use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use crate::stack::SizedValue;

#[derive(Debug, Clone)]
pub struct Store {
    pub values: Vec<SizedValue>,
}


impl Store {
    pub fn new() -> Store {
        let mut vec = Vec::new();
        vec.resize(10, SizedValue::Null);
        Store {
            values: vec
        }
    }

    pub fn set(&mut self, key: usize, value: SizedValue) {
        self.values[key] = value;
    }

    pub fn get(&self, key: usize) -> SizedValue {
        self.values[key].clone()
    }
}
