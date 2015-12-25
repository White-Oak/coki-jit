use asm_ops::*;
use asm_ops::AsmOp::*;
use asm_ops::AsmOperand::*;

fn optimize_stmt(op: AsmOp, previous: AsmOp) -> (AsmOp, AsmOp){
    //fn remains() -> (AsmOp, AsmOp)
    match (&previous, &op){
        //From push rax; pop rbx
        //To mov rbx, rax
        (&Push(ref l), &Pop(ref r)) => {
            match (l , r) {
                (&Memory(_), &Memory(_)) =>{
                    //can't mov [], [] -- skip it on
                    (previous.clone(), op.clone())
                },
                (_, _) if l == r => (Nop, Nop),
                (_, _) => (Mov(r.clone(), l.clone()), Nop),
            }
        },
        (&Mov(ref before, _), &Push(ref after)) => {
            match (before, after) {
                (&RegisterOperand(ref r), &RegisterOperand(ref l)) if r == l => {
                     (Push(before.clone()), Nop)
                },
                (_, _) => (previous.clone(), op.clone())
            }
        }
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
