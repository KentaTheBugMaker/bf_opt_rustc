use crate::optimizer::{Direction, OptInstruction, Sign};

pub fn emit_rust_code(opt_ir: &[OptInstruction]) -> String {
    let mut rust_code = include_str!("bf_init.rs_sub").to_owned();
    let mut nest = 1;
    for ir in opt_ir {
        rust_code.push_str(&*"\t".repeat(nest));
        let code_let = match ir {
            OptInstruction::Add(x) => {
                format!("mem[ptr] = mem[ptr].wrapping_add({});\n", x)
            }
            OptInstruction::Sub(x) => {
                format!("mem[ptr] = mem[ptr].wrapping_sub({});\n", x)
            }
            OptInstruction::AddPtr(x) => {
                format!("ptr = ptr.wrapping_add({});\n", x)
            }
            OptInstruction::SubPtr(x) => {
                format!("ptr = ptr.wrapping_sub({});\n", x)
            }
            OptInstruction::JZ(_) => {
                nest += 1;
                "while mem[ptr] != 0 {\n".to_owned()
            }
            OptInstruction::Jnz(_) => {
                nest -= 1;
                "}\n".to_owned()
            }
            OptInstruction::Read => {
                "reader.read_exact(&mut mem[ptr..ptr + 1]).unwrap();\n".to_owned()
            }
            OptInstruction::Write => "writer.write_all(&mem[ptr..ptr + 1]).unwrap();\n".to_owned(),
            OptInstruction::LoopStart => {
                nest += 1;
                "while mem[ptr] != 0 {\n".to_owned()
            }
            OptInstruction::LoopEnd => {
                nest -= 1;
                "}\n".to_owned()
            }
            OptInstruction::ZeroClear => "mem[ptr] = 0;\n".to_owned(),
            OptInstruction::MovingAdd(direction, offset, sign, multiplier) => match direction {
                Direction::Left => match sign {
                    Sign::Plus => {
                        format!(
                            "mem[ptr - {}] += mem[ptr].wrapping_mul({});\n",
                            offset, multiplier
                        )
                    }
                    Sign::Minus => {
                        format!(
                            "mem[ptr - {}] -= mem[ptr].wrapping_mul({});\n",
                            offset, multiplier
                        )
                    }
                },
                Direction::Right => match sign {
                    Sign::Plus => {
                        format!(
                            "mem[ptr + {}] += mem[ptr].wrapping_mul({});\n",
                            offset, multiplier
                        )
                    }
                    Sign::Minus => {
                        format!(
                            "mem[ptr + {}] -= mem[ptr].wrapping_mul({});\n",
                            offset, multiplier
                        )
                    }
                },
            },
            OptInstruction::PtrMoveRight(x) => {
                format!("while mem[ptr] != 0 {{ ptr += {};}}\n", x)
            }
            OptInstruction::PtrMoveLeft(x) => {
                format!("while mem[ptr] != 0 {{ ptr -= {};}}\n", x)
            }
            OptInstruction::Nop | OptInstruction::OtherChar(_) => "".to_owned(),
        };
        rust_code.push_str(&code_let);
    }
    rust_code.push('}');
    rust_code
}
