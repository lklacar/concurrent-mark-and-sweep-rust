use crate::gc::gc;
use std::collections::HashMap;
use std::hash::Hash;

use std::thread;
use std::thread::{JoinHandle, Thread};

use crate::heap::{Heap, UnsizedValue};
use crate::{OpCode, PushI32};
use crate::stack::{SizedValue, Stack};

pub struct Vm {
    pub stack: Stack,
    pub heap: Heap,
    variable_store: Vec<HashMap<String, SizedValue>>,
    program: UnsizedValue,
}

impl Vm {
    pub fn new(program: UnsizedValue) -> Vm {
        Vm {
            stack: Stack::new(),
            heap: Heap::new(),
            variable_store: vec![HashMap::new()],
            program,
        }
    }

    fn program(&mut self, gc_thread: &Thread) {
        let mut i = 0;
        let program = self.program.as_function();

        while i < program.len() {
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
                PushI32(value) => {
                    let value = SizedValue::I32(*value);
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
                    let heap_lock = self.heap.values.lock().unwrap();
                    let name = heap_lock.get(name_address.as_address().clone()).unwrap().as_string();

                    let value = self.stack.pop();
                    let variable_store = self.variable_store.last_mut().unwrap();
                    variable_store.insert(name.clone(), value);
                }
                OpCode::Load => {
                    let name_address = self.stack.pop();
                    let heap_lock = self.heap.values.lock().unwrap();
                    let name = heap_lock.get(name_address.as_address().clone()).unwrap().as_string();

                    let variable_store = self.variable_store.last().unwrap();
                    let value = variable_store.get(name).unwrap();
                    self.stack.push(value.clone());
                }
                OpCode::Call => {
                    let function = self.stack.pop();
                    let heap_lock = self.heap.values.lock().unwrap();
                    let function = heap_lock.get(function.as_address().clone()).unwrap().as_function();
                    println!("function: {:?}", function);
                }
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

        println!("Stack: {:?}", self.stack);
        println!("Heap: {:?}", self.heap);
        println!("Variable Store: {:?}", self.variable_store);
    }

    pub fn run(&mut self) {
        let gc_handle = self.start_gc_thread();
        let gc_thread = gc_handle.thread();

        self.program(gc_thread);
    }

    fn start_gc_thread(&mut self) -> JoinHandle<()> {
        let gc_handle = thread::spawn({
            let mut stack = self.stack.clone();
            let mut heap = self.heap.clone();
            move || loop {
                let start = std::time::Instant::now();
                // gc(&mut stack, &mut heap);
                let end = std::time::Instant::now();
                let duration = end.duration_since(start);
                println!("GC took {:?}", duration);
                thread::park();
            }
        });
        gc_handle
    }
}
