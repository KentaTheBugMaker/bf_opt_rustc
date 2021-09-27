use crate::interpreter::BFInstruction;
use crate::optimizer::OptInstruction::{
    Add, AddPtr, Dec, DecPtr, Inc, IncPtr, Jnz, LoopEnd, LoopStart, MovingAdd, Nop, Read, Sub,
    SubPtr, Write, ZeroClear, JZ,
};
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Direction {
    Left,
    Right,
}
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Sign {
    Plus,
    Minus,
}
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum OptInstruction {
    Add(u8),
    Sub(u8),
    AddPtr(usize),
    SubPtr(usize),
    JZ(usize),
    Jnz(usize),
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
    //[-<+>] [-<->] [->+<] [->-<]
    //direction offset sign multiplier
    MovingAdd(Direction, usize, Sign, u8),
    //[AddPtr]
    PtrMoveRight(usize),
    //[SubPtr]
    PtrMoveLeft(usize),
    //最適化の最中に削除された命令の代わりに入れる
    Nop,
}
#[derive(Eq, PartialEq)]
pub enum OptLevel {
    IncDecOpt1,
    IncDecOpt2,
    ZeroClear,
    JumpOpt,
    LoopPtrMove,
    LoopDataMove,
}
pub fn optimize(program: &[BFInstruction], opt_level: OptLevel) -> Vec<OptInstruction> {
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
    if opt_level == OptLevel::IncDecOpt1 {
        return ir_stack;
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
    if opt_level == OptLevel::IncDecOpt2 {
        return ir_stack;
    }
    //opt [-] [+]
    if ir_stack.len() > 2 {
        for i in 0..(ir_stack.len() - 2) {
            if ir_stack[i] == LoopStart && ir_stack[i + 1] == Dec && ir_stack[i + 2] == LoopEnd {
                ir_stack[i] = ZeroClear;
                ir_stack[i + 1] = Nop;
                ir_stack[i + 2] = Nop;
            }
            if ir_stack[i] == LoopStart && ir_stack[i + 1] == Inc && ir_stack[i + 2] == LoopEnd {
                ir_stack[i] = ZeroClear;
                ir_stack[i + 1] = Nop;
                ir_stack[i + 2] = Nop;
            }
        }
    }
    ir_stack = ir_stack.iter().filter(|ir| **ir != Nop).copied().collect();
    if opt_level == OptLevel::ZeroClear {
        return ir_stack;
    }
    // opt [>]
    if ir_stack.len() > 2 {
        for i in 0..(ir_stack.len() - 2) {
            if ir_stack[i] == LoopStart && ir_stack[i + 2] == LoopEnd {
                match ir_stack[i + 1] {
                    IncPtr => ir_stack[i + 1] = OptInstruction::PtrMoveRight(1),
                    DecPtr => ir_stack[i + 1] = OptInstruction::PtrMoveLeft(1),
                    AddPtr(x) => ir_stack[i + 1] = OptInstruction::PtrMoveRight(x),
                    SubPtr(x) => ir_stack[i + 1] = OptInstruction::PtrMoveLeft(x),
                    _ => {
                        continue;
                    }
                }
                ir_stack[i] = Nop;
                ir_stack[i + 2] = Nop;
            }
        }
    }
    ir_stack = ir_stack.iter().filter(|ir| **ir != Nop).copied().collect();
    if opt_level == OptLevel::LoopPtrMove {
        return ir_stack;
    }
    // previous nop elimination there are no nops current ir so i can
    //0  1     2     3       4    5
    //[ Dec SubPtr Inc/Dec AddPtr ]
    //[ Dec AddPtr Inc/Dec SubPtr ]
    let test_inc_dec = |ir: OptInstruction| -> Option<(Sign, u8)> {
        match ir {
            Add(x) => Some((Sign::Plus, x)),
            Sub(x) => Some((Sign::Minus, x)),
            Inc => Some((Sign::Plus, 1)),
            Dec => Some((Sign::Minus, 1)),
            _ => None,
        }
    };
    if ir_stack.len() > 5 {
        for i in 0..(ir_stack.len() - 5) {
            let mut generated_ir = Nop;
            if ir_stack[i] == LoopStart && ir_stack[i + 1] == Dec && ir_stack[i + 5] == LoopEnd {
                match ir_stack[i + 2] {
                    AddPtr(i_x) => {
                        if ir_stack[i + 4] == SubPtr(i_x) {
                            if let Some(ir) = test_inc_dec(ir_stack[i + 3]) {
                                generated_ir = MovingAdd(Direction::Right, i_x, ir.0, ir.1)
                            }
                        }
                    }
                    SubPtr(i_x) => {
                        if ir_stack[i + 4] == AddPtr(i_x) {
                            if let Some(ir) = test_inc_dec(ir_stack[i + 3]) {
                                generated_ir = MovingAdd(Direction::Left, i_x, ir.0, ir.1)
                            }
                        }
                    }
                    IncPtr => {
                        if ir_stack[i + 4] == DecPtr {
                            if let Some(ir) = test_inc_dec(ir_stack[i + 3]) {
                                generated_ir = MovingAdd(Direction::Right, 1, ir.0, ir.1)
                            }
                        }
                    }
                    DecPtr => {
                        if ir_stack[i + 4] == IncPtr {
                            if let Some(ir) = test_inc_dec(ir_stack[i + 3]) {
                                generated_ir = MovingAdd(Direction::Left, 1, ir.0, ir.1)
                            }
                        }
                    }
                    _ => {}
                }
            }
            //insert ir
            if generated_ir != Nop {
                ir_stack[i] = Nop;
                ir_stack[i + 1] = Nop;
                ir_stack[i + 2] = Nop;
                ir_stack[i + 3] = generated_ir;
                ir_stack[i + 4] = Nop;
                ir_stack[i + 5] = Nop;
            }
        }
    }

    ir_stack = ir_stack.iter().filter(|ir| **ir != Nop).copied().collect();
    if opt_level == OptLevel::LoopDataMove {
        return ir_stack;
    }
    //Jump optimizing
    let mut i_ptr = 0;

    while let Some(ir) = ir_stack.get(i_ptr) {
        if OptInstruction::LoopStart == *ir {
            let mut depth = 1;
            let saved_i_ptr = i_ptr;
            while depth != 0 {
                i_ptr += 1;
                if let Some(i) = ir_stack.get(i_ptr) {
                    match i {
                        LoopStart => depth += 1,
                        LoopEnd => depth -= 1,
                        _ => {}
                    }
                }
            }
            ir_stack[saved_i_ptr] = JZ(i_ptr);
            ir_stack[i_ptr] = Jnz(saved_i_ptr);
            i_ptr = saved_i_ptr;
        }
        i_ptr += 1;
    }

    ir_stack
}
pub fn exec_opt_ir<R: std::io::Read, W: std::io::Write>(
    program: Vec<OptInstruction>,
    mut reader: R,
    mut writer: W,
    debug: bool,
) {
    //VM
    let mut instruction_ptr = 0;
    let mut data_ptr: usize = 0;
    let mut mem = vec![0u8; 30000];
    loop {
        let op_code = program.get(instruction_ptr);
        if debug {
            println!(
                "i_ptr {} i {:?} d_ptr {} ",
                instruction_ptr, op_code, data_ptr
            );
        }
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
                Add(x) => mem[data_ptr] = mem[data_ptr].wrapping_add(*x),
                Sub(x) => mem[data_ptr] = mem[data_ptr].wrapping_sub(*x),
                AddPtr(x) => data_ptr = data_ptr.wrapping_add(*x),
                SubPtr(x) => data_ptr = data_ptr.wrapping_sub(*x),
                JZ(x) => {
                    if mem[data_ptr] == 0 {
                        instruction_ptr = *x;
                    }
                }
                Jnz(x) => {
                    if mem[data_ptr] != 0 {
                        instruction_ptr = *x;
                    }
                }
                ZeroClear => {
                    mem[data_ptr] = 0;
                }
                Nop => {}
                OptInstruction::PtrMoveRight(x) => {
                    while mem[data_ptr] != 0 {
                        data_ptr += *x;
                    }
                }
                OptInstruction::PtrMoveLeft(x) => {
                    while mem[data_ptr] != 0 {
                        data_ptr -= *x;
                    }
                }

                MovingAdd(direction, offset, sign, multiplier) => match direction {
                    Direction::Left => match sign {
                        Sign::Plus => {
                            mem[data_ptr - *offset] += mem[data_ptr].wrapping_mul(*multiplier);
                            mem[data_ptr] = 0;
                        }
                        Sign::Minus => {
                            mem[data_ptr - *offset] -= mem[data_ptr].wrapping_mul(*multiplier);
                            mem[data_ptr] = 0;
                        }
                    },
                    Direction::Right => match sign {
                        Sign::Plus => {
                            mem[data_ptr + *offset] += mem[data_ptr].wrapping_mul(*multiplier);
                            mem[data_ptr] = 0;
                        }
                        Sign::Minus => {
                            mem[data_ptr + *offset] -= mem[data_ptr].wrapping_mul(*multiplier);
                            mem[data_ptr] = 0;
                        }
                    },
                },
            }
            // if overflow detected dump all memory to std out
            // one line per 16 byte display
            if data_ptr >= mem.len() {
                for line in mem.chunks(16) {
                    println!("{:0x?}", line);
                }
                return;
            }
        } else {
            return;
        }
        instruction_ptr += 1;
    }
}
