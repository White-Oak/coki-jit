use asm_ops::*;
use asm_ops::AsmOp::*;
use asm_ops::AsmOperand::*;
macro_rules! enum_shuffle(
    (($l:expr, $r:expr) when ($sp1:pat , $sp2:pat) -> ($success:expr)
    $(or when ($sp_or:pat) $(if ($cond:expr))* -> ($failure:expr))*) =>(
        match ($l, $r) {
            ($sp1, $sp1) | ($sp1, $sp2) | ($sp2, $sp1) | ($sp2, $sp2) => $success,
            $($sp_or $(if $cond)* => $failure,)*
        }
    )
);

fn optimize_stmt(op: AsmOp, previous: AsmOp) -> (AsmOp, AsmOp) {
    // fn remains() -> (AsmOp, AsmOp)
    match (&previous, &op) {
        // From push rax; pop rbx
        // To mov rbx, rax
        //
        // can't mov [], [] -- skip it on
        // push rax; pop rax
        // finally, if it fits, transform into mov r, l
        (&Push(ref l), &Pop(ref r)) => {
            enum_shuffle!((l, r)
                when (&Memory(_), &MemoryRegister(_)) -> ((previous.clone(), op.clone())) or
                when (_) if (l == r) -> ((Nop, Nop)) or
                when (_) -> ((Mov(r.clone(), l.clone()), Nop))
            )
        }
        // From mov rax, ... ; push rax
        // To push ...
        (&Mov(ref before, ref needed), &Push(ref after)) if before == after => {
            match *before {
                Memory(_) => (previous.clone(), op.clone()),
                _ => (Push(needed.clone()), Nop),
            }
        }
        (_, _) => (previous.clone(), op.clone()),
    }
}

pub fn optimize(mut res_ops: Vec<AsmOp>, iters: u8) -> Vec<AsmOp> {
    let mut changed = true;
    let mut iterations = 0;
    while changed && iterations < iters {
        iterations += 1;
        changed = false;
        let mut ops: Vec<AsmOp> = Vec::new();
        let mut previous = res_ops.remove(0);
        for op in res_ops {
            let (optimized, pre) = optimize_stmt(op.clone(), previous);
            ops.push(optimized);
            previous = pre;
            if previous == Nop {
                changed = true;
            }
        }
        ops.push(previous);
        ops.retain(|ref i| **i != Nop);
        res_ops = ops;
    }

    println!("It took {} iterations to optimize IR code.", iterations);
    res_ops
}
