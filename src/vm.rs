use crate::gc::gc;
use std::collections::HashMap;

use std::thread;
use std::thread::{JoinHandle, Thread};

use crate::heap::{Heap, UnsizedValue};
use crate::{OpCode, PushI64};
use crate::stack::{SizedValue, Stack};
use crate::store::Store;

pub struct Vm {
    pub stack: Stack,
    pub heap: Heap,
    store: Store,
}


macro_rules! heap_load {
    ($self:ident, $address:ident, $name:ident) => {
        let heap_lock = $self.heap.values.lock().unwrap();
        let $name = heap_lock.get($address.as_address().clone()).unwrap();
    }
}



impl Vm {
    pub fn new() -> Vm {
        Vm {
            stack: Stack::new(),
            heap: Heap::new(),
            store: Store::new(),
        }
    }

    fn execute(&mut self, program: &UnsizedValue, gc_thread: &Thread) {
        let mut i = 0;
        let program = program.as_function();
        self.store.push();
        let mut counter = 0;
        while i < program.len() {
            counter += 1;
            if counter % 100 == 0 {
                let mut stack = self.stack.clone();
                let mut heap = self.heap.clone();
                let mut store = self.store.clone();
                gc(&mut stack, &mut heap, &mut store);
            }

            let instruction = &program[i];

            match instruction {
                OpCode::Add => {
                    let a = self.stack.pop();
                    let b = self.stack.pop();
                    self.stack.push(a + b);
                }
                OpCode::Sub => {
                    let a = self.stack.pop();
                    let b = self.stack.pop();
                    self.stack.push(a - b);
                }
                OpCode::Mul => {
                    let a = self.stack.pop();
                    let b = self.stack.pop();
                    self.stack.push(a * b);
                }
                OpCode::Div => {
                    let a = self.stack.pop();
                    let b = self.stack.pop();
                    self.stack.push(b / a);
                }
                OpCode::Mod => {
                    let a = self.stack.pop();
                    let b = self.stack.pop();
                    self.stack.push(b % a);
                }
                OpCode::Neg => {
                    let a = self.stack.pop();
                    self.stack.push(-a);
                }
                OpCode::Not => {
                    let a = self.stack.pop();
                    self.stack.push(!a);
                }
                OpCode::And => {
                    let a = self.stack.pop();
                    let b = self.stack.pop();
                    self.stack.push(a & b);
                }
                OpCode::Or => {
                    let a = self.stack.pop();
                    let b = self.stack.pop();
                    self.stack.push(a | b);
                }
                OpCode::Eq => {
                    let a = self.stack.pop();
                    let b = self.stack.pop();
                    self.stack.push(SizedValue::Bool(a == b));
                }
                OpCode::Neq => {
                    let a = self.stack.pop();
                    let b = self.stack.pop();
                    self.stack.push(SizedValue::Bool(a != b));
                }
                OpCode::Lt => {
                    let a = self.stack.pop();
                    let b = self.stack.pop();
                    self.stack.push(SizedValue::Bool(b < a));
                }
                OpCode::Gt => {
                    let a = self.stack.pop();
                    let b = self.stack.pop();
                    self.stack.push(SizedValue::Bool(b > a));
                }
                OpCode::Lte => {
                    let a = self.stack.pop();
                    let b = self.stack.pop();
                    self.stack.push(SizedValue::Bool(b <= a));
                }
                OpCode::Gte => {
                    let a = self.stack.pop();
                    let b = self.stack.pop();
                    self.stack.push(SizedValue::Bool(b >= a));
                }
                PushI64(value) => {
                    let value = SizedValue::I64(*value);
                    self.stack.push(value);
                }
                OpCode::PushF32(value) => {
                    let value = SizedValue::F32(*value);
                    self.stack.push(value);
                }
                OpCode::PushString(value) => {
                    let value = UnsizedValue::String(value.clone());
                    let value = self.heap.alloc(value);
                    self.stack.push(SizedValue::Address(value));
                }
                OpCode::PushBool(value) => {
                    let value = SizedValue::Bool(*value);
                    self.stack.push(value);
                }
                OpCode::PushObject => {
                    let value = UnsizedValue::Object(HashMap::new());
                    let value = self.heap.alloc(value);
                    self.stack.push(SizedValue::Address(value));
                }
                OpCode::PushList => {
                    let value = UnsizedValue::List(Vec::new());
                    let value = self.heap.alloc(value);
                    self.stack.push(SizedValue::Address(value));
                }
                OpCode::Function(instructions) => {
                    let value = UnsizedValue::Function(instructions.clone());
                    let value = self.heap.alloc(value);
                    self.stack.push(SizedValue::Address(value));
                }
                OpCode::Store => {
                    let name_address = self.stack.pop();
                    heap_load!(self, name_address, name);
                    let value = self.stack.pop();
                    self.store.set(name.as_string().clone(), value);
                }
                OpCode::Load => {
                    let name_address = self.stack.pop();
                    heap_load!(self, name_address, name);
                    let value = self.store.get(name.as_string().clone());
                    self.stack.push(value);
                }
                OpCode::Call => {}
                OpCode::Return => {}
                OpCode::Jump(offset) => {
                    i = ((i as i64) + offset) as usize;
                }
                OpCode::JumpIfFalse(offset) => {
                    let value = self.stack.pop();
                    if !value.as_bool() {
                        i = ((i as i64) + offset) as usize;
                    }
                }
            }


            i += 1;
        }

        // println!("Stack: {:?}", self.stack);
        // println!("Heap: {:?}", self.heap);
        println!("Variable Store: {:?}", self.store);
    }


    pub fn run(&mut self, program: &UnsizedValue) {
        let gc_handle = self.start_gc_thread();
        let gc_thread = gc_handle.thread();

        self.execute(program, gc_thread);
    }

    fn start_gc_thread(&self) -> JoinHandle<()> {
        let gc_handle = thread::spawn({
            let mut stack = self.stack.clone();
            let mut heap = self.heap.clone();
            let mut store = self.store.clone();
            move || loop {
                thread::park();
                let start = std::time::Instant::now();
                gc(&mut stack, &mut heap, &mut store);
                let end = std::time::Instant::now();
                let duration = end.duration_since(start);
                println!("GC took {:?}", duration);
            }
        });
        gc_handle
    }
}
