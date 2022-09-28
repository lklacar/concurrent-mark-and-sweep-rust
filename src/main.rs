extern crate core;

use crate::heap::UnsizedValue;
use crate::OpCode::*;
use crate::opcodes::OpCode;
use crate::vm::Vm;

mod gc;
mod heap;
mod stack;
mod vm;
mod opcodes;
mod store;

fn main() {
    let program = vec![
        PushI64(0),
        PushString("sum".to_string()),
        Store,

        PushI64(0),
        PushString("i".to_string()),
        Store,

        PushString("i".to_string()),
        Load,
        PushI64(10000),
        Lt,

        JumpIfFalse(500),
        PushString("i".to_string()),
        Load,
        PushString("sum".to_string()),
        Load,
        Add,
        PushString("sum".to_string()),
        Store,

        PushString("i".to_string()),
        Load,
        PushI64(1),
        Add,
        PushString("i".to_string()),
        Store,
        Jump(-19),
    ];

    let program = UnsizedValue::Function(program);

    Vm::new().run(&program);
}
