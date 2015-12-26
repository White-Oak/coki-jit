#[derive(Debug, Eq, PartialEq, Clone)]
pub enum AsmOp{
    Add(AsmOperand, AsmOperand),
    Sub(Register, AsmOperand),

    Mul(Register, AsmOperand),
    Div(Register, AsmOperand),
    Mod(Register, AsmOperand),

    Mov(AsmOperand, AsmOperand),
    Push(AsmOperand),
    Pop(AsmOperand),
    Out,

    Label(String),
    Loop(String),

    Nop
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub enum AsmOperand{
    RegisterOperand(Register),
    Value(i32),
    Memory(u16),
    MemoryRegister(Register),
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub enum Register{
    RAX,
    RBX,
    RCX,
    RDX,
    R8,
}
