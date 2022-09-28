use std::ops::{Add, BitAnd, BitOr, Div, Mul, Neg, Not, Rem, Sub};
use std::sync::{Arc, Mutex};
use crate::heap::Heap;
use crate::UnsizedValue;

#[derive(Debug, Clone, PartialOrd, PartialEq)]
pub enum SizedValue {
    I64(i64),
    F32(f32),
    Bool(bool),
    Address(usize),
}


impl SizedValue {
    pub fn as_address(&self) -> &usize {
        match self {
            SizedValue::Address(address) => address,
            _ => panic!("Cannot convert to address"),
        }
    }

    pub fn as_bool(&self) -> &bool {
        match self {
            SizedValue::Bool(b) => b,
            _ => panic!("Expected bool, got {:?}", self),
        }
    }
}


impl Add for SizedValue {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        match (self, other) {
            (SizedValue::I64(a), SizedValue::I64(b)) => SizedValue::I64(a + b),
            _ => panic!("Cannot add non-i64 values"),
        }
    }
}

impl Sub for SizedValue {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        match (self, other) {
            (SizedValue::I64(a), SizedValue::I64(b)) => SizedValue::I64(a - b),
            _ => panic!("Cannot subtract non-i32 values"),
        }
    }
}

impl Mul for SizedValue {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        match (self, other) {
            (SizedValue::I64(a), SizedValue::I64(b)) => SizedValue::I64(a * b),
            _ => panic!("Cannot multiply non-i32 values"),
        }
    }
}

impl Div for SizedValue {
    type Output = Self;

    fn div(self, other: Self) -> Self {
        match (self, other) {
            (SizedValue::I64(a), SizedValue::I64(b)) => SizedValue::I64(a / b),
            _ => panic!("Cannot divide non-i32 values"),
        }
    }
}

impl Rem for SizedValue {
    type Output = Self;

    fn rem(self, other: Self) -> Self {
        match (self, other) {
            (SizedValue::I64(a), SizedValue::I64(b)) => SizedValue::I64(a % b),
            _ => panic!("Cannot modulo non-i32 values"),
        }
    }
}

impl Neg for SizedValue {
    type Output = Self;

    fn neg(self) -> Self {
        match self {
            SizedValue::I64(a) => SizedValue::I64(-a),
            _ => panic!("Cannot negate non-i32 values"),
        }
    }
}

impl Not for SizedValue {
    type Output = Self;

    fn not(self) -> Self {
        match self {
            SizedValue::Bool(a) => SizedValue::Bool(!a),
            _ => panic!("Cannot negate non-bool values"),
        }
    }
}


impl BitAnd for SizedValue {
    type Output = Self;

    fn bitand(self, other: Self) -> Self {
        match (self, other) {
            (SizedValue::Bool(a), SizedValue::Bool(b)) => SizedValue::Bool(a & b),
            _ => panic!("Cannot bitwise and non-bool values"),
        }
    }
}

impl BitOr for SizedValue {
    type Output = Self;

    fn bitor(self, other: Self) -> Self {
        match (self, other) {
            (SizedValue::Bool(a), SizedValue::Bool(b)) => SizedValue::Bool(a | b),
            _ => panic!("Cannot bitwise or non-bool values"),
        }
    }
}


#[derive(Debug, Clone)]
pub struct Stack {
    pub(crate) values: Arc<Mutex<Vec<SizedValue>>>,
}

impl Stack {
    pub fn new() -> Stack {
        Stack {
            values: Arc::new(Mutex::new(Vec::new())),
        }
    }

    pub fn push(&mut self, value: SizedValue) {
        self.values.lock().unwrap().push(value);
    }

    pub fn pop(&mut self) -> SizedValue {
        self.values.lock().unwrap().pop().unwrap()
    }
}
