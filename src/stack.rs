use std::fmt::{Debug, Display, Formatter};
use std::ops::{Add, BitAnd, BitOr, Div, Mul, Neg, Not, Rem, Sub};
use crate::Vm;

#[derive(Clone)]
pub enum SizedValue {
    I64(i64),
    F64(f64),
    Bool(bool),
    Address(usize),
    FunctionPtr(fn(&mut Vm)),
    Null,
}

impl Debug for SizedValue {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            SizedValue::I64(i) => write!(f, "{}", i),
            SizedValue::F64(val) => write!(f, "{}", val),
            SizedValue::Bool(b) => write!(f, "{}", b),
            SizedValue::Address(a) => write!(f, "{}", a),
            SizedValue::FunctionPtr(_) => write!(f, "<function-ptr>"),
            SizedValue::Null => write!(f, "<null>"),
        }
    }
}

impl PartialEq for SizedValue {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (SizedValue::I64(a), SizedValue::I64(b)) => a == b,
            (SizedValue::F64(a), SizedValue::F64(b)) => a == b,
            (SizedValue::Bool(a), SizedValue::Bool(b)) => a == b,
            (SizedValue::Address(a), SizedValue::Address(b)) => a == b,
            (SizedValue::Null, SizedValue::Null) => true,
            _ => false,
        }
    }
}

impl PartialOrd for SizedValue {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match (self, other) {
            (SizedValue::I64(a), SizedValue::I64(b)) => a.partial_cmp(b),
            (SizedValue::F64(a), SizedValue::F64(b)) => a.partial_cmp(b),
            (SizedValue::Bool(a), SizedValue::Bool(b)) => a.partial_cmp(b),
            (SizedValue::Address(a), SizedValue::Address(b)) => a.partial_cmp(b),
            (SizedValue::Null, SizedValue::Null) => Some(std::cmp::Ordering::Equal),
            _ => None,
        }
    }
}

impl Display for SizedValue {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            SizedValue::I64(value) => write!(f, "{}", value),
            SizedValue::F64(value) => write!(f, "{}", value),
            SizedValue::Bool(value) => write!(f, "{}", value),
            SizedValue::Address(value) => write!(f, "{}", value),
            SizedValue::FunctionPtr(_) => write!(f, "<function-ptr>"),
            SizedValue::Null => write!(f, "null"),
        }
    }
}


impl SizedValue {
    pub fn as_address(&self) -> &usize {
        match self {
            SizedValue::Address(address) => address,
            _ => panic!("Cannot convert to address"),
        }
    }

    pub fn as_usize(&self) -> usize {
        match self {
            SizedValue::I64(address) => *address as usize,
            _ => panic!("Cannot convert to usize"),
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

#[derive(Debug)]
pub struct Stack {
    pub(crate) values: Vec<SizedValue>,
}

impl Stack {
    pub fn new() -> Stack {
        let values = Vec::with_capacity(100);
        Stack { values }
    }

    pub fn push(&mut self, value: SizedValue) {
        self.values.push(value);
    }

    pub fn pop(&mut self) -> SizedValue {
        self.values.pop().unwrap()
    }
}
