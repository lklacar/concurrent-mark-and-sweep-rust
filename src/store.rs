use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use crate::stack::SizedValue;

#[derive(Debug, Clone)]
pub struct Store {
    pub values: Arc<Mutex<Vec<HashMap<String, SizedValue>>>>,
}


impl Store {
    pub fn new() -> Store {
        Store {
            values: Arc::new(Mutex::new(Vec::new())),
        }
    }

    pub fn set(&self, key: String, value: SizedValue) {
        let mut values = self.values.lock().unwrap();
        values.last_mut().unwrap().insert(key, value);
    }

    pub fn get(&self, key: String) -> SizedValue {
        let values = self.values.lock().unwrap();
        values.last().unwrap().get(&key).unwrap().clone()
    }

    pub fn push(&mut self) {
        self.values.lock().unwrap().push(HashMap::new());
    }

    pub fn pop(&mut self) {
        self.values.lock().unwrap().pop();
    }
}
