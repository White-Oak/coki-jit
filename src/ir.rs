use grammar::*;
use asm_ops::*;
use asm_ops::AsmOperand::*;
use asm_ops::AsmOp::*;

struct AsmProgram {
    contents : Vec<AsmOp>
}

impl AsmProgram{
    fn add(&mut self, op: AsmOp){
        println!("{:?}", &op);
        self.contents.push(op);
    }

    fn new() -> AsmProgram {
        AsmProgram {contents: Vec::new()}
    }
}

impl Extend<AsmOp> for AsmProgram {
    fn extend<T: IntoIterator<Item=AsmOp>>(&mut self, iterable: T) {
        for elem in iterable {
            self.add(elem);
        }
    }
}

fn move_into_expr(expr: &Expr) -> Vec<AsmOp>{
let mut ops = Vec::new();
        match *expr {
            Expr::Num(ref num) => ops.push(Add(Register::RAX, Value(*num))),
            Expr::AddSub(ref terms)  => {
                ops.push(Push(RegisterOperand(Register::RAX)));
                ops.extend(add(terms.as_slice()));
                ops.push(Pop(RegisterOperand(Register::RBX)));
                ops.push(Pop(RegisterOperand(Register::RAX)));
                ops.push(Add(Register::RAX, RegisterOperand(Register::RBX)));
            },
            Expr::MultDiv(ref terms) => {
                ops.push(Push(RegisterOperand(Register::RAX)));
                ops.extend(mult(terms.as_slice()));
                ops.push(Pop(RegisterOperand(Register::RBX)));
                ops.push(Pop(RegisterOperand(Register::RAX)));
                ops.push(Mul(Register::RAX, RegisterOperand(Register::RBX)));
            }
            _ => panic!()
        }
    ops
}

fn move_into_first_term_expr(expr: &Expr) -> Vec<AsmOp>{
    let mut ops = Vec::new();
    match *expr {
        Expr::Num(ref num) => ops.push(Mov(Register::RAX, Value(*num))),
        Expr::AddSub(ref terms) => {
            ops.extend(add(terms.as_slice()));
            ops.push(Pop(RegisterOperand(Register::RAX)));
        },
        Expr::MultDiv(ref terms) => {
            ops.extend(mult(terms.as_slice()));
            ops.push(Pop(RegisterOperand(Register::RAX)));
        },
        _ => panic!()
    }
    ops
}

fn add(terms: &[AddTerm]) -> Vec<AsmOp>{
    let mut ops = Vec::new();
    let mut counter = 0;
    for term in terms {
        let &AddTerm(_, ref expr) = term;
        if counter == 0{
            ops.extend(move_into_first_term_expr(expr))
        } else {
            ops.extend(move_into_expr(expr));
        }
        counter +=1;
    }
    ops.push(Push(RegisterOperand(Register::RAX)));
    ops
}

fn mult(terms: &[MultTerm]) -> Vec<AsmOp>{
    let mut ops = Vec::new();
    let mut counter = 0;
    for term in terms {
        let &MultTerm(_, ref expr) = term;
        if counter == 0{
            ops.extend(move_into_first_term_expr(expr))
        } else {
            ops.extend(move_into_expr(expr));
        }
        counter +=1;
    }
    ops.push(Push(RegisterOperand(Register::RAX)));
    ops
}

pub fn translate(block: &Vec<Statement>) -> Box<Vec<AsmOp>>{
    let mut ops =  AsmProgram::new();
    for stmt in block {
        println!("\n{:?}\nIs translated into:", stmt);
        match *stmt {
            Statement::Assign(_, ref expr) => {
                match *expr {
                    Expr::AddSub(ref terms) => ops.extend(add(terms.as_slice())),
                    _ => panic!()
                }
            },
            _ => {}
        }
    }
    Box::new(ops.contents)
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
