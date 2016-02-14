#[derive(Debug, Eq, PartialEq, Clone)]
pub enum AsmOp {
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

    Nop,
}

use std::fmt::{Display, Formatter, Result};
impl Display for AsmOp{
    fn fmt(&self, f: &mut Formatter) -> Result {
        use self::AsmOp::*;
        match self {
            &Out => write!(f,
r"push r8
mov r10, print
fastcall r10, qword [r8]
pop r8
"),
            _ => unimplemented!()
        }
    }
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub enum AsmOperand {
    RegisterOperand(Register),
    Value(i32),
    Memory(u16),
    MemoryRegister(Register),
}

impl Display for AsmOperand {
    fn fmt(&self, f: &mut Formatter) -> Result {
        use self::AsmOperand::*;
        match self {
            &RegisterOperand(ref dest) => write!(f, "{}", dest),
            &Value(ref dest) => write!(f, "{}", dest),
            &Memory(ref mem) => write!(f, "[{}]", mem),
            &MemoryRegister(ref mem) => write!(f, "[{}]", mem),
        }
    }
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub enum Register {
    RAX,
    RBX,
    RCX,
    RDX,
    R8,
    R15
}
