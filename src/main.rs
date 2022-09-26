use crate::vm::Vm;

mod gc;
mod heap;
mod stack;
mod vm;

fn main() {
    Vm::new().run();
}
