use std::io::prelude::*;

mod opcode;
use opcode::{Opcode, Code};

struct Interpreter {
    stack: Vec<u8>,
}

impl std::default::Default for Interpreter {
    fn default() -> Self {
        Self { stack: vec![0; 1] }
    }
}

impl Interpreter {
    fn run(&mut self, data: Vec<u8>) -> Result<(), Box<dyn std::error::Error>> {
        let code = Code::from(data)?;
        let code_len = code.instrs.len();
        let mut pc = 0; // Program counter
        let mut ps = 0; // Stack Pointer
        loop {
            if pc >= code_len {
                break;
            }
            match code.instrs[pc] {
                Opcode::SHL => ps = if ps == 0 { 0 } else { ps - 1 },
                Opcode::SHR => {
                    ps += 1;
                    if ps == self.stack.len() {
                        self.stack.push(0)
                    }
                }
                Opcode::ADD => {
                    self.stack[ps] = self.stack[ps].overflowing_add(1).0;
                }
                Opcode::SUB => {
                    self.stack[ps] = self.stack[ps].overflowing_sub(1).0;
                }
                Opcode::PUTCHAR => {
                    std::io::stdout().write_all(&[self.stack[ps]])?;
                }
                Opcode::GETCHAR => {
                    let mut buf: Vec<u8> = vec![0; 1];
                    std::io::stdin().read_exact(&mut buf)?;
                    self.stack[ps] = buf[0];
                }
                Opcode::LB => {
                    if self.stack[ps] == 0x00 {
                        pc = code.jtable[&pc];
                    }
                }
                Opcode::RB => {
                    if self.stack[ps] != 0x00 {
                        pc = code.jtable[&pc];
                    }
                }
            }
            pc += 1;
        }
        Ok(())
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = std::env::args().collect();
    assert!(args.len() >= 2);
    let data = std::fs::read(&args[1])?;

    let mut interpreter = Interpreter::default();
    interpreter.run(data)?;

    Ok(())
}