#[derive(Debug, Clone)]
pub enum OpCode {
    Add,
    Sub,
    Mul,
    Div,
    Mod,
    Neg,
    Not,
    And,
    Or,
    Eq,
    Neq,
    Lt,
    Gt,
    Lte,
    Gte,

    PushI32(i32),
    PushF32(f32),
    PushString(String),
    PushBool(bool),
    PushObject,
    PushList,

    Function(Vec<OpCode>),

    Store,
    Load,

    Jump(i64),
    JumpIfFalse(i64),

    Call,
    Return,
}