use crate::parser::BFInstruction;
use std::io::{Read, Write};

pub struct Interpreter<R: Read, W: Write> {
    mem: Vec<u8>,
    program: Vec<BFInstruction>,
    instruction_ptr: usize,
    data_ptr: usize,
    reader: R,
    writer: W,
}

impl<R: Read, W: Write> Interpreter<R, W> {
    pub fn load_program(program: Vec<BFInstruction>, reader: R, writer: W) -> Self {
        Self {
            mem: vec![0; 30000],
            program,
            instruction_ptr: 0,
            data_ptr: 0,
            reader,
            writer,
        }
    }
    pub fn interpret_1_op(&mut self) -> bool {
        let op_code = self.program.get(self.instruction_ptr);
        if let Some(op_code) = op_code {
            match op_code {
                BFInstruction::IncPtr => self.data_ptr = self.data_ptr.wrapping_add(1),
                BFInstruction::DecPtr => self.data_ptr = self.data_ptr.wrapping_sub(1),
                BFInstruction::Inc => {
                    self.mem[self.data_ptr] = self.mem[self.data_ptr].wrapping_add(1)
                }
                BFInstruction::Dec => {
                    self.mem[self.data_ptr] = self.mem[self.data_ptr].wrapping_sub(1)
                }
                BFInstruction::Read => {
                    self.reader
                        .read_exact(&mut self.mem[self.data_ptr..self.data_ptr + 1])
                        .unwrap();
                }
                BFInstruction::Write => {
                    self.writer
                        .write_all(&self.mem[self.data_ptr..self.data_ptr + 1])
                        .unwrap();
                }
                BFInstruction::LoopStart => {
                    if self.mem[self.data_ptr] == 0 {
                        let mut bracket_nesting = 1;
                        let saved_pc = self.instruction_ptr;
                        while bracket_nesting > 0 {
                            self.instruction_ptr += 1;
                            if let Some(instruction) = self.program.get(self.instruction_ptr) {
                                match instruction {
                                    BFInstruction::LoopEnd => {
                                        bracket_nesting -= 1;
                                    }
                                    BFInstruction::LoopStart => {
                                        bracket_nesting += 1;
                                    }
                                    _ => {}
                                }
                            } else {
                                eprintln!("unmatched [ at pc= {}", saved_pc);
                            }
                        }
                    }
                }
                BFInstruction::LoopEnd => {
                    if self.mem[self.data_ptr] != 0 {
                        let mut bracket_nesting = 1;
                        let saved_pc = self.instruction_ptr;
                        while bracket_nesting > 0 {
                            self.instruction_ptr -= 1;
                            if let Some(instruction) = self.program.get(self.instruction_ptr) {
                                match instruction {
                                    BFInstruction::LoopStart => {
                                        bracket_nesting -= 1;
                                    }
                                    BFInstruction::LoopEnd => {
                                        bracket_nesting += 1;
                                    }
                                    _ => {}
                                }
                            } else {
                                eprintln!("unmatched ] at pc= {}", saved_pc);
                            }
                        }
                    }
                }
            }
            self.instruction_ptr += 1;
            true
        } else {
            false
        }
    }
    pub fn exec_program(&mut self) {
        while self.interpret_1_op() {}
    }
}
