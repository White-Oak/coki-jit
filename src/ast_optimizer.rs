use coki_parser::*;
use coki_parser::Statement::*;
use coki_parser::Expr::*;
fn optimize_expr(expr: &Expr) -> Expr{
    match expr{
        &AddSub(ref _terms) => {
            let mut terms = Vec::new();
            let mut accumulator: i32 = 0;
            for term in _terms{
                let new_expr_for_term = optimize_expr(&term.1);
                if let Num(ref value) = new_expr_for_term {
                    match &term.0{
                        &AddOp::Add | &AddOp::Start => {accumulator += *value},
                        &AddOp::Subtract => {accumulator -= *value},
                    }
                } else {
                    terms.push(AddTerm(term.0.clone(), new_expr_for_term));
                }
            }
            if terms.len() == 0{
                Num(accumulator)
            }else{
                if accumulator != 0 {
                    terms.push(AddTerm(AddOp::Add, Num(accumulator)));
                }
                AddSub(terms)
            }
        },
        &MultDiv(ref _terms) => {
            let mut terms = Vec::new();
            let mut accumulator: i32 = 1;
            for term in _terms{
                let new_expr_for_term = optimize_expr(&term.1);
                if let Num(ref value) = new_expr_for_term {
                    match &term.0{
                        &MultOp::Multiply | &MultOp::Start => {accumulator *= *value},
                        &MultOp::Divide => {accumulator /= *value},
                        _ => {} //What to do with modulo??
                    }
                } else {
                    terms.push(MultTerm(term.0.clone(), new_expr_for_term));
                }
            }
            if terms.len() == 0{
                Num(accumulator)
            }else{
                if accumulator != 1 {
                    terms.push(MultTerm(MultOp::Multiply, Num(accumulator)));
                }
                MultDiv(terms)
            }
        },
        _ => expr.clone(),
    }
}

fn optimize_stmt(stmt: &Statement) -> Statement{
    match stmt{
        &Assign(ref name, ref expr) => {
            Assign(name.clone(), optimize_expr(expr))
        },
        _ => stmt.clone()
    }
}

pub fn optimize_ast(mut block: Vec<Statement>, iters: u8) -> Vec<Statement>{
    let mut changed = true;
    let mut iterations = 0;
    while changed && iterations < iters {
        iterations += 1;
        changed = false;
        let mut ops: Vec<Statement> = Vec::new();
        for stmt in block{
            ops.push(optimize_stmt(&stmt));
        }
        block = ops;
    }

    println!("It took {} iterations to optimize IR code.", iterations);
    block
}
