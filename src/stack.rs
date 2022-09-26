use std::sync::{Arc, Mutex};

#[derive(Debug, Clone)]
pub enum SizedValue {
    I32(i32),
    Address(usize),
}

#[derive(Debug, Clone)]
pub struct Stack {
    pub(crate) values: Arc<Mutex<Vec<SizedValue>>>,
}

impl Stack {
    pub fn new() -> Stack {
        Stack {
            values: Arc::new(Mutex::new(Vec::new())),
        }
    }

    pub fn push(&mut self, value: SizedValue) {
        self.values.lock().unwrap().push(value);
    }

    pub fn pop(&mut self) -> SizedValue {
        self.values.lock().unwrap().pop().unwrap()
    }
}
