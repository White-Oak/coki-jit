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

fn move_into_expr<'a, F: FnOnce()>(fun: F) -> Vec<AsmOp>{
    let mut ops = Vec::new();
    ops.push(Push(RegisterOperand(Register::RAX)));
    fun();
    ops.push(Pop(RegisterOperand(Register::RBX)));
    ops.push(Pop(RegisterOperand(Register::RAX)));
    ops.push(Mul(Register::RAX, RegisterOperand(Register::RBX)));
    ops
}
fn mult<'a>(terms: &[MultTerm], mut ops: &'a mut AsmProgram){
    let mut counter = 0;
    for term in terms {
        let &MultTerm(_, ref expr) = term;
        if counter == 0{
            match *expr {
                Expr::Num(ref num) => ops.add(Mov(Register::RAX, Value(*num))),
                _ => panic!()
            }
        } else {
            match *expr {
                Expr::Num(ref num) => ops.add(Mul(Register::RAX, Value(*num))),
                Expr::AddSub(ref terms)  => {
                    let _ops = move_into_expr(|| add(terms.as_slice(), ops));
                    ops.extend(_ops)
                },
                Expr::MultDiv(ref terms) => {
                    let _ops = move_into_expr(|| mult(terms.as_slice(), ops));
                    ops.extend(_ops)
                }
                _ => panic!()
            }
        }
        counter +=1;
    }
    ops.add(Push(RegisterOperand(Register::RAX)));
}

fn add<'a>(terms: &[AddTerm], mut ops: &'a mut AsmProgram){
    let mut counter = 0;
    for term in terms {
        let &AddTerm(_, ref expr) = term;
        if counter == 0{
            match *expr {
                Expr::Num(ref num) => ops.add(Mov(Register::RAX, Value(*num))),
                Expr::AddSub(ref terms) => {
                    add(terms.as_slice(), ops);
                    ops.add(Pop(RegisterOperand(Register::RAX)));
                },
                Expr::MultDiv(ref terms) => {
                    mult(terms.as_slice(), ops);
                    ops.add(Pop(RegisterOperand(Register::RAX)));
                },
                _ => panic!()
            }
        } else {
            match *expr {
                Expr::Num(ref num) => ops.add(Add(Register::RAX, Value(*num))),
                Expr::AddSub(ref terms)  => {
                    ops.add(Push(RegisterOperand(Register::RAX)));
                    add(terms.as_slice(), ops);
                    ops.add(Pop(RegisterOperand(Register::RBX)));
                    ops.add(Pop(RegisterOperand(Register::RAX)));
                    ops.add(Add(Register::RAX, RegisterOperand(Register::RBX)));
                },
                Expr::MultDiv(ref terms) => {
                    ops.add(Push(RegisterOperand(Register::RAX)));
                    mult(terms.as_slice(), ops);
                    ops.add(Pop(RegisterOperand(Register::RBX)));
                    ops.add(Pop(RegisterOperand(Register::RAX)));
                    ops.add(Add(Register::RAX, RegisterOperand(Register::RBX)));
                }
                _ => panic!()
            }
        }
        counter +=1;
    }
    ops.add(Push(RegisterOperand(Register::RAX)));
}

pub fn translate(block: &Vec<Statement>) -> Box<Vec<AsmOp>>{
    let mut ops =  AsmProgram::new();
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
