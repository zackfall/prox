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
    println!("{name:<-16} {constant:?} {value}");
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
    print!("{offset:04} ");

    let instruction = chunk.get_value(offset);
    match instruction {
        Val::OpCode(op_code) => match op_code {
            OpCode::OpConstant => constant_instruction("OpConstant".to_owned(), chunk, offset),
            OpCode::OpReturn => simple_instruction("OpReturn".to_owned(), offset),
        },
        _ => {
            // if let Val::U8(val) = instruction {
            //     println!("Unknown OpCode {val}");
            // }
            // offset + 1
            println!("Unkown OpCode {:?}", instruction);
            offset + 1
        }
    }
}

pub fn simple_instruction(name: String, offset: usize) -> usize {
    println!("{name}");
    return offset + 1;
}
