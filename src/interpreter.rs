use std::io::{Read, Write};

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub enum BFInstruction {
    //ptr++;
    IncPtr,
    //ptr--;
    DecPtr,
    // *ptr++;
    Inc,
    // *ptr--;
    Dec,
    //I/O
    Read,
    Write,
    //ControlFlow
    LoopStart,
    LoopEnd,
}

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
                    let mut depth = 1;
                    if self.mem[self.data_ptr] == 0 {
                        while depth != 0 {
                            #[cfg(debug_assertions)]
                            println!("depth : {} i_ptr : {}", depth, self.instruction_ptr);
                            self.instruction_ptr += 1;
                            if let Some(i) = self.program.get(self.instruction_ptr) {
                                match i {
                                    BFInstruction::LoopStart => depth += 1,
                                    BFInstruction::LoopEnd => depth -= 1,
                                    _ => {}
                                }
                            }
                        }
                    }
                }
                BFInstruction::LoopEnd => {
                    let mut depth = 1;
                    if self.mem[self.data_ptr] != 0 {
                        while depth != 0 {
                            #[cfg(debug_assertions)]
                            println!("depth : {} i_ptr : {}", depth, self.instruction_ptr);
                            self.instruction_ptr -= 1;
                            if let Some(i) = self.program.get(self.instruction_ptr) {
                                match i {
                                    BFInstruction::LoopStart => depth -= 1,
                                    BFInstruction::LoopEnd => depth += 1,
                                    _ => {}
                                }
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
    pub fn get_program(&self) -> Vec<BFInstruction> {
        self.program.clone()
    }
}
