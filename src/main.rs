use std::env::args;

use prox::chunk::{Chunk, OpCode, Val};
use prox::debug::disassemble_chunk;

fn main() {
    let args: Vec<String> = args().collect();
    let mut chunk = Chunk::new();
    let constant = chunk.add_constant(1.2);
    chunk.push_chunk(Val::OpCode(OpCode::OpConstant));
    chunk.push_chunk(Val::U8(constant as u8));
    let constant2 = chunk.add_constant(1.5);
    chunk.push_chunk(Val::OpCode(OpCode::OpConstant));
    chunk.push_chunk(Val::U8(constant2 as u8));
    chunk.push_chunk(Val::OpCode(OpCode::OpReturn));
    disassemble_chunk(&chunk, "test chunk".to_owned());
    println!("{chunk:?}");
    chunk.free_chunk();
    println!("free {chunk:?}");
}
