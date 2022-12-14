use crate::gc::gc;
use std::collections::HashMap;
use std::sync::atomic::AtomicU64;
use std::sync::Arc;
use std::thread;
use std::thread::JoinHandle;
use std::time::Duration;

use crate::heap::{Heap, UnsizedValue};
use crate::stack::{SizedValue, Stack};

pub struct Vm {
    pub stack: Stack,
    pub heap: Heap,
    last_instruction_duration: Arc<AtomicU64>,
}

impl Vm {
    pub fn new() -> Vm {
        Vm {
            stack: Stack::new(),
            heap: Heap::new(),
            last_instruction_duration: Arc::new(AtomicU64::new(0)),
        }
    }

    fn program(&mut self) {
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

            drop(stack_values);
            drop(heap_values);

            let stack_values = self.stack.values.lock().unwrap();
            let mut heap_values = self.heap.values.lock().unwrap();

            let object = heap_values.get_mut(1).unwrap();
            match object {
                UnsizedValue::Object(map) => {
                    map.remove("refers_to_string");
                }
                _ => {}
            }
            drop(stack_values);
            drop(heap_values);

            i += 1;
            if i % 20000 == 0 {
                let mut stack_values = self.stack.values.lock().unwrap();

                stack_values.clear();

                drop(stack_values);
            }

            let end = std::time::Instant::now();
            let duration = end.duration_since(start);

            self.last_instruction_duration.store(
                duration.as_nanos() as u64,
                std::sync::atomic::Ordering::Relaxed,
            );
        }
    }

    pub fn run(&mut self) {
        let _gc_handle = self.start_gc_thread();
        self.program();
    }

    fn start_gc_thread(&mut self) -> JoinHandle<()> {
        let gc_handle = thread::spawn({
            let mut stack = self.stack.clone();
            let mut heap = self.heap.clone();
            let last_instruction_duration = self.last_instruction_duration.clone();
            move || loop {
                gc(&mut stack, &mut heap);
                let wait_duration =
                    last_instruction_duration.load(std::sync::atomic::Ordering::Relaxed);
                thread::sleep(Duration::from_nanos(wait_duration * 100));
            }
        });
        gc_handle
    }
}
