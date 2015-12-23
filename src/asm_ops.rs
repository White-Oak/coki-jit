#[derive(Debug, Eq, PartialEq)]
pub enum AsmOp{
    Add(Register, AsmOperand),
    Mul(Register, AsmOperand),
    Mov(Register, AsmOperand),
    Push(AsmOperand),
    Pop(AsmOperand),
    Out,
}

#[derive(Debug, Eq, PartialEq)]
pub enum AsmOperand{
    RegisterOperand(Register),
    Value(i32),
    Memory(u16),
}

#[derive(Debug, Eq, PartialEq,)]
pub enum Register{
    RAX,
    RBX,
}
