use crate::chunk::{Chunk, OpCode, Val};

/// This function get a constant from the chunk and then get a value
/// from the values vector, i uses the index of the Constant
pub fn constant_instruction(name: String, chunk: &Chunk, offset: usize) -> usize {
    let constant = chunk.get_value(offset + 1);
    let values = chunk.get_constants().get_values();
    // get the index of the constant
    let index = match constant.clone() {
        Val::U8(num) => num - 1,
        // This will never happen
        _ => 0,
    };
    let value = values[index as usize];
    println!("{name:<-16} {constant} '{value}'");
    offset + 2
}

pub fn disassemble_chunk(chunk: &Chunk, name: String) {
    println!("== {name} ==");
    let mut offset = 0;

    for idx in 0..chunk.len() {
        offset = disassemble_instruction(chunk, idx);
    }
}

pub fn disassemble_instruction(chunk: &Chunk, offset: usize) -> usize {
    let instruction = chunk.get_value(offset);
    match instruction.clone() {
        Val::U8(_) => (),
        Val::OpCode(_) => {
            print!("{offset:04} ");
            if offset > 0 && chunk.lines()[offset] == chunk.lines()[offset - 1] {
                print!("   | ");
            } else {
                print!("{:>4} ", chunk.lines()[offset]);
            }
        }
    }

    // let instruction = chunk.get_value(offset);
    match instruction {
        Val::U8(_) => constant_instruction("OpConstant".to_owned(), chunk, offset - 1),
        Val::OpCode(op_code) => match op_code {
            // OpCode::OpConstant => constant_instruction("OpConstant".to_owned(), chunk, offset),
            OpCode::OpReturn => simple_instruction("OpReturn".to_owned(), offset),
            // i don't have fucking idea what this does
            _ => 0,
        },
        _ => {
            println!("Unkown OpCode {:?}", instruction);
            offset + 1
        }
    }
}

pub fn simple_instruction(name: String, offset: usize) -> usize {
    println!("{name}");
    return offset + 1;
}
