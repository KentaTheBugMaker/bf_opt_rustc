//! brainfuck code formatter
use crate::optimizer::{pass_inc_dec_opt, OptInstruction};
use crate::parser::src_to_ir;

pub fn brain_fuck_fmt(source: &str) -> String {
    let mut nest = 0;
    let bf_ir = src_to_ir(source);
    let grouped_ir = pass_inc_dec_opt(&bf_ir);
    let mut bf_code = String::new();
    let mut pop_tab = false;
    for ir in grouped_ir.iter() {
        bf_code.push_str(&("\t".repeat(nest)));
        let code_let = match ir {
            OptInstruction::Add(x) => "+".repeat(*x as usize),
            OptInstruction::Sub(x) => "-".repeat(*x as usize),
            OptInstruction::AddPtr(x) => ">".repeat(*x),
            OptInstruction::SubPtr(x) => "<".repeat(*x),
            OptInstruction::Read => ",".to_owned(),
            OptInstruction::Write => ".".to_owned(),
            OptInstruction::LoopStart => {
                nest += 1;
                "[".to_owned()
            }
            OptInstruction::LoopEnd => {
                nest -= 1;
                pop_tab = true;
                "]".to_owned()
            }
            OptInstruction::OtherChar(_) => "".to_owned(),
            unreachable => {
                unreachable!("pass give me not supported instruction : {:?}", unreachable)
            }
        };
        if pop_tab {
            bf_code.pop();
        }
        bf_code.push_str(&code_let);
        bf_code.push('\n');
        pop_tab = false;
    }
    bf_code
}
