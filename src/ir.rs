use grammar::*;

fn mult(terms: &[MultTerm], mut ops: &mut Vec<AsmOp>){
    for term in terms {
        let &MultTerm(_, ref expr) = term;
        match *expr {
            Expr::Num(ref num) =>  {
                let op = AsmOp::Add(Register::RAX, AsmOperand::Value(*num));
                println!("{:?}", op);
                ops.push(op)
            },
            _ => panic!()
        }
    }
}

fn add(terms: &[AddTerm], mut ops: &mut Vec<AsmOp>){
    for term in terms {
        let &AddTerm(_, ref expr) = term;
        match *expr {
            Expr::MultDiv(ref terms) => mult(terms.as_slice(), ops),
            _ => panic!()
        }
    }
}

pub fn translate(block: &Vec<Statement>) -> Box<Vec<AsmOp>>{
    let mut ops =  Vec::new();
    for stmt in block {
        println!("\n{:?}\nIs translated into:", stmt);
        match *stmt {
            Statement::Assign(_, ref expr) => {
                match *expr {
                    Expr::AddSub(ref terms) => add(terms.as_slice(), &mut ops),
                    _ => panic!()
                }
            },
            _ => {}
        }
    }
    Box::new(ops)
}
/*
Assign("a",
AddSub(
[AddTerm(Add, MultDiv([MultTerm(Multiply, Num(1))])),
AddTerm(Add, MultDiv([MultTerm(Multiply, Num(3))])),
AddTerm(Add, MultDiv([MultTerm(Multiply, Num(1))]))]
)
)*/
/*
mov rax, 1
add rax, 3
add rax, 1
ret
*/
