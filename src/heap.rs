use crate::stack::SizedValue;
use std::collections::HashMap;
use std::fmt::{Debug, Display};

use crate::{OpCode, Vm};

#[derive(Clone)]
pub enum UnsizedValue {
    String(String),
    List((Vec<SizedValue>, HashMap<String, SizedValue>)),
    Object(HashMap<String, SizedValue>),
    Function(Vec<OpCode>),
    Empty,
}

fn length(vm: &mut Vm) {
    let value = vm.stack.pop();
    match value {
        SizedValue::Address(s) => {
            let value = vm.heap.get(s);
            match value {
                UnsizedValue::String(s) => vm.stack.push(SizedValue::I64(s.len() as i64)),
                UnsizedValue::List(l) => vm.stack.push(SizedValue::I64(l.0.len() as i64)),
                _ => panic!("Expected string or list, got {:?}", value),
            }
        }
        _ => panic!("Expected address, got {:?}", value),
    }
}

fn for_each(vm: &mut Vm) {
    let list_address = vm.stack.pop();
    let list_address = list_address.as_address();
    let list = vm.heap.get(list_address.clone()).clone();

    let function_address = vm.stack.pop();

    match list {
        UnsizedValue::List(l) => {
            for value in l.0.iter() {
                vm.stack.push(value.clone());

                // Do this without cloning
                let function = vm.heap.get(function_address.as_address().clone()).clone();

                vm.execute(&function);
            }
        }
        _ => panic!("Expected list, got"),
    }
}

impl Display for UnsizedValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            UnsizedValue::String(s) => write!(f, "{}", s),
            UnsizedValue::List(l) => {
                write!(f, "[")?;
                for (i, v) in l.0.iter().enumerate() {
                    write!(f, "{}", v)?;
                    if i != l.0.len() - 1 {
                        write!(f, ", ")?;
                    }
                }
                write!(f, "]")
            }
            UnsizedValue::Object(o) => {
                write!(f, "{{")?;
                for (i, (k, v)) in o.iter().enumerate() {
                    write!(f, "{}: {}", k, v)?;
                    if i != o.len() - 1 {
                        write!(f, ", ")?;
                    }
                }
                write!(f, "}}")
            }
            UnsizedValue::Function(_) => write!(f, "<function>"),
            UnsizedValue::Empty => write!(f, "<empty>"),
        }
    }
}

impl Debug for UnsizedValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            UnsizedValue::String(s) => write!(f, "{}", s),
            UnsizedValue::List(l) => {
                write!(f, "[")?;
                for (i, v) in l.0.iter().enumerate() {
                    write!(f, "{:?}", v)?;
                    if i != l.0.len() - 1 {
                        write!(f, ", ")?;
                    }
                }
                write!(f, "]")
            }
            UnsizedValue::Object(o) => write!(f, "{:?}", o),
            UnsizedValue::Function(_) => write!(f, "Function"),
            UnsizedValue::Empty => write!(f, "Empty"),
        }
    }
}


impl UnsizedValue {
    pub fn as_function(&self) -> &Vec<OpCode> {
        match self {
            UnsizedValue::Function(f) => f,
            _ => panic!("Expected function, got {:?}", self),
        }
    }

    pub fn as_string(&self) -> &String {
        match self {
            UnsizedValue::String(s) => s,
            _ => panic!("Expected string, got {:?}", self),
        }
    }

    pub fn as_list(&self) -> &Vec<SizedValue> {
        match self {
            UnsizedValue::List(l) => &l.0,
            _ => panic!("Expected list, got {:?}", self),
        }
    }

    pub fn new_list(list: Vec<SizedValue>) -> UnsizedValue {
        let mut methods = HashMap::new();
        methods.insert("length".to_string(), SizedValue::FunctionPtr(length));
        methods.insert("forEach".to_string(), SizedValue::FunctionPtr(for_each));

        UnsizedValue::List((list, methods))
    }
}

#[derive(Debug)]
pub struct Heap {
    pub(crate) values: Vec<UnsizedValue>,
}


impl Heap {
    pub fn new() -> Heap {
        Heap { values: Vec::new() }
    }

    pub fn alloc(&mut self, value: UnsizedValue) -> usize {
        self.values.push(value);
        self.values.len() - 1
    }

    pub fn push(&mut self, value: UnsizedValue) {
        self.values.push(value);
    }

    pub fn pop(&mut self) -> UnsizedValue {
        self.values.pop().unwrap()
    }

    pub fn get(&self, address: usize) -> &UnsizedValue {
        &self.values[address]
    }
}
