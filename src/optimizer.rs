use asm_ops::*;
fn optimize_stmt(op: AsmOp, previous: AsmOp) -> (AsmOp, AsmOp){
    match (&op, &previous){
        //From pop rax; push rbx
        //To mov rax, rbx
        (&AsmOp::Pop(ref l), &AsmOp::Push(ref r)) => {
            match (l , r) {
                (&AsmOperand::Memory(_), &AsmOperand::Memory(_)) =>{
                    //can't mov [], [] -- skip it on
                    (previous.clone(), op.clone())
                },
                (_, _) => {
                    (AsmOp::Mov(l.clone(), r.clone()), AsmOp::Nop)
                }
            }
        },
        (_,_) => (previous.clone(), op.clone())
    }
}
pub fn optimize(mut _ops: Vec<AsmOp>) -> Vec<AsmOp>{
    let mut ops: Vec<AsmOp> = Vec::new();
    let mut previous = _ops.remove(0);
    for op in _ops{
        let (optimized, pre) = optimize_stmt(op.clone(), previous);
        ops.push(optimized);
        previous = pre;
    }
    ops.push(previous);
    ops
}
