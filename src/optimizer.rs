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
                (&Memory(_), &Memory(_)) => (previous.clone(), op.clone()), //can't mov [], [] -- skip it on
                _ if l == r => (Nop, Nop), // push rax; pop rax
                _ => (Mov(r.clone(), l.clone()), Nop),
            }
        },
        //From mov rax, ... ; push rax
        //To push ...
        (&Mov(ref before, ref needed), &Push(ref after)) if before == after => match before {
            &Memory(_) => (previous.clone(), op.clone()),
            _ => (Push(needed.clone()), Nop)
        },
        (_,_) => (previous.clone(), op.clone())
    }
}

pub fn optimize(mut _ops: Vec<AsmOp>, iters: u8) -> Vec<AsmOp>{
    let mut changed = true;
    let mut iterations = 0;
    while changed && iterations < iters {
        iterations += 1;
        changed = false;
        let mut ops: Vec<AsmOp> = Vec::new();
        let mut previous = _ops.remove(0);
        for op in _ops{
            let (optimized, pre) = optimize_stmt(op.clone(), previous);
            ops.push(optimized);
            previous = pre;
            if previous == Nop{
                changed = true;
            }
        }
        ops.push(previous);
        ops.retain(|ref i|**i != Nop);
        _ops = ops;
    }

    println!("It took {} iterations to optimize IR code.", iterations);
    _ops
}
