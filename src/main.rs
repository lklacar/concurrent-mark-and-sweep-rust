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

fn main() {
    let program = vec![
        PushI32(0),
        PushString("sum".to_string()),
        Store,

        PushI32(0),
        PushString("i".to_string()),
        Store,

        PushI32(200),
        PushString("i".to_string()),
        Load,

        Gt,
        JumpIfFalse(200),
        PushString("sum".to_string()),
        Load,
        PushString("i".to_string()),
        Load,
        Add,
        PushString("sum".to_string()),
        Store,

        PushString("i".to_string()),
        Load,
        PushI32(1),
        Add,
        PushString("i".to_string()),
        Store,

        Jump(-19),
    ];

    let program = UnsizedValue::Function(program);

    Vm::new(program).run();
}
