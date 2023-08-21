use crate::{
    chunk::{Chunk, OpCode},
    debug::disassemble_instruction,
    value::Value,
    DEBUG_TRACE_EXECUTION, STACK_MAX,
};

#[derive(Debug, Clone)]
pub struct VM {
    chunk: Chunk,
    ip: Vec<OpCode>,
    ip_idx: usize,
    stack: Vec<Value>,
    stack_top: usize,
}

#[derive(Debug, Clone, Copy)]
pub enum InterpretResult {
    InterpretOk,
    InterpretCompileError,
    InterpretRuntimeError,
}

impl VM {
    pub fn free(&self) {}

    pub fn init(&mut self) {
        self.reset_stack();
    }

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
            stack: Vec::with_capacity(STACK_MAX),
            stack_top: 0,
        }
    }

    pub fn pop(&mut self) -> Value {
        self.stack_top -= 1;
        self.stack.pop().unwrap_or(0.0)
    }

    pub fn push(&mut self, value: Value) {
        self.stack.push(value);
        self.stack_top += 1;
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

    fn reset_stack(&mut self) {
        self.stack_top = self.stack.len();
    }

    fn run(&mut self) -> InterpretResult {
        loop {
            if DEBUG_TRACE_EXECUTION {
                let offset = self.ip_idx;
                if self.stack_top != 0 {
                    print!("     Stack: ");
                    for slot in &self.stack {
                        print!("[{slot}]");
                    }
                    println!();
                }
                disassemble_instruction(&self.chunk, offset);
            }

            match self.read_byte() {
                Some(instruction) => match instruction {
                    OpCode::OpReturn(_) => {
                        println!("{}", self.pop());
                        return InterpretResult::InterpretOk;
                    }
                    OpCode::OpConstant(_) => {
                        let constant: Value = self.chunk.get_constants().get_values()
                            [self.read_byte().unwrap_or(OpCode::OpReturn(0)).get_index()];
                        self.push(constant);
                        println!("{constant}");
                    }
                    _ => (),
                },
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
