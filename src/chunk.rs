use std::fmt::Display;
use std::rc::Rc;
use std::slice::Iter;

use crate::memory::grow_array;
use crate::value::{Value, ValueArray};

#[derive(Debug, Clone)]
pub struct Chunk {
    offset: usize,
    code: Rc<Vec<Val>>,
    constants: ValueArray,
}

impl Chunk {
    pub fn add_constant(&mut self, value: Value) -> usize {
        self.constants.push_value(value);
        self.constants.len()
    }

    pub fn get_value(&self, idx: usize) -> Val {
        self.code[idx].clone()
    }

    pub fn get_constants(&self) -> ValueArray {
        self.constants.clone()
    }

    pub fn free_chunk(&mut self) {
        self.constants.free_value();
        self.code = Rc::new(Vec::with_capacity(8));
    }

    pub fn len(&self) -> usize {
        self.code.len()
    }

    pub fn iter(&self) -> Iter<Val> {
        self.code.iter()
    }

    pub fn new() -> Chunk {
        Chunk {
            offset: 0,
            code: Rc::new(Vec::with_capacity(8)),
            constants: ValueArray::new(),
        }
    }

    pub fn push_chunk(&mut self, byte: Val) {
        let m_code = Rc::make_mut(&mut self.code);
        m_code.push(byte);

        if m_code.capacity() == m_code.len() {
            self.code = grow_array(m_code);
        }
    }
}

#[derive(Debug, Clone)]
pub enum Val {
    U8(u8),
    OpCode(OpCode),
}

impl Display for Val {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Val::U8(num) => write!(f, "{}", num),
            Val::OpCode(op_code) => write!(f, "{}", op_code),
        }
    }
}

#[derive(Debug, Clone)]
pub enum OpCode {
    OpConstant,
    OpReturn,
}

impl OpCode {
    pub fn get_index(&self) -> usize {
        match self {
            OpCode::OpConstant => 1,
            OpCode::OpReturn => 0,
        }
    }
}

impl Display for OpCode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.get_index())
    }
}

// pub trait Pushable {
//     fn push_chunk(self, chunk: &mut Chunk);
// }

// impl Pushable for OpCode {
//     fn push_chunk(self, chunk: &mut Chunk) {
//         chunk.push_chunk(Val::OpCode(self))
//     }
// }

// impl Pushable for u8 {
//     fn push_chunk(self, chunk: &mut Chunk) {
//         chunk.push_chunk(Val::U8(self))
//     }
// }
