#[derive(Debug, Eq, PartialEq, CLone)]
pub enum AsmOp{
    Add(Register, AsmOperand),
    Mov(Register, AsmOperand),
}

#[derive(Debug, Eq, PartialEq, CLone)]
pub enum AsmOperand{
    Register(Register),
    Value(i32),
}

#[derive(Debug, Eq, PartialEq, CLone)]
pub enum Register{
    RAX,
}
