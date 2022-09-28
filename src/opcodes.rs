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

            let opcode = Self::from_u64(vec, &mut i, opcode);

            result.push(opcode);

            i += 1;
        }

        return result;
    }

    fn from_u64(vec: &Vec<u64>, mut i: &mut usize, opcode: u64) -> OpCode {
        let opcode = match opcode {
            1 => OpCode::Add,
            2 => OpCode::Sub,
            3 => OpCode::Mul,
            4 => OpCode::Div,
            5 => OpCode::Mod,
            6 => OpCode::Neg,
            7 => OpCode::Not,
            8 => OpCode::And,
            9 => OpCode::Or,
            10 => OpCode::Eq,
            11 => OpCode::Neq,
            12 => OpCode::Lt,
            13 => OpCode::Gt,
            14 => OpCode::Lte,
            15 => OpCode::Gte,
            16 => {
                let value = Self::consume(vec, &mut i);
                OpCode::PushI64(value as i64)
            }
            17 => {
                let value = Self::consume(vec, &mut i);
                let value = f64::from_bits(value);
                OpCode::PushF64(value)
            }
            18 => {
                let size = Self::consume(vec, &mut i);
                let u64s = vec[(*i + 1)..(*i + size as usize + 1)].to_vec();
                *i += size as usize;

                let bytes = u64s
                    .iter()
                    .flat_map(|u| u.to_le_bytes().to_vec())
                    .collect::<Vec<u8>>();

                let string = String::from_utf8(bytes).unwrap();
                let string = string.trim_end_matches(char::from(0)).to_string();
                OpCode::PushString(string)
            }
            19 => {
                *i += 1;
                let value = vec[*i];
                let value = value != 0;
                OpCode::PushBool(value)
            }
            20 => OpCode::PushObject,
            21 => OpCode::PushList,
            22 => {
                let size = Self::consume(vec, &mut i);
                let mut func = Vec::new();
                let mut j = 0;
                while j < size {
                    let opcode = Self::consume(vec, &mut i);
                    let opcode = Self::from_u64(vec, &mut i, opcode);
                    func.push(opcode);
                    j += 1;
                }

                OpCode::Function(func)
            },
            23 => {
                *i += 1;
                let address = vec[*i];
                OpCode::Store(address as usize)
            }
            24 => {
                let offset = Self::consume(vec, &mut i);
                OpCode::Load(offset as usize)
            }
            25 => {
                let offset = Self::consume(vec, &mut i);
                OpCode::Jump(offset as i64)
            }
            26 => {
                *i += 1;
                let offset = vec[*i];
                OpCode::JumpIfFalse(offset as i64)
            }
            27 => OpCode::Call,
            28 => OpCode::Return,
            _ => panic!("Unknown opcode {}", opcode),
        };
        opcode
    }

    fn consume(vec: &Vec<u64>, i: &mut usize) -> u64 {
        *i += 1;
        let a = vec[*i];
        a
    }
}
