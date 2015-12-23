#[derive(Debug, Eq, PartialEq, CLone)]
pub enum AsmOp{
    Add(Register, AsmOperand),
    Mul(Register, AsmOperand),
    Mov(Register, AsmOperand),
    Push(AsmOperand),
    Pop(AsmOperand),
}

#[derive(Debug, Eq, PartialEq, CLone)]
pub enum AsmOperand{
    RegisterOperand(Register),
    Value(i32),
}

#[derive(Debug, Eq, PartialEq, CLone)]
pub enum Register{
    RAX,
    RBX,
}
