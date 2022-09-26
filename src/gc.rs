use std::collections::BTreeSet;
use crate::heap::{Heap, UnsizedValue};
use crate::stack::{SizedValue, Stack};

pub fn gc(stack: &mut Stack, heap: &mut Heap) {
    let start = std::time::Instant::now();

    let mut marked: BTreeSet<usize> = BTreeSet::new();

    for value in stack.values.lock().unwrap().iter() {
        match value {
            SizedValue::Address(address) => {
                marked.insert(*address);
                let heap_values = heap.values.lock().unwrap();
                let heap_object = heap_values.get(*address).unwrap();

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

    for (index, value) in heap.values.lock().unwrap().iter_mut().enumerate() {
        if !marked.contains(&index) {
            *value = UnsizedValue::Empty;
        }
    }

    let end = std::time::Instant::now();
    // println!("GC took {}ms", end.duration_since(start).as_millis());
}
