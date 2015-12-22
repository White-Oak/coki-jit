use asm_ops::*;

pub fn compile(ops: &Vec<AsmOp>){
    for op in ops{
        match *op {
            AsmOp::Add(ref dest, ref operand) => match *operand{
                AsmOperand::Register(ref source) => println!("add {:?}, {:?}", dest, source),
                AsmOperand::Value(ref i) => println!("add {:?}, {:?}", dest, i)
            },
            _ => {}
        }
    }
}
