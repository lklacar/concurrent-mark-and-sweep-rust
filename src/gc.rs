use std::collections::BTreeSet;
use std::thread;
use std::time::Duration;
use crate::heap::{Heap, UnsizedValue};
use crate::stack::{SizedValue, Stack};

pub fn gc(stack: &mut Stack, heap: &mut Heap) {
    let stack_lock = stack.values.lock().unwrap();
    let mut heap_lock = heap.values.lock().unwrap();

    let start = std::time::Instant::now();

    let mut marked: Vec<usize> = Vec::new();

    for value in stack_lock.iter() {
        match value {
            SizedValue::Address(address) => {
                marked.push(*address);
                let heap_object = heap_lock.get(*address).unwrap();

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
                    UnsizedValue::List(list) => {
                        for value in list.iter() {
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

    for (index, value) in heap_lock.iter_mut().enumerate() {
        if !marked.contains(&index) {
            *value = UnsizedValue::Empty;
        }
    }

    // trim heap by removing all Empty values from the end
    let mut empty_values = 0;
    for value in heap_lock.iter().rev() {
        match value {
            UnsizedValue::Empty => {
                empty_values += 1;
            }
            _ => {
                break;
            }
        }
    }
    let len = heap_lock.len();
    heap_lock.truncate(len - empty_values);


    let end = std::time::Instant::now();
}
