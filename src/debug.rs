use crate::chunk::{Chunk, OpCode};

/// This function get a constant from the chunk and then get a value
/// from the values vector, i uses the index of the Constant
pub fn constant_instruction(name: String, chunk: &Chunk, offset: usize) -> usize {
    let constant = chunk.get_value(offset + 1);
    let values = chunk.get_constants().get_values();
    let value = values[constant as usize];
    println!("{name:<-16} {constant} '{value}'");
    offset + 2
}

pub fn disassemble_chunk(chunk: &Chunk, name: String) {
    println!("== {name} ==");
    let mut _offset = 0;

    for idx in 0..chunk.len() {
        _offset = disassemble_instruction(chunk, idx);
    }
}

pub fn disassemble_instruction(chunk: &Chunk, offset: usize) -> usize {
    let instruction = chunk.get_op_codes(offset);
    // print!("{offset:04} ");

    match instruction {
        OpCode::OpU8(_) => print!(""),
        _ => {
            print!("{offset:04} ");
            if offset > 0 && chunk.lines()[offset] == chunk.lines()[offset - 1] {
                print!("   | ");
            } else {
                print!("{:>4} ", chunk.lines()[offset]);
            }
        }
    }

    match instruction {
        OpCode::OpConstant(_) => constant_instruction("OpConstant".to_owned(), chunk, offset),
        OpCode::OpReturn(_) => simple_instruction("OpReturn".to_owned(), offset),
        OpCode::OpU8(n) => offset + n as usize,
    }
}

pub fn simple_instruction(name: String, offset: usize) -> usize {
    println!("{name}");
    offset + 1
}
