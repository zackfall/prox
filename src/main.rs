use std::env::args;

use prox::chunk::{Chunk, OpCode, Val};
use prox::debug::disassemble_chunk;

fn main() {
    let args: Vec<String> = args().collect();
    let mut chunk = Chunk::new();
    let constant = chunk.add_constant(1.2);
    chunk.push_chunk(Val::OpCode(OpCode::OpConstant), 13);
    chunk.push_chunk(Val::U8(constant as u8), 13);
    let constant2 = chunk.add_constant(1.5);
    chunk.push_chunk(Val::OpCode(OpCode::OpConstant), 13);
    chunk.push_chunk(Val::U8(constant2 as u8), 13);
    let constant3 = chunk.add_constant(4.2);
    chunk.push_chunk(Val::OpCode(OpCode::OpConstant), 15);
    chunk.push_chunk(Val::U8(constant3 as u8), 15);
    chunk.push_chunk(Val::OpCode(OpCode::OpReturn), 16);
    disassemble_chunk(&chunk, "test chunk".to_owned());
    println!("{chunk:?}");
    chunk.free_chunk();
    println!("free {chunk:?}");
}
