use std::fmt::Display;
use std::rc::Rc;
use std::slice::Iter;

use crate::line_encoder::LineEncoder;
use crate::memory::grow_array;
use crate::value::{Value, ValueArray};

#[derive(Debug, Clone)]
pub struct Chunk {
    code: Rc<Vec<OpCode>>,
    constants: ValueArray,
    lines: Vec<usize>,
    line_encoder: LineEncoder,
}

impl Chunk {
    pub fn add_constant(&mut self, value: Value) -> usize {
        self.constants.push_value(value);
        self.constants.len()
    }

    pub fn encode_lines(&mut self) {
        self.line_encoder.encode_lines(self.lines.clone())
    }

    pub fn decode_lines(&mut self) -> Vec<usize> {
        self.line_encoder.decode_lines()
    }

    pub fn get_constants(&self) -> ValueArray {
        self.constants.clone()
    }

    pub fn get_line(&self, idx: usize) -> usize {
        self.line_encoder.get_line(idx).unwrap_or(0)
    }

    pub fn get_value(&self, idx: usize) -> OpCode {
        self.code[idx].clone()
    }

    pub fn free_chunk(&mut self) {
        self.constants.free_value();
        self.lines = Vec::new();
        self.code = Rc::new(Vec::with_capacity(8));
        self.line_encoder = LineEncoder::new()
    }

    pub fn len(&self) -> usize {
        self.lines.len()
    }

    pub fn lines(&self) -> Vec<usize> {
        self.lines.clone()
    }

    pub fn iter(&self) -> Iter<OpCode> {
        self.code.iter()
    }

    pub fn new() -> Chunk {
        Chunk {
            code: Rc::new(Vec::with_capacity(8)),
            constants: ValueArray::new(),
            lines: Vec::new(),
            line_encoder: LineEncoder::new(),
        }
    }

    pub fn push_chunk(&mut self, byte: OpCode, line: usize) {
        let m_code = Rc::make_mut(&mut self.code);
        m_code.push(byte);
        self.lines.push(line);

        if m_code.capacity() == m_code.len() {
            self.code = grow_array(m_code);
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum OpCode {
    OpConstant(u8),
    OpU8(u8),
    OpReturn(u8),
}

impl OpCode {
    pub fn get_index(&self) -> usize {
        match self {
            OpCode::OpConstant(byte) => *byte as usize,
            OpCode::OpU8(byte) => *byte as usize,
            OpCode::OpReturn(byte) => *byte as usize,
        }
    }
}

impl Display for OpCode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.get_index())
    }
}
