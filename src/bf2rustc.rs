use crate::optimizer::{Direction, OptInstruction, Sign};

pub fn emit_rust_code(opt_ir: &[OptInstruction]) -> String {
    let mut rust_code = include_str!("bf_init.rs_sub").to_owned();
    let mut nest = 1;
    for ir in opt_ir {
        rust_code.push_str(&*"\t".repeat(nest));
        let code_let = match ir {
            OptInstruction::Add(x) => {
                format!("mem[data_ptr] = mem[data_ptr].wrapping_add({});\n", x)
            }
            OptInstruction::Sub(x) => {
                format!("mem[data_ptr] = mem[data_ptr].wrapping_sub({});\n", x)
            }
            OptInstruction::AddPtr(x) => {
                format!("data_ptr = data_ptr.wrapping_add({});\n", x)
            }
            OptInstruction::SubPtr(x) => {
                format!("data_ptr = data_ptr.wrapping_sub({});\n", x)
            }
            OptInstruction::JZ(_) => {
                nest += 1;
                "while mem[data_ptr] != 0 {\n".to_owned()
            }
            OptInstruction::Jnz(_) => {
                nest -= 1;
                "}\n".to_owned()
            }
            OptInstruction::Read => {
                "reader.read_exact(&mut mem[data_ptr..data_ptr + 1]).unwrap();\n".to_owned()
            }
            OptInstruction::Write => {
                "writer.write_all(&mem[data_ptr..data_ptr + 1]).unwrap();\n".to_owned()
            }
            OptInstruction::LoopStart => {
                nest += 1;
                "while mem[data_ptr]!=0{\n".to_owned()
            }
            OptInstruction::LoopEnd => {
                nest -= 1;
                "}\n".to_owned()
            }
            OptInstruction::Inc => "mem[data_ptr] = mem[data_ptr].wrapping_add(1);\n".to_owned(),
            OptInstruction::Dec => "mem[data_ptr] = mem[data_ptr].wrapping_sub(1);\n".to_owned(),
            OptInstruction::IncPtr => "data_ptr = data_ptr.wrapping_add(1);\n".to_owned(),
            OptInstruction::DecPtr => "data_ptr = data_ptr.wrapping_sub(1);\n".to_owned(),
            OptInstruction::ZeroClear => "mem[data_ptr] = 0;\n".to_owned(),
            OptInstruction::MovingAdd(direction, offset, sign, multiplier) => match direction {
                Direction::Left => match sign {
                    Sign::Plus => {
                        let code = format!(
                            "mem[data_ptr - {}] += mem[data_ptr].wrapping_mul({});\n",
                            offset, multiplier
                        );
                        rust_code.push_str(&code);
                        let code = format!("{}mem[data_ptr] = 0;\n", "\t".repeat(nest));
                        rust_code.push_str(&code);
                        "".to_owned()
                    }
                    Sign::Minus => {
                        let code = format!(
                            "mem[data_ptr - {}] -= mem[data_ptr].wrapping_mul({});\n",
                            offset, multiplier
                        );
                        rust_code.push_str(&code);
                        let code = format!("{}mem[data_ptr] = 0;\n", "\t".repeat(nest));
                        rust_code.push_str(&code);
                        "".to_owned()
                    }
                },
                Direction::Right => match sign {
                    Sign::Plus => {
                        let code = format!(
                            "mem[data_ptr + {}] += mem[data_ptr].wrapping_mul({});\n",
                            offset, multiplier
                        );
                        rust_code.push_str(&code);
                        let code = format!("{}mem[data_ptr] = 0;\n", "\t".repeat(nest));
                        rust_code.push_str(&code);
                        "".to_owned()
                    }
                    Sign::Minus => {
                        let code = format!(
                            "mem[data_ptr + {}] -= mem[data_ptr].wrapping_mul({});\n",
                            offset, multiplier
                        );
                        rust_code.push_str(&code);
                        let code = format!("{}mem[data_ptr] = 0;\n", "\t".repeat(nest));
                        rust_code.push_str(&code);
                        "".to_owned()
                    }
                },
            },
            OptInstruction::PtrMoveRight(x) => {
                format!("while mem[data_ptr] != 0 {{ data_ptr += {};}}\n", x)
            }
            OptInstruction::PtrMoveLeft(x) => {
                format!("while mem[data_ptr] != 0 {{ data_ptr -= {};}}\n", x)
            }
            OptInstruction::Nop => "".to_owned(),
        };
        rust_code.push_str(&code_let);
    }
    rust_code.push('}');
    rust_code
}
