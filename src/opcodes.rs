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

    PushI64(i64),
    PushF64(f64),
    PushString(String),
    PushBool(bool),
    PushObject,
    PushList,

    Function(Vec<OpCode>),

    Store(usize),
    Load(usize),

    Jump(i64),
    JumpIfFalse(i64),

    Call,
    Return,
}

impl OpCode {
    pub fn from_vec(vec: &Vec<u64>) -> Vec<OpCode> {
        let mut result = Vec::new();

        let mut i = 0;
        while i < vec.len() {
            let opcode = vec[i];

            match opcode {
                1 => result.push(OpCode::Add),
                2 => result.push(OpCode::Sub),
                3 => result.push(OpCode::Mul),
                4 => result.push(OpCode::Div),
                5 => result.push(OpCode::Mod),
                6 => result.push(OpCode::Neg),
                7 => result.push(OpCode::Not),
                8 => result.push(OpCode::And),
                9 => result.push(OpCode::Or),
                10 => result.push(OpCode::Eq),
                11 => result.push(OpCode::Neq),
                12 => result.push(OpCode::Lt),
                13 => result.push(OpCode::Gt),
                14 => result.push(OpCode::Lte),
                15 => result.push(OpCode::Gte),
                16 => {
                    let value = Self::consume(vec, &mut i);
                    result.push(OpCode::PushI64(value as i64))
                }
                17 => {
                    let value = Self::consume(vec, &mut i);
                    let value = f64::from_bits(value);
                    result.push(OpCode::PushF64(value))
                }
                18 => {
                    let size = Self::consume(vec, &mut i);
                    let u64s = vec[(i + 1)..(i + size as usize + 1)].to_vec();
                    i += size as usize;

                    let bytes = u64s.iter()
                        .flat_map(|u| u.to_le_bytes().to_vec())
                        .collect::<Vec<u8>>();

                    let string = String::from_utf8(bytes).unwrap();
                    let string = string.trim_end_matches(char::from(0)).to_string();
                    result.push(OpCode::PushString(string));
                }
                19 => {
                    i += 1;
                    let value = vec[i];
                    let value = value != 0;
                    result.push(OpCode::PushBool(value))
                }
                20 => result.push(OpCode::PushObject),
                21 => result.push(OpCode::PushList),
                22 => result.push(OpCode::Function(Vec::new())),
                23 => {
                    i += 1;
                    let address = vec[i];
                    result.push(OpCode::Store(address as usize));
                }
                24 => {
                    let offset = Self::consume(vec, &mut i);
                    result.push(OpCode::Load(offset as usize));
                }
                25 => {
                    let offset = Self::consume(vec, &mut i);
                    result.push(OpCode::Jump(offset as i64));
                }
                26 => {
                    i += 1;
                    let offset = vec[i];
                    result.push(OpCode::JumpIfFalse(offset as i64));
                }
                27 => result.push(OpCode::Call),
                28 => result.push(OpCode::Return),
                _ => panic!("Unknown opcode {}", opcode),
            }

            i += 1;
        }


        return result;
    }

    fn consume(vec: &Vec<u64>, i: &mut usize) -> u64 {
        *i += 1;
        let a = vec[*i];
        a
    }
}