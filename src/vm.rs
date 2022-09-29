use std::collections::HashMap;

use std::thread;
use std::thread::JoinHandle;

use crate::heap::{Heap, UnsizedValue};
use crate::stack::{SizedValue, Stack};
use crate::store::Store;
use crate::{OpCode};

pub struct Vm {
    pub stack: Stack,
    pub heap: Heap,
    store: Store,
    stdlib: HashMap<String, SizedValue>,
}


fn print(vm: &mut Vm) {
    let value = vm.stack.pop();
    match value {
        SizedValue::Address(s) => {
            let value = vm.heap.get(s);
            println!("{}", value);
        }
        _ => println!("{}", value),
    }
}



impl Vm {
    pub fn new() -> Vm {
        Vm {
            stack: Stack::new(),
            heap: Heap::new(),
            store: {
                let mut store = Store::new();
                store.set(9, SizedValue::Address(0));
                store
            },
            stdlib: {
                let mut map = HashMap::new();
                map.insert("print".to_string(), SizedValue::FunctionPtr(print));
                map
            },
        }
    }

    pub fn execute(&mut self, program: &UnsizedValue) {
        let mut i = 0;
        let program = program.as_function();
        let mut counter = 0;
        while i < program.len() {
            counter += 1;
            if counter % 1000 == 0 {
                // gc(&mut self.stack, &mut self.heap, &mut self.store);
            }

            let instruction = &program[i];

            match instruction {
                OpCode::Add => {
                    let b = self.stack.pop();
                    let a = self.stack.pop();
                    self.stack.push(a + b);
                }
                OpCode::Sub => {
                    let b = self.stack.pop();
                    let a = self.stack.pop();
                    self.stack.push(a - b);
                }
                OpCode::Mul => {
                    let b = self.stack.pop();
                    let a = self.stack.pop();
                    self.stack.push(a * b);
                }
                OpCode::Div => {
                    let b = self.stack.pop();
                    let a = self.stack.pop();
                    self.stack.push(b / a);
                }
                OpCode::Mod => {
                    let b = self.stack.pop();
                    let a = self.stack.pop();
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
                    let b = self.stack.pop();
                    let a = self.stack.pop();
                    self.stack.push(a & b);
                }
                OpCode::Or => {
                    let b = self.stack.pop();
                    let a = self.stack.pop();
                    self.stack.push(a | b);
                }
                OpCode::Eq => {
                    let b = self.stack.pop();
                    let a = self.stack.pop();
                    self.stack.push(SizedValue::Bool(a == b));
                }
                OpCode::Neq => {
                    let b = self.stack.pop();
                    let a = self.stack.pop();
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
                OpCode::PushI64(value) => {
                    let value = SizedValue::I64(*value);
                    self.stack.push(value);
                }
                OpCode::PushF64(value) => {
                    let value = SizedValue::F64(*value);
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
                OpCode::PushList(size) => {
                    let mut list = Vec::with_capacity(*size as usize);
                    for _ in 0..*size {
                        let value = self.stack.pop();
                        list.push(value);
                    }
                    let value = UnsizedValue::new_list((list));
                    let value = self.heap.alloc(value);
                    self.stack.push(SizedValue::Address(value));
                }
                OpCode::Function(instructions) => {
                    let value = UnsizedValue::Function(instructions.clone());
                    let value = self.heap.alloc(value);
                    self.stack.push(SizedValue::Address(value));
                }
                OpCode::Store(address) => {
                    let value = self.stack.pop();
                    self.store.set(address.clone(), value);
                }
                OpCode::Load(address) => {
                    let value = self.store.get(address.clone());
                    self.stack.push(value);
                }
                OpCode::Call => {
                    let address = self.stack.pop();

                    match address {
                        SizedValue::Address(address) => {
                            let function = self.heap.get(address.clone());
                            self.execute(&function.clone());

                        }
                        SizedValue::FunctionPtr(function_ptr) => {
                            function_ptr(self);
                        }
                        _ => panic!("Expected address or function pointer, got {:?}", address),
                    }
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
                OpCode::Import(address, module) => {
                    let module_name = module.clone();
                    let module = self.stdlib.get(&module_name).unwrap();
                    self.store.set(address.clone(), module.clone());
                }
                OpCode::ListAccess => {
                    let index = self.stack.pop();
                    let list = self.stack.pop();
                    let list = list.as_address();
                    let list = self.heap.get(list.clone());
                    let list = list.as_list();
                    let value = list[index.as_usize()].clone();
                    self.stack.push(value);
                }
                OpCode::PropertyLoad => {
                    let property_address = self.stack.pop();
                    let property_address = property_address.as_address();
                    let property = self.heap.get(property_address.clone());

                    let object = self.stack.pop();
                    let object_address = object.as_address();
                    let object_value = self.heap.get(object_address.clone());

                    let value = match object_value {
                        UnsizedValue::List(list) => {
                            let value = list.1.get(property.as_string()).unwrap();
                            value.clone()
                        }
                        _ => panic!("Not a list"),
                    };

                    self.stack.push(value);
                }
                OpCode::Dup => {
                    let value = self.stack.pop();
                    self.stack.push(value.clone());
                    self.stack.push(value);
                }
            }

            i += 1;
        }

        println!("Stack: {:?}", self.stack);
        println!("Heap: {:?}", self.heap);
        println!("Variable Store: {:?}", self.store);
    }

    pub fn run(&mut self, program: &UnsizedValue) {
        // let gc_handle = self.start_gc_thread();
        // let gc_thread = gc_handle.thread();

        self.execute(program);
    }

    fn start_gc_thread(&self) -> JoinHandle<()> {
        let gc_handle = thread::spawn({
            // let mut stack = self.stack.clone();
            // let mut heap = self.heap;
            // let mut store = self.store.clone();
            move || loop {
                thread::park();
                // let start = std::time::Instant::now();
                // gc(&mut stack, &mut heap, &mut store);
                // let end = std::time::Instant::now();
                // let duration = end.duration_since(start);
                // println!("GC took {:?}", duration);
            }
        });
        gc_handle
    }
}
