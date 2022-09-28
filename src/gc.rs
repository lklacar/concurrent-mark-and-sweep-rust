use std::collections::BTreeSet;
use std::ops::Sub;

use crate::heap::{Heap, UnsizedValue};
use crate::stack::{SizedValue, Stack};
use crate::store::Store;

pub fn gc(stack: &mut Stack, heap: &mut Heap, store: &mut Store) {
    let stack_lock = &stack.values;
    let heap_lock = &mut heap.values;
    let variables_lock = &store.values;

    // get only values that are addresses
    let mut variables = Vec::new();
    for value in variables_lock {
        if let SizedValue::Address(_) = value {
            variables.push(value);
        }
    }

    let _start_marking = std::time::Instant::now();

    let mut marked: BTreeSet<usize> = BTreeSet::new();
    for value in stack_lock.iter().chain(variables) {
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
    let _end_marking = std::time::Instant::now();

    let _start_sweeping = std::time::Instant::now();
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
    let _end_sweeping = std::time::Instant::now();

    // print percentage of mark time and sweep time
    // let marking_duration = end_marking.sub(start_marking);
    // let sweeping_duration = end_sweeping.sub(start_sweeping);
    // let total_duration = marking_duration.add(sweeping_duration);
    // let marking_percentage = marking_duration.as_nanos() as f64 / total_duration.as_nanos() as f64 * 100.0;
    // let sweeping_percentage = sweeping_duration.as_nanos() as f64 / total_duration.as_nanos() as f64 * 100.0;
    // println!("Marking took {}% of the time", marking_percentage);
    // println!("Sweeping took {}% of the time", sweeping_percentage);
    // println!();
}
