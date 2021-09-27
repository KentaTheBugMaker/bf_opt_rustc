#![feature(test)]
use crate::optimizer::OptLevel;
use crate::parser::src_to_ir;
use std::time::Instant;
extern crate test;
mod interpreter;
mod optimizer;
mod parser;

fn main() {
    let code = include_str!("../factor.b");
    let bf_ir = src_to_ir(code);
    let opt_ir_inc_dec2 = optimizer::optimize(&bf_ir, OptLevel::IncDecOpt2);
    let opt_ir_zero_clear = optimizer::optimize(&bf_ir, OptLevel::ZeroClear);
    let opt_ir_move_ptr=optimizer::optimize(&bf_ir,OptLevel::LoopPtrMove);
    let opt_ir_data_move=optimizer::optimize(&bf_ir,OptLevel::LoopDataMove);
    let writer = std::io::stdout();
    let instant = Instant::now();
    let bench_data: Vec<u8> = include_bytes!("../bench_number.txt").to_vec();
    optimizer::exec_opt_ir(opt_ir_inc_dec2, bench_data.as_slice(), writer,false);
    let inc_dec_opt_2 = instant.elapsed();
    println!("inc_dec_opt2 interpreter elapsed {:?}", inc_dec_opt_2);
    let writer = std::io::stdout();
    let instant = Instant::now();
    optimizer::exec_opt_ir(opt_ir_zero_clear, bench_data.as_slice(), writer,false);
    let zero_clear_opt = instant.elapsed();
    println!("zero_clear_opt interpreter elapsed {:?}", zero_clear_opt);
    let writer = std::io::stdout();
    let instant = Instant::now();
    optimizer::exec_opt_ir(opt_ir_move_ptr, bench_data.as_slice(), writer,false);
    let move_ptr_opt = instant.elapsed();
    println!("move_ptr interpreter elapsed {:?}", move_ptr_opt);
    let writer = std::io::stdout();
    let instant = Instant::now();
    optimizer::exec_opt_ir(opt_ir_data_move, bench_data.as_slice(), writer,false);
    let move_data_opt = instant.elapsed();
    println!("move_data interpreter elapsed {:?}", move_data_opt);
}

#[cfg(test)]
mod tests {

    use crate::optimizer::OptLevel;
    use crate::parser::src_to_ir;
    use crate::{interpreter, optimizer};
    use test::Bencher;

    #[bench]
    fn test_zero_clear_opt(b: &mut Bencher) {
        let code = include_str!("../factor.b");
        let bf_ir = src_to_ir(code);
        let bench_data: Vec<u8> = include_bytes!("../bench_number.txt").to_vec();
        let opt_ir_zero_clear = optimizer::optimize(&bf_ir, OptLevel::ZeroClear);
        let mut v = vec![0u8; 1024];
        b.iter(|| {
            optimizer::exec_opt_ir(
                opt_ir_zero_clear.clone(),
                bench_data.as_slice(),
                v.as_mut_slice(),
                false
            );
        })
    }
    #[bench]
    fn test_move_ptr_opt(b: &mut Bencher) {
        let code = include_str!("../factor.b");
        let bf_ir = src_to_ir(code);
        let bench_data: Vec<u8> = include_bytes!("../bench_number.txt").to_vec();
        let opt_ir_zero_clear = optimizer::optimize(&bf_ir, OptLevel::LoopPtrMove);
        let mut v = vec![0u8; 1024];
        b.iter(|| {
            optimizer::exec_opt_ir(
                opt_ir_zero_clear.clone(),
                bench_data.as_slice(),
                v.as_mut_slice(),
                false
            );
        })
    }
    #[bench]
    fn test_inc_dec_opt(b: &mut Bencher) {
        let code = include_str!("../factor.b");
        let bf_ir = src_to_ir(code);
        let bench_data: Vec<u8> = include_bytes!("../bench_number.txt").to_vec();
        let opt_ir_zero_clear = optimizer::optimize(&bf_ir, OptLevel::IncDecOpt2);
        let mut v = vec![0u8; 1024];
        b.iter(|| {
            optimizer::exec_opt_ir(
                opt_ir_zero_clear.clone(),
                bench_data.as_slice(),
                v.as_mut_slice(),
                false
            );
        })
    }
}
