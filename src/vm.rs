use crate::gc::gc;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::thread;
use std::thread::JoinHandle;
use std::time::Duration;

use crate::heap::{Heap, UnsizedValue};
use crate::stack::{SizedValue, Stack};

pub struct Vm {
    pub stack: Stack,
    pub heap: Heap,
    last_instruction_duration: Arc<Mutex<Duration>>,
}

impl Vm {
    pub fn new() -> Vm {
        Vm {
            stack: Stack::new(),
            heap: Heap::new(),
            last_instruction_duration: Arc::new(Mutex::new(Duration::new(0, 0))),
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

            // thread::sleep(std::time::Duration::from_secs(5));

            let mut stack_values = self.stack.values.lock().unwrap();
            let mut heap_values = self.heap.values.lock().unwrap();

            // println!("Removing reference from object to string");

            let object = heap_values.get_mut(1).unwrap();
            match object {
                UnsizedValue::Object(map) => {
                    map.remove("refers_to_string");
                }
                _ => {}
            }
            drop(stack_values);
            drop(heap_values);

            let end = std::time::Instant::now();
            let duration = end.duration_since(start);
            // println!("Last instruction took {:?}ms", duration);

            let mut last_duration = self.last_instruction_duration.lock().unwrap();
            *last_duration = duration;

            i += 1;
            if i % 1000 == 0 {
                println!("{} instructions", i);

                let mut stack_values = self.stack.values.lock().unwrap();
                stack_values.clear();
                drop(stack_values);
            }

            if i > 100000 {
                break;
            }
        }
    }

    pub fn run(&mut self) {
        let gc_handle = self.start_gc_thread();

        self.program();

        gc_handle.join().unwrap();
    }

    fn start_gc_thread(&mut self) -> JoinHandle<()> {
        let gc_handle = thread::spawn({
            let mut stack = self.stack.clone();
            let mut heap = self.heap.clone();
            let duration = self.last_instruction_duration.clone();
            let mut i = 0;
            move || loop {
                gc(&mut stack, &mut heap);

                // let duration_mutex = duration.lock().unwrap();
                // let duration = duration_mutex.clone().as_nanos() as u64;
                // drop(duration_mutex);
                thread::sleep(Duration::from_nanos(100));
                i += 1;
                if i % 1 == 0 {
                    // println!("GC run {}", i);
                }
            }
        });
        gc_handle
    }
}
