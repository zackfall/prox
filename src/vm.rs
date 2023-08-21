use crate::{
    chunk::{Chunk, OpCode},
    debug::disassemble_instruction,
    value::Value,
    DEBUG_TRACE_EXECUTION,
};

#[derive(Debug, Clone)]
pub struct VM {
    chunk: Chunk,
    ip: Vec<OpCode>,
    ip_idx: usize,
}

#[derive(Debug, Clone, Copy)]
pub enum InterpretResult {
    InterpretOk,
    InterpretCompileError,
    InterpretRuntimeError,
}

impl VM {
    pub fn free(&self) {}

    pub fn init(&self) {}

    pub fn interpret(&mut self, chunk: Chunk) -> InterpretResult {
        self.chunk = chunk;
        self.ip = self.chunk.op_codes.clone();
        self.run()
    }

    pub fn new() -> VM {
        VM {
            chunk: Chunk::new(),
            ip: Vec::new(),
            ip_idx: 0,
        }
    }

    fn read_byte(&mut self) -> Option<OpCode> {
        if self.ip_idx < self.ip.len() {
            let byte = self.ip[self.ip_idx];
            self.ip_idx += 1;
            Some(byte)
        } else {
            None
        }
    }

    fn run(&mut self) -> InterpretResult {
        loop {
            if DEBUG_TRACE_EXECUTION {
                let offset = self.ip_idx;
                println!("DEBUG WITH OFFSET {offset}");
                disassemble_instruction(&self.chunk, offset);
            }

            match self.read_byte() {
                Some(instruction) => {
                    match instruction {
                        OpCode::OpReturn(_) => return InterpretResult::InterpretOk,
                        OpCode::OpConstant(_) => {
                            let constant: Value = self.chunk.get_constants().get_values()
                                [self.read_byte().unwrap_or(OpCode::OpReturn(0)).get_index()];
                            println!("{constant}");
                        }
                        _ => (),
                    }
                    if let OpCode::OpReturn(_) = instruction {
                        return InterpretResult::InterpretOk;
                    }
                }
                None => return InterpretResult::InterpretOk,
            }
        }
    }
}

impl Default for VM {
    fn default() -> Self {
        Self::new()
    }
}
