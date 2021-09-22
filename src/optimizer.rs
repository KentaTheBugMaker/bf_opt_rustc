use crate::interpreter::BFInstruction;
use crate::optimizer::OptInstruction::{Add, AddPtr, Dec, DecPtr, Inc, IncPtr, LoopEnd, LoopStart, Read, Sub, SubPtr, Write, JNZ, JZ, ZeroClear,Nop};
use crate::optimizer::OptLevel::{ IncDecOpt1, IncDecOpt2};

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum OptInstruction {
    Add(u8),
    Sub(u8),
    AddPtr(usize),
    SubPtr(usize),
    JZ(usize),
    JNZ(usize),
    Read,
    Write,
    //temporally use
    LoopStart,
    LoopEnd,
    //Add Sub AddPtr SubPtr 1 の特殊化
    Inc,
    Dec,
    IncPtr,
    DecPtr,
    //[-]
    ZeroClear,
    Nop,
}
#[derive(Eq, PartialEq)]
pub enum OptLevel{
    IncDecOpt1,
    IncDecOpt2,
    ZeroClear,
    JumpOpt,
}
pub fn optimize(program: &[BFInstruction],opt_level:OptLevel) -> Vec<OptInstruction> {
    let mut scope_stack = vec![];
    let mut prev_instruction = None;
    let mut burst_len = 1;
    let mut ir_stack = vec![];

    //まずDec Inc DecPtr IncPtr をまとめる
    //バースト長を計算
    fn emit_ir(prev_i: BFInstruction, burst_len: i32) -> OptInstruction {
        match prev_i {
            BFInstruction::IncPtr => AddPtr(burst_len as usize),
            BFInstruction::DecPtr => SubPtr(burst_len as usize),
            BFInstruction::Inc => Add(burst_len as u8),
            BFInstruction::Dec => Sub(burst_len as u8),
            BFInstruction::Read => Read,
            BFInstruction::Write => Write,
            BFInstruction::LoopStart => LoopStart,
            BFInstruction::LoopEnd => LoopEnd,
        }
    }
    for instruction in program {
        if let Some(prev_i) = prev_instruction {
            if prev_i == *instruction
                && !matches!(
                    prev_i,
                    BFInstruction::LoopEnd
                        | BFInstruction::LoopStart
                        | BFInstruction::Write
                        | BFInstruction::Read
                )
            {
                //バーストしている場合
                burst_len += 1;
            } else {
                //バーストが止まった場合
                ir_stack.push(emit_ir(prev_i, burst_len));
                burst_len = 1;
            }
        }
        prev_instruction.replace(*instruction);
    }

    //残りのIRを出力
    if let Some(prev_i) = prev_instruction {
        ir_stack.push(emit_ir(prev_i, burst_len));
    }
    if opt_level==IncDecOpt1{
        return ir_stack
    }
    //Add(1) Sub(1) AddPtr(1) SubPtr(1) をInc,Dec,IncPtr,DecPtrに特殊化
    ir_stack = ir_stack
        .iter()
        .map(|ir| match ir {
            Add(1) => Inc,
            Sub(1) => Dec,
            AddPtr(1) => IncPtr,
            SubPtr(1) => DecPtr,
            _ => *ir,
        })
        .collect();
    if opt_level==OptLevel::IncDecOpt2{
        return ir_stack;
    }

    if ir_stack.len() >=2{
        for i in 0..(ir_stack.len()-2){
            if ir_stack[i]==LoopStart && ir_stack[i+1]==Dec &&ir_stack[i+2]==LoopEnd{
                ir_stack[i]=ZeroClear;
                ir_stack[i+1]=Nop;
                ir_stack[i+2]=Nop;
            }
        }
    }
    ir_stack =ir_stack.iter().filter(|ir|**ir!=Nop).copied().collect();
    if opt_level==OptLevel::ZeroClear{
        return ir_stack;
    }
    for (ptr, instruction) in ir_stack.iter_mut().enumerate() {
        match instruction {
            LoopStart => {
                scope_stack.push((ptr, instruction));
            }
            LoopEnd => {
                let address = scope_stack.pop();
                if let Some((address, t_instruction)) = address {
                    *t_instruction = JZ(ptr + 1);
                    *instruction = JNZ(address+1);
                } else {
                    panic!("Invalid ]")
                }
            }
            _ => {}
        }
    }
    ir_stack

}
pub fn exec_opt_ir<R: std::io::Read, W: std::io::Write>(
    program: Vec<OptInstruction>,
    mut reader: R,
    mut writer: W,
) {
    //VM
    let mut instruction_ptr = 0;
    let mut data_ptr: usize = 0;
    let mut mem = vec![0u8; 30000];
    loop {
        let op_code = program.get(instruction_ptr);
        if let Some(op_code) = op_code {

            match op_code {
                IncPtr => data_ptr = data_ptr.wrapping_add(1),
                DecPtr => data_ptr = data_ptr.wrapping_sub(1),
                Inc => mem[data_ptr] = mem[data_ptr].wrapping_add(1),
                Dec => mem[data_ptr] = mem[data_ptr].wrapping_sub(1),
                Read => {
                    reader.read_exact(&mut mem[data_ptr..data_ptr + 1]).unwrap();
                }
                Write => {
                    writer.write_all(&mem[data_ptr..data_ptr + 1]).unwrap();
                }
                LoopStart => {
                    let mut depth = 1;
                    if mem[data_ptr] == 0 {
                        while depth != 0 {
                            instruction_ptr += 1;
                            if let Some(i) = program.get(instruction_ptr) {
                                match i {
                                    LoopStart => depth += 1,
                                    LoopEnd => depth -= 1,
                                    _ => {}
                                }
                            }
                        }

                    }
                }
                LoopEnd => {
                    let mut depth = 1;
                    if mem[data_ptr] != 0 {
                        while depth != 0 {
                            instruction_ptr -= 1;
                            if let Some(i) = program.get(instruction_ptr) {
                                match i {
                                    LoopStart => depth -= 1,
                                    LoopEnd => depth += 1,
                                    _ => {}
                                }
                            }
                        }

                    }
                }
                Add(x) =>  mem[data_ptr] = mem[data_ptr].wrapping_add(*x),
                Sub(x) =>  mem[data_ptr] = mem[data_ptr].wrapping_sub(*x),
                AddPtr(x) => data_ptr = data_ptr.wrapping_add(*x),
                SubPtr(x) => data_ptr = data_ptr.wrapping_sub(*x),
                JZ(x) => {
                    if mem[data_ptr] == 0 {
                        instruction_ptr = *x;
                    }
                }
                JNZ(x) => {
                    if mem[data_ptr] != 0 {
                        instruction_ptr = *x;
                    }
                }
                ZeroClear => {
                    mem[data_ptr]=0;
                }
                Nop => {}
            }

        }else{
            return;
        }
        instruction_ptr += 1;
    }
}
