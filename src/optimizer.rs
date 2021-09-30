use crate::optimizer::OptInstruction::{
    Add, AddPtr, Jnz, LoopEnd, LoopStart, MovingMultiplyAdd, Nop, OtherChar, Read, Sub, SubPtr,
    Write, ZeroClear, JZ,
};
use crate::parser::BFInstruction;
use std::option::Option::Some;

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
    //[-]
    ZeroClear,
    //[-<+>] [-<->] [->+<] [->-<]
    //direction offset sign multiplier
    MovingMultiplyAdd(Direction, usize, Sign, u8),
    MovingAdd(Direction, usize, Sign),
    //[AddPtr]
    PtrMoveRight(usize),
    //[SubPtr]
    PtrMoveLeft(usize),
    //dummy instruction inserted to eliminated instruction
    Nop,
    //comment
    OtherChar(char),
}
#[derive(Eq, PartialEq)]
pub enum OptLevel {
    IncDecOpt1,
    ZeroClear,
    JumpOpt,
    LoopPtrMove,
    LoopDataMove,
}

/// > < + - optimization
pub fn pass_inc_dec_opt(program: &[BFInstruction]) -> Vec<OptInstruction> {
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
    ir_stack
}
/// optimize \[-] \[+]
pub fn pass_zero_clear(mut ir_stack: Vec<OptInstruction>) -> Vec<OptInstruction> {
    //opt [-] [+]
    if ir_stack.len() > 2 {
        for i in 0..(ir_stack.len() - 2) {
            if ir_stack[i] == LoopStart && ir_stack[i + 1] == Sub(1) && ir_stack[i + 2] == LoopEnd {
                ir_stack[i] = ZeroClear;
                ir_stack[i + 1] = Nop;
                ir_stack[i + 2] = Nop;
            }
            if ir_stack[i] == LoopStart && ir_stack[i + 1] == Add(1) && ir_stack[i + 2] == LoopEnd {
                ir_stack[i] = ZeroClear;
                ir_stack[i + 1] = Nop;
                ir_stack[i + 2] = Nop;
            }
        }
    }
    ir_stack
}
///optimize \[>] \[<]
pub fn pass_ptr_move(mut ir_stack: Vec<OptInstruction>) -> Vec<OptInstruction> {
    if ir_stack.len() > 2 {
        for i in 0..(ir_stack.len() - 2) {
            if ir_stack[i] == LoopStart && ir_stack[i + 2] == LoopEnd {
                match ir_stack[i + 1] {
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
    ir_stack
}
/// optimize data move.
///
/// target instruction stream
///
/// * LS Sub(1) (AddPtr(x) Add|Sub(const))* SubPtr(sum(x)) LE
/// * LS Sub(1) (SubPtr(x) Add|Sub(const))* AddPtr(sum(x)) LE   
/// * LS (AddPtr(x) Add|Sub(const))* SubPtr(sum(x)) Sub(1) LE (not implemented yet)
/// * LS (SubPtr(x) Add|Sub(const))* AddPtr(sum(x)) Sub(1) LE (not implemented yet)
pub fn pass_generic_data_move(mut ir_stack: Vec<OptInstruction>) -> Vec<OptInstruction> {
    let mut i_ptr = 0;
    let mut code_lets = Vec::new();
    let mut replaces = Vec::new();
    let mut ptr_offsets = 0;
    let mut direction = None;
    while let Some(start) = ir_stack.get(i_ptr) {
        // [
        if *start == OptInstruction::LoopStart {
            let saved_pc = i_ptr;
            let next = ir_stack[i_ptr + 1];
            //println!("{} : {:?}", i_ptr + 1, next);
            //-
            if next == OptInstruction::Sub(1) {
                loop {
                    let pair = (ir_stack[i_ptr + 2], ir_stack[i_ptr + 3]);
                    //println!("{},{} {:?}", i_ptr + 2, i_ptr + 3, pair);
                    match pair {
                        (AddPtr(offset), Add(x)) => {
                            ptr_offsets += offset;
                            direction.replace(Direction::Right);
                            code_lets.push(MovingMultiplyAdd(
                                Direction::Right,
                                ptr_offsets,
                                Sign::Plus,
                                x,
                            ));
                        }
                        (AddPtr(offset), Sub(x)) => {
                            ptr_offsets += offset;
                            direction.replace(Direction::Right);
                            code_lets.push(MovingMultiplyAdd(
                                Direction::Right,
                                ptr_offsets,
                                Sign::Minus,
                                x,
                            ));
                        }
                        (SubPtr(offset), Add(x)) => {
                            ptr_offsets += offset;
                            direction.replace(Direction::Left);
                            code_lets.push(MovingMultiplyAdd(
                                Direction::Left,
                                ptr_offsets,
                                Sign::Plus,
                                x,
                            ));
                        }
                        (SubPtr(offset), Sub(x)) => {
                            ptr_offsets += offset;
                            direction.replace(Direction::Left);
                            code_lets.push(MovingMultiplyAdd(
                                Direction::Left,
                                ptr_offsets,
                                Sign::Minus,
                                x,
                            ));
                        }
                        //end loop
                        (SubPtr(offset), LoopEnd) => {
                            if let Some(Direction::Right) = direction {
                                if offset == ptr_offsets {
                                    replaces.push((saved_pc..i_ptr + 3, code_lets.clone()));
                                    //println!("emit {}..{} {:?}", saved_pc, i_ptr + 3, code_lets);
                                    i_ptr += 2;
                                    ptr_offsets = 0;
                                    code_lets.clear();
                                    break;
                                }
                            }
                        }
                        (AddPtr(offset), LoopEnd) => {
                            if let Some(Direction::Left) = direction {
                                if offset == ptr_offsets {
                                    replaces.push((saved_pc..i_ptr + 3, code_lets.clone()));
                                    //println!("emit {}..{} {:?}", saved_pc, i_ptr + 3, code_lets);
                                    i_ptr += 2;
                                    ptr_offsets = 0;
                                    code_lets.clear();
                                    break;
                                }
                            }
                        }
                        _ => {
                            code_lets.clear();
                            i_ptr += 2;
                            ptr_offsets = 0;
                            break;
                        }
                    }
                    i_ptr += 2;
                }
            } else {
                i_ptr += 1;
            }
        } else {
            i_ptr += 1;
        }
    }

    for (range, mut instructions) in replaces {
        let code_len = instructions.len();
        let saved_pc = range.start;
        let end = range.end;
        /*
        println!(
            "{}..{} : replace {:?}->{:?}",
            saved_pc,
            end,
            &ir_stack[saved_pc..(end + 1)],
            instructions
        );*/
        //insert moving add instruction
        /*
        for i in saved_pc..(saved_pc + code_len) {
            ir_stack[i] = instructions.remove(0);
        }*/
        for ir in ir_stack.iter_mut().skip(saved_pc).take(code_len){
            *ir=instructions.remove(0);
        }
        for ir in ir_stack.iter_mut().take(end).skip(saved_pc+code_len){
            *ir=Nop;
        }
        /*
        for i in (saved_pc + code_len)..(end) {
            ir_stack[i] = OptInstruction::Nop;
        }*/
        // insert zero clear instruction
        ir_stack[end] = OptInstruction::ZeroClear;
    }
    ir_stack
}
///remove unneeded multiply
pub fn pass_moving_add_specialization(ir_stack: Vec<OptInstruction>) -> Vec<OptInstruction> {
    ir_stack
        .iter()
        .map(|ir| {
            if let OptInstruction::MovingMultiplyAdd(direction, offset, sign, 1) = ir {
                OptInstruction::MovingAdd(*direction, *offset, *sign)
            } else {
                *ir
            }
        })
        .collect()
}
/// this eliminate ZeroClear by following conditions
/// PtrMoveLeft()->ZeroClear
/// PtrMoveRight()->ZeroClear
/// ZeroClear->ZeroClear
pub fn pass_remove_already_cleared(mut ir_stack:Vec<OptInstruction>)->Vec<OptInstruction>{
    let mut replaces=vec![];
    for i in 0..(ir_stack.len()-1){
        let prev_i=ir_stack[i];
        let instruction=ir_stack[i+1];
            if (prev_i == instruction
                && prev_i==OptInstruction::ZeroClear ) || matches!(prev_i,OptInstruction::PtrMoveLeft(_)|OptInstruction::PtrMoveRight(_)) {
                replaces.push(i+1);
            }
    }
    for i in replaces{
        ir_stack[i]=OptInstruction::Nop;
    }
    ir_stack
}
/// pointer propagation (not implemented)
/// this observe pointer fixed location
pub fn pass_pointer_propagation(ir_stack: Vec<OptInstruction>) -> Vec<OptInstruction> {
    //initialize d_ptr
    let mut current_d_ptr:Option<isize>=Some(0);
    //
    let mut is_in_loop =false;
    for (i,ir) in ir_stack.iter().enumerate(){
        println!("i_ptr {} ir {:?} d_ptr {:?}",i,ir,current_d_ptr);
        match *ir {
            Add(_) => {}
            Sub(_) => {}
            AddPtr(x) => {
                if let Some(ref mut  ptr)= current_d_ptr{
                    *ptr+=x as isize;
                }
                }
            SubPtr(x) => {
                if let Some(ref mut  ptr)= current_d_ptr{
                    *ptr-=x as isize;
                }
            }
            JZ(_) => {is_in_loop=true;}
            Jnz(_) => {is_in_loop=false;}
            LoopStart => {is_in_loop=true;}
            LoopEnd => {is_in_loop=false;}
            //these ir does not affect to pointer is fixed value
            Read => {}
            Write => {}
            ZeroClear => {}
            MovingMultiplyAdd(_, _, _, _) => {}
            OptInstruction::MovingAdd(_, _, _) => {}
            // these ir affect to pointer
            OptInstruction::PtrMoveRight(_) => {current_d_ptr=None;}
            OptInstruction::PtrMoveLeft(_) => {current_d_ptr=None;}
            _=>{}
        }
    }
    ir_stack
}
/// remove nop
pub fn pass_nop_remove(ir_stack: Vec<OptInstruction>) -> Vec<OptInstruction> {
    ir_stack
        .iter()
        .filter(|ir| **ir != OptInstruction::Nop)
        .copied()
        .collect()
}
/// [ ] -> JZ JNZ pair this is the finalizer pass
pub fn pass_jump_calc(mut ir_stack: Vec<OptInstruction>) -> Vec<OptInstruction> {
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
pub fn optimize(program: &[BFInstruction], opt_level: OptLevel) -> Vec<OptInstruction> {
    let mut ir_stack = pass_inc_dec_opt(program);
    if opt_level == OptLevel::IncDecOpt1 {
        return ir_stack;
    }

    //opt [-] [+]
    ir_stack = pass_zero_clear(ir_stack);
    if opt_level == OptLevel::ZeroClear {
        return ir_stack;
    }
    // opt [>]
    ir_stack = pass_ptr_move(ir_stack);
    if opt_level == OptLevel::LoopPtrMove {
        return ir_stack;
    }
    // opt [->+<] [-<->] [->-<] [-<+>]
    ir_stack = pass_generic_data_move(ir_stack);
    if opt_level == OptLevel::LoopDataMove {
        return ir_stack;
    }
    //Jump optimizing
    ir_stack = pass_jump_calc(ir_stack);
    if opt_level == OptLevel::JumpOpt {
        return ir_stack;
    }

    // pointer propagation
    //let mut d_ptr=0;

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
                "i_ptr {} i {:?} d_ptr {} cell_value {}",
                instruction_ptr, op_code, data_ptr,mem[data_ptr]
            );
        }
        if let Some(op_code) = op_code {
            match op_code {
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
                Nop | OtherChar(_) => {}
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

                MovingMultiplyAdd(direction, offset, sign, multiplier) => match direction {
                    Direction::Left => match sign {
                        Sign::Plus => {
                            mem[data_ptr - *offset] += mem[data_ptr].wrapping_mul(*multiplier);
                        }
                        Sign::Minus => {
                            mem[data_ptr - *offset] -= mem[data_ptr].wrapping_mul(*multiplier);
                        }
                    },
                    Direction::Right => match sign {
                        Sign::Plus => {
                            mem[data_ptr + *offset] += mem[data_ptr].wrapping_mul(*multiplier);
                        }
                        Sign::Minus => {
                            mem[data_ptr + *offset] -= mem[data_ptr].wrapping_mul(*multiplier);
                        }
                    },
                },
                OptInstruction::MovingAdd(direction, offset, sign) => match direction {
                    Direction::Left => match sign {
                        Sign::Plus => {
                            mem[data_ptr - *offset] += mem[data_ptr];
                        }
                        Sign::Minus => {
                            mem[data_ptr - *offset] -= mem[data_ptr];
                        }
                    },
                    Direction::Right => match sign {
                        Sign::Plus => {
                            mem[data_ptr + *offset] += mem[data_ptr];
                        }
                        Sign::Minus => {
                            mem[data_ptr + *offset] -= mem[data_ptr];
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
