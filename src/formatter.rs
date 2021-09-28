//! brainfuck code formatter
use crate::optimizer::{pass_inc_dec_opt, OptInstruction};
use crate::parser::{src_to_ir, BFInstruction};

pub fn brain_fuck_fmt(source: &str) -> String {
    let mut nest = 0;
    let mut formatted_code = String::new();
    let bf_ir = src_to_ir(source);
    let grouped_ir = pass_inc_dec_opt(&bf_ir);
    let mut i_ptr = 0;
    while let Some(i) = grouped_ir.get(i_ptr) {
        formatted_code.push_str(&*"\t".repeat(nest));
        match i {
            OptInstruction::Read => formatted_code.push(','),
            OptInstruction::Write => formatted_code.push('.'),
            OptInstruction::LoopStart => {
                nest += 1;
                formatted_code.push('[');
            }
            OptInstruction::LoopEnd => {
                nest -= 1;
                formatted_code.push(']');
            }
            OptInstruction::OtherChar(ch) => {
                let mut comment = String::from(*ch);
                while let Some(ir) = bf_ir.get(i_ptr) {
                    if let BFInstruction::OtherChar(ch) = ir {
                        comment.push(*ch);
                        i_ptr += 1;
                        // new line
                        if ch == '\n' {
                            formatted_code.push_str(&comment);
                            break;
                        }
                    }
                }
                continue;
            }
            OptInstruction::Add(x) => {
                formatted_code.push_str(&"+".repeat(*x as usize));
            }
            OptInstruction::Sub(x) => {
                formatted_code.push_str(&"-".repeat(*x as usize));
            }
            OptInstruction::AddPtr(x) => {
                formatted_code.push_str(&">".repeat(*x));
            }
            OptInstruction::SubPtr(x) => {
                formatted_code.push_str(&"<".repeat(*x));
            }
            unreachable => {
                unreachable!("optimizer doesn't emit this kind ir {}", unreachable)
            }
        }
        formatted_code.push('\n');
        i_ptr += 1;
    }

    formatted_code
}
