#![feature(test)]
//! Optimizing Brain f**k interpreter
//! wrap around cell
//! 8 bit cell
//!
use crate::optimizer::OptLevel;
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
const CODE: &'static str = include_str!("../factor.b");
fn main() {
    let bf_ir = src_to_ir(CODE);
    //let formatted_code = formatter::brain_fuck_fmt(CODE);
    //println!("{}", formatted_code);

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

    let inc_dec_opt = optimizer::pass_inc_dec_opt(&bf_ir);
    let instant = Instant::now();
    optimizer::exec_opt_ir(
        inc_dec_opt.clone(),
        BENCH_DATA.deref().as_slice(),
        std::io::stdout(),
        false,
    );
    println!("++ -- >> << Inc Dec optimization {:?}", instant.elapsed());

    let zero_clear_opt = optimizer::pass_zero_clear(inc_dec_opt);
    let zero_clear_opt = optimizer::pass_nop_remove(zero_clear_opt);
    let instant = Instant::now();
    optimizer::exec_opt_ir(
        zero_clear_opt.clone(),
        BENCH_DATA.deref().as_slice(),
        std::io::stdout(),
        false,
    );
    println!("[-] 0 clear idiom optimization {:?}", instant.elapsed());

    let ptr_move_opt = optimizer::pass_ptr_move(zero_clear_opt);
    let ptr_move_opt = optimizer::pass_nop_remove(ptr_move_opt);
    let instant = Instant::now();
    optimizer::exec_opt_ir(
        ptr_move_opt.clone(),
        BENCH_DATA.deref().as_slice(),
        std::io::stdout(),
        false,
    );
    println!(
        "[>>] 0 cell search by constant stride optimization {:?}",
        instant.elapsed()
    );

    let data_move_opt = optimizer::pass_generic_data_move(ptr_move_opt.clone());
    let data_move_opt = optimizer::pass_nop_remove(data_move_opt);
    /*
    for (i, (ir, un_optimized)) in data_move_opt.iter().zip(ptr_move_opt.iter()).enumerate() {
        println!("{} : {:?} {:?}", i, ir, un_optimized);
    }*/
    let instant = Instant::now();
    optimizer::exec_opt_ir(
        data_move_opt.clone(),
        BENCH_DATA.deref().as_slice(),
        std::io::stdout(),
        false,
    );
    println!(
        "[->+<] data moving add optimization {:?}",
        instant.elapsed()
    );
    let rust_code = bf2rustc::emit_rust_code(&data_move_opt);
    println!("{}", rust_code);
    let jump_calc = optimizer::pass_jump_calc(data_move_opt);
    let instant = Instant::now();
    optimizer::exec_opt_ir(
        jump_calc.clone(),
        BENCH_DATA.deref().as_slice(),
        std::io::stdout(),
        false,
    );
    println!(
        " [ ] compile time jump address calculation {:?}",
        instant.elapsed()
    );
    let instant = Instant::now();
    optimized_rust_code_factor::bf_main(BENCH_DATA.deref().as_slice(), std::io::stdout());
    println!("native code from data moving {:?}", instant.elapsed());
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
