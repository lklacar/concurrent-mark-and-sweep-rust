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

            let stack_values = self.stack.values.lock().unwrap();
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
            if i % 3000 == 0 {
                println!("{} instructions", i);

                let mut stack_values = self.stack.values.lock().unwrap();

                stack_values.clear();

                drop(stack_values);
            }

            if i > 1000000 {
                break;
            }

            // every 5000 instructions, run the GC
            // if i % 5000 == 0 {
                // print heap
                // println!("Heap before GC: {:?}", self.heap.values.lock().unwrap());

                gc(&mut self.stack, &mut self.heap);
                // println!("Heap size: {}", self.heap.values.lock().unwrap().len());
            // }

        }
    }

    pub fn run(&mut self) {
        let gc_handle = self.start_gc_thread();

        self.program();

        // gc_handle.join().unwrap();
    }

    fn start_gc_thread(&mut self) -> JoinHandle<()> {
        let gc_handle = thread::spawn({
            let mut stack = self.stack.clone();
            let mut heap = self.heap.clone();
            let duration = self.last_instruction_duration.clone();
            let mut i = 0;
            move || loop {
                let duration_mutex = duration.lock().unwrap();
                let duration = duration_mutex.clone();
                drop(duration_mutex);

                gc(&mut stack, &mut heap);

                let duration = duration.as_nanos() as u64;


                let wait_duration = Duration::from_nanos(duration * 100);
                thread::sleep(wait_duration);
                i += 1;
            }
        });
        gc_handle
    }
}
