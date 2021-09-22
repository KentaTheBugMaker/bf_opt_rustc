#![feature(test)]
use crate::optimizer::OptLevel;
use std::time::Instant;
extern crate  test;
mod interpreter;
mod optimizer;

fn main() {
    let reader = std::io::stdin();
    let writer = std::io::stdout();
    let code = include_str!("../factor.b");
    let mut vm = interpreter::Interpreter::load_program(code, reader, writer);

    let opt_ir_inc_dec2 = optimizer::optimize(&vm.get_program(), OptLevel::IncDecOpt2);
    let opt_ir_zero_clear=optimizer::optimize(&vm.get_program(),OptLevel::ZeroClear);
    let writer = std::io::stdout();
    let instant=Instant::now();
    let bench_data:Vec<u8>=include_bytes!("../bench_number.txt").to_vec();
    optimizer::exec_opt_ir(opt_ir_inc_dec2, bench_data.as_slice(), writer);
    let inc_dec_opt_2 =instant.elapsed();
    println!("inc_dec_opt2 interpreter elapsed {:?}", inc_dec_opt_2);
    let writer = std::io::stdout();
    let instant=Instant::now();
    optimizer::exec_opt_ir(opt_ir_zero_clear, bench_data.as_slice(), writer);
    let zero_clear_opt=instant.elapsed();
    println!("zero_clear_opt interpreter elapsed {:?}",zero_clear_opt);
}

#[cfg(test)]
mod tests{

    use crate::{optimizer, interpreter};
    use crate::optimizer::OptLevel;
    use test::Bencher;

    #[bench]
    fn test_zero_clear_opt(b:&mut Bencher){
        let reader = std::io::stdin();
        let writer = std::io::stdout();
        let code = include_str!("../factor.b");
        let mut vm = interpreter::Interpreter::load_program(code, reader, writer);
        let bench_data:Vec<u8>=include_bytes!("../bench_number.txt").to_vec();
        let opt_ir_zero_clear=optimizer::optimize(&vm.get_program(),OptLevel::ZeroClear);
        let mut v=vec![0u8;1024];
        b.iter(||{
            optimizer::exec_opt_ir(opt_ir_zero_clear.clone(), bench_data.as_slice(), v.as_mut_slice());
        })
    }
    #[bench]
    fn test_inc_dec_opt(b:&mut Bencher){
        let reader = std::io::stdin();
        let writer = std::io::stdout();
        let code = include_str!("../factor.b");
        let mut vm = interpreter::Interpreter::load_program(code, reader, writer);
        let bench_data:Vec<u8>=include_bytes!("../bench_number.txt").to_vec();
        let opt_ir_zero_clear=optimizer::optimize(&vm.get_program(),OptLevel::IncDecOpt2);
        let mut v=vec![0u8;1024];
        b.iter(||{
            optimizer::exec_opt_ir(opt_ir_zero_clear.clone(), bench_data.as_slice(), v.as_mut_slice());
        })
    }
}