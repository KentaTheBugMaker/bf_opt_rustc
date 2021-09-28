#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub enum BFInstruction {
    //ptr++;
    IncPtr,
    //ptr--;
    DecPtr,
    // *ptr++;
    Inc,
    // *ptr--;
    Dec,
    //I/O
    Read,
    Write,
    //ControlFlow
    LoopStart,
    LoopEnd,
}
pub fn src_to_ir(code: &str) -> Vec<BFInstruction> {
    code.chars()
        .filter(|x| matches!(x, '>' | '<' | '+' | '-' | '.' | ',' | '[' | ']'))
        .map(|x| match x {
            '>' => BFInstruction::IncPtr,
            '<' => BFInstruction::DecPtr,
            '+' => BFInstruction::Inc,
            '-' => BFInstruction::Dec,
            '.' => BFInstruction::Write,
            ',' => BFInstruction::Read,
            '[' => BFInstruction::LoopStart,
            ']' => BFInstruction::LoopEnd,
            _ => unreachable!(),
        })
        .collect()
}
