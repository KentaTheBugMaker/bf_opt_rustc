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
    //other char
    OtherChar(char),
}
pub fn src_to_ir(code: &str) -> Vec<BFInstruction> {
    code.chars()
        .map(|x| match x {
            '>' => BFInstruction::IncPtr,
            '<' => BFInstruction::DecPtr,
            '+' => BFInstruction::Inc,
            '-' => BFInstruction::Dec,
            '.' => BFInstruction::Write,
            ',' => BFInstruction::Read,
            '[' => BFInstruction::LoopStart,
            ']' => BFInstruction::LoopEnd,
            ch => BFInstruction::OtherChar(ch),
        })
        .collect()
}
