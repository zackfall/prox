use std::env::args;

use prox::chunk::{Chunk, OpCode};
use prox::debug::disassemble_chunk;
use prox::vm::VM;

fn main() {
    let _args: Vec<String> = args().collect();
    let mut vm = VM::default();
    vm.init();
    let mut chunk = Chunk::new();
    let constant = chunk.add_constant(1.2);
    chunk.push_chunk(OpCode::OpConstant(0), 13);
    chunk.push_chunk(OpCode::OpU8((constant.saturating_sub(1)) as u8), 13);
    let constant2 = chunk.add_constant(1.5);
    chunk.push_chunk(OpCode::OpConstant(1), 13);
    chunk.push_chunk(OpCode::OpU8((constant2.saturating_sub(1)) as u8), 13);
    let constant3 = chunk.add_constant(4.2);
    chunk.push_chunk(OpCode::OpConstant(2), 15);
    chunk.push_chunk(OpCode::OpU8((constant3.saturating_sub(1)) as u8), 15);
    let constant4 = chunk.add_constant(2.3);
    chunk.push_chunk(OpCode::OpConstant(3), 15);
    chunk.push_chunk(OpCode::OpU8((constant4.saturating_sub(1)) as u8), 15);
    let constant5 = chunk.add_constant(0.4);
    chunk.push_chunk(OpCode::OpConstant(4), 16);
    chunk.push_chunk(OpCode::OpU8((constant5.saturating_sub(1)) as u8), 16);
    chunk.push_chunk(OpCode::OpReturn(5), 16);
    disassemble_chunk(&chunk, "test chunk".to_owned());
    chunk.encode_lines();
    vm.interpret(chunk.clone());
    vm.free();
    // println!("{chunk:?}");
    chunk.free_chunk();
}
