use std::collections::BTreeSet;
use std::ops::Sub;

use crate::heap::{Heap, UnsizedValue};
use crate::stack::{SizedValue, Stack};

pub fn gc(stack: &mut Stack, heap: &mut Heap) {
    let stack_lock = stack.values.lock().unwrap();
    let mut heap_lock = heap.values.lock().unwrap();

    let mut marked: BTreeSet<usize> = BTreeSet::new();

    for value in stack_lock.iter() {
        match value {
            SizedValue::Address(address) => {
                marked.insert(*address);
                let heap_object = heap_lock.get(*address).unwrap();

                match heap_object {
                    UnsizedValue::Object(map) => {
                        for (_, value) in map.iter() {
                            match value {
                                SizedValue::Address(address) => {
                                    marked.insert(*address);
                                }
                                _ => {}
                            }
                        }
                    }
                    UnsizedValue::List(list) => {
                        for value in list.iter() {
                            match value {
                                SizedValue::Address(address) => {
                                    marked.insert(*address);
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

    let heap_size = heap_lock.len();
    let all: BTreeSet<usize> = (0..heap_size).collect();
    let unmarked = all.sub(&marked);

    for index in unmarked.iter() {
        heap_lock[*index] = UnsizedValue::Empty;
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
}
