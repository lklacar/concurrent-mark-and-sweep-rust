use crate::gc::gc;
use std::collections::HashMap;

use std::thread;
use std::thread::{JoinHandle, Thread};

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

    fn program(&mut self, gc_thread: &Thread) {
        let mut i = 0;

        loop {
            let start = std::time::Instant::now();

            let string = UnsizedValue::String("Hello, World!".to_string());
            let mut stack_values = self.stack.values.lock().unwrap();
            let mut heap_values = self.heap.values.lock().unwrap();
            heap_values.push(string);

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

            let object = heap_values.get_mut(1).unwrap();
            match object {
                UnsizedValue::Object(map) => {
                    map.remove("refers_to_string");
                }
                _ => {}
            }

            i += 1;
            if i % 20000 == 0 {
                println!("{} instructions", i);
                stack_values.clear();
            }

            let end = std::time::Instant::now();
            let _duration = end.duration_since(start);

            if i > 1000000 {
                break;
            }

            if i % 10000 == 0 {
                gc_thread.unpark();
            }
        }
    }

    pub fn run(&mut self) {
        let gc_handle = self.start_gc_thread();
        let gc_thread = gc_handle.thread();

        self.program(gc_thread);
    }

    fn start_gc_thread(&mut self) -> JoinHandle<()> {
        let gc_handle = thread::spawn({
            let mut stack = self.stack.clone();
            let mut heap = self.heap.clone();
            move || loop {
                gc(&mut stack, &mut heap);
                thread::park();
            }
        });
        gc_handle
    }
}
