use crate::gc::gc;
use std::collections::HashMap;
use std::thread;

use crate::heap::{Heap, UnsizedValue};
use crate::stack::{SizedValue, Stack};

pub struct Vm {
    pub stack: Stack,
    pub heap: Heap,
}

impl Vm {
    pub fn new() -> Vm {
        Vm {
            stack: Stack::new(),
            heap: Heap::new(),
        }
    }

    fn program(&mut self) {
        let string = UnsizedValue::String("Hello, World!".to_string());
        let mut stack_values = self.stack.values.lock().unwrap();
        let mut heap_values = self.heap.values.lock().unwrap();
        heap_values.push(string);

        stack_values.push(SizedValue::Address(0));
        stack_values.push(SizedValue::Address(1));

        let mut object = UnsizedValue::Object(HashMap::new());
        match &mut object {
            UnsizedValue::Object(map) => {
                map.insert("foo".to_string(), SizedValue::I32(42));
                map.insert("bar".to_string(), SizedValue::I32(1337));
                map.insert("refers_to_string".to_string(), SizedValue::Address(0));
            }
            _ => {}
        }
        heap_values.push(object);

        drop(stack_values);
        drop(heap_values);

        thread::sleep(std::time::Duration::from_secs(5));

        let mut stack_values = self.stack.values.lock().unwrap();
        stack_values.pop();
    }

    pub fn run(&mut self) {
        let gc_handle = thread::spawn({
            let mut stack = self.stack.clone();
            let mut heap = self.heap.clone();
            move || loop {
                gc(&mut stack, &mut heap);
                thread::sleep(std::time::Duration::from_millis(1000));
            }
        });

        self.program();

        gc_handle.join().unwrap();
    }
}
