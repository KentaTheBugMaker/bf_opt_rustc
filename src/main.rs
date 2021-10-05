#![feature(test)]
//! Optimizing Brain f**k interpreter
//! wrap around cell
//! 8 bit cell
//!
use crate::optimizer::OptInstruction;
use crate::parser::src_to_ir;
use std::ops::Deref;
use std::time::Instant;

extern crate test;
mod bf2rustc;
mod formatter;
mod interpreter;
mod optimized_rust_code_factor;
mod optimizer;
mod parser;

static BENCH_DATA: once_cell::sync::Lazy<Vec<u8>> = once_cell::sync::Lazy::new(|| {
    let mut bench_data = include_str!("../bench_number.txt").to_ascii_lowercase();
    bench_data = bench_data.replace("\r\n", "\n");
    bench_data.into_bytes()
});
const CODE: &str = include_str!("../factor.b");
fn estimate_performance(opt_ir: Vec<OptInstruction>, message: &str) {
    let instant = Instant::now();
    optimizer::exec_opt_ir(
        opt_ir,
        BENCH_DATA.deref().as_slice(),
        std::io::stdout(),
        false,
    );
    println!("{} {:?}", message, instant.elapsed());
}
fn main() {
    let bf_ir = src_to_ir(CODE);
    //let formatted_code = formatter::brain_fuck_fmt(CODE);
    //println!("{}", formatted_code);
    /*
        let mut vm = interpreter::Interpreter::load_program(
            bf_ir.clone(),
            BENCH_DATA.deref().as_slice(),
            std::io::stdout(),
        );
        let instant = Instant::now();
        vm.exec_program();
        println!(
            "Non optimized reference interpreter {:?}",
            instant.elapsed()
        );
    */

    let inc_dec_opt = optimizer::pass_inc_dec_opt(&bf_ir);

    estimate_performance(inc_dec_opt.clone(), "++ -- >> << Inc Dec optimization");

    let zero_clear_opt = optimizer::pass_zero_clear(inc_dec_opt);
    let zero_clear_opt = optimizer::pass_nop_remove(zero_clear_opt);

    estimate_performance(zero_clear_opt.clone(), "[-] zero clear optimization");

    let ptr_move_opt = optimizer::pass_ptr_move(zero_clear_opt);
    let ptr_move_opt = optimizer::pass_nop_remove(ptr_move_opt);

    estimate_performance(ptr_move_opt.clone(), "[>] seek to 0 cell by given stride ");

    let data_move_opt = optimizer::pass_generic_data_move(ptr_move_opt);
    let data_move_opt = optimizer::pass_nop_remove(data_move_opt);

    estimate_performance(data_move_opt.clone(), "[-<++>] data move opt");

    let moving_add_opt = optimizer::pass_moving_add_specialization(data_move_opt);
    let moving_add_opt = optimizer::pass_nop_remove(moving_add_opt);

    estimate_performance(
        moving_add_opt.clone(),
        "[-<+>] specialize data move opt when multiplier = 1",
    );
    //let rust_code = bf2rustc::emit_rust_code(&zero_clear_opt);
    //println!("{}", rust_code);
    let jump_calc = optimizer::pass_jump_calc(moving_add_opt);

    estimate_performance(jump_calc.clone(), "compile time jump address calculation");
    let set_after_set = optimizer::pass_remove_already_cleared(jump_calc);

    let set_after_set = optimizer::pass_nop_remove(set_after_set);
    let set_after_set = optimizer::pass_recalculate_jump_address(set_after_set);

    estimate_performance(set_after_set.clone(), "set after set ");

    let add_after_set=optimizer::pass_add_after_set(set_after_set);
    let add_after_set=optimizer::pass_nop_remove(add_after_set);
    let add_after_set=optimizer::pass_recalculate_jump_address(add_after_set);



    estimate_performance(add_after_set.clone(),"add after set");
    // while if specialization
    //let un_optimize=optimizer::pass_un_optimize_jump_address(add_after_set);
    //let while_to_if=optimizer::pass_specialize_while_to_if(un_optimize);
    //let while_to_if =optimizer::pass_nop_remove(while_to_if);

    //let rust_code = bf2rustc::emit_rust_code(&while_to_if);
    //println!("{}", rust_code);

    let instant = Instant::now();
    optimized_rust_code_factor::bf_main(BENCH_DATA.deref().as_slice(), std::io::stdout());
    println!("native code from if specialization {:?}", instant.elapsed());
}

#[cfg(test)]
mod tests {

    use crate::optimizer::OptLevel;
    use crate::parser::src_to_ir;
    use crate::{optimized_rust_code_factor, optimizer, BENCH_DATA, CODE};
    use std::ops::Deref;
    use test::Bencher;

    #[bench]
    fn test_inc_dec_opt(b: &mut Bencher) {
        let bf_ir = src_to_ir(CODE);
        let opt_ir_zero_clear = optimizer::optimize(&bf_ir, OptLevel::IncDecOpt1);
        b.iter(|| {
            optimizer::exec_opt_ir(
                opt_ir_zero_clear.clone(),
                BENCH_DATA.deref().as_slice(),
                std::io::stdout(),
                false,
            );
        })
    }

    #[bench]
    fn test_zero_clear_opt(b: &mut Bencher) {
        let bf_ir = src_to_ir(CODE);
        let opt_ir_zero_clear = optimizer::optimize(&bf_ir, OptLevel::ZeroClear);
        b.iter(|| {
            optimizer::exec_opt_ir(
                opt_ir_zero_clear.clone(),
                BENCH_DATA.deref().as_slice(),
                std::io::stdout(),
                false,
            );
        })
    }
    #[bench]
    fn test_move_ptr_opt(b: &mut Bencher) {
        let bf_ir = src_to_ir(CODE);
        let opt_ir_zero_clear = optimizer::optimize(&bf_ir, OptLevel::LoopPtrMove);
        b.iter(|| {
            optimizer::exec_opt_ir(
                opt_ir_zero_clear.clone(),
                BENCH_DATA.deref().as_slice(),
                std::io::stdout(),
                false,
            );
        })
    }
    #[bench]
    fn test_move_data_opt(b: &mut Bencher) {
        let bf_ir = src_to_ir(CODE);
        let opt_ir_zero_clear = optimizer::optimize(&bf_ir, OptLevel::LoopDataMove);
        b.iter(|| {
            optimizer::exec_opt_ir(
                opt_ir_zero_clear.clone(),
                BENCH_DATA.deref().as_slice(),
                std::io::stdout(),
                false,
            );
        })
    }
    #[bench]
    fn test_jump_opt(b: &mut Bencher) {
        let bf_ir = src_to_ir(CODE);
        let opt_ir_zero_clear = optimizer::optimize(&bf_ir, OptLevel::JumpOpt);
        b.iter(|| {
            optimizer::exec_opt_ir(
                opt_ir_zero_clear.clone(),
                BENCH_DATA.deref().as_slice(),
                std::io::stdout(),
                false,
            );
        })
    }
    #[bench]
    fn test_native_code(b: &mut Bencher) {
        b.iter(|| {
            optimized_rust_code_factor::bf_main(BENCH_DATA.deref().as_slice(), std::io::stdout());
        })
    }
}
