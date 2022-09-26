use crate::heap::{Heap, UnsizedValue};
use crate::stack::{SizedValue, Stack};

pub fn gc(stack: &mut Stack, heap: &mut Heap) {
    let stack = stack.values.lock().unwrap();
    let mut heap = heap.values.lock().unwrap();

    let mut marked = Vec::new();
    for value in stack.iter() {
        match value {
            SizedValue::Address(address) => {
                marked.push(*address);
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

    for (index, value) in heap.iter_mut().enumerate() {
        if !marked.contains(&index) {
            *value = UnsizedValue::Empty;
        }
    }
}
