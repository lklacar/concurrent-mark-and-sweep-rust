use std::collections::HashMap;

use std::sync::{Arc, Mutex};
use std::thread;

#[derive(Debug, Clone)]
enum SizedValue {
    Int(i32),
    Address(usize),
}

#[derive(Debug, Clone)]
enum UnsizedValue {
    String(String),
    Object(HashMap<String, SizedValue>),
    Empty,
}

fn program(stack: &mut Arc<Mutex<Vec<SizedValue>>>, heap: &mut Arc<Mutex<Vec<UnsizedValue>>>) {
    let string = UnsizedValue::String("Hello, World!".to_string());
    heap.lock().unwrap().push(string);
    // stack.lock().unwrap().push(SizedValue::Address(0));
    stack.lock().unwrap().push(SizedValue::Address(1));

    let mut object = UnsizedValue::Object(HashMap::new());
    match &mut object {
        UnsizedValue::Object(map) => {
            map.insert("foo".to_string(), SizedValue::Int(42));
            map.insert("bar".to_string(), SizedValue::Int(1337));
            map.insert("refers_to_string".to_string(), SizedValue::Address(0));
        }
        _ => {}
    }
    heap.lock().unwrap().push(object);
}

fn gc(stack: &mut Arc<Mutex<Vec<SizedValue>>>, heap: &mut Arc<Mutex<Vec<UnsizedValue>>>) {
    println!("{:?} {:?}", stack.lock().unwrap(), heap.lock().unwrap());

    let mut marked = Vec::new();
    for value in stack.lock().unwrap().iter() {
        match value {
            SizedValue::Address(address) => {
                marked.push(*address);
                let heap = heap.lock().unwrap();
                let heap_object = heap.get(*address).unwrap();

                match heap_object {
                    UnsizedValue::Object(map) => {
                        for (_, value) in map.iter() {
                            match value {
                                SizedValue::Address(address) => {
                                    marked.push(*address);
                                }
                                _ => {}
                            }
                        }
                    }
                    _ => {}
                }
            }
            _ => {}
        }
    }

    let mut heap = heap.lock().unwrap();
    for (index, value) in heap.iter_mut().enumerate() {
        if !marked.contains(&index) {
            *value = UnsizedValue::Empty;
        }
    }
}

fn main() {
    println!(
        "size of UnsizedValue: {}",
        std::mem::size_of::<UnsizedValue>()
    );

    let heap: Arc<Mutex<Vec<UnsizedValue>>> = Arc::new(Mutex::new(Vec::new()));
    let stack: Arc<Mutex<Vec<SizedValue>>> = Arc::new(Mutex::new(Vec::new()));

    let program_handle = thread::spawn({
        let mut stack = stack.clone();
        let mut heap = heap.clone();
        move || {
            program(&mut stack, &mut heap);
        }
    });

    let gc_handle = thread::spawn({
        let mut stack = stack.clone();
        let mut heap = heap.clone();
        move || loop {
            gc(&mut stack, &mut heap);
            thread::sleep(std::time::Duration::from_secs(1));
        }
    });

    thread::spawn({
        let heap = heap.clone();
        move || {
            thread::sleep(std::time::Duration::from_secs(5));
            let mut heap = heap.lock().unwrap();
            let heap_object = heap.get_mut(1).unwrap();

            match heap_object {
                UnsizedValue::Object(map) => {
                    map.remove("refers_to_string");
                }
                _ => {}
            }


            let mut stack_lock = stack.lock().unwrap();
            thread::sleep(std::time::Duration::from_secs(2));
            stack_lock.pop();
            drop(stack_lock);
        }
    });

    program_handle.join().unwrap();
    gc_handle.join().unwrap();
}
