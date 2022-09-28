extern crate core;

use crate::heap::UnsizedValue;
use crate::opcodes::OpCode;
use crate::vm::Vm;
use crate::OpCode::*;
use std::fs::File;
use std::io::Read;

mod gc;
mod heap;
mod opcodes;
mod stack;
mod store;
mod vm;

pub fn new_program(path: &str) -> Vec<u64> {
    let mut file = File::open(path).unwrap();
    let mut data = Vec::new();
    file.read_to_end(&mut data).unwrap();

    let data = data
        .chunks(8)
        .map(|x| {
            let mut y = [0; 8];
            y.copy_from_slice(x);
            u64::from_be_bytes(y)
        })
        .collect::<Vec<u64>>();

    data
}

fn main() {
    let program =
        new_program("/home/luka/Projects/Experiments/yapl/yapl-compiler/program.bytecode");
    // let program = vec![
    //     PushI64(0),
    //     Store(0),
    //     PushI64(0),
    //     Store(1),
    //     Load(1),
    //     PushI64(10),
    //     Lt,
    //     JumpIfFalse(500),
    //     Load(1),
    //     Load(0),
    //     Add,
    //     Store(0),
    //     Load(1),
    //     PushI64(1),
    //     Add,
    //     Store(1),
    //     Jump(-13),
    // ];
    let program = OpCode::from_vec(&program);

    let program = UnsizedValue::Function(program);

    Vm::new().run(&program);
}
