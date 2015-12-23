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

use std::collections::HashMap;

struct VarStore{
    variables: HashMap<String, u16>,
    current_address: u16
}

impl VarStore{
    fn new() -> VarStore{
        VarStore{variables: HashMap::new(), current_address: 1000}
    }

    fn get_var_address(&mut self, name: &String) -> u16{
        let result = self.variables.insert(name.clone(), self.current_address);
        match result{
            Some(address) => address,
            None =>{
                self.current_address += 1;
                self.current_address - 1
            }
        }
    }
}

fn calculate_expr(expr: &Expr, var_store: &mut VarStore) -> Vec<AsmOp>{
    let mut ops = Vec::new();
    match *expr {
        Expr::AddSub(ref terms) => ops.extend(add(terms.as_slice())),
        Expr::MultDiv(ref terms) => ops.extend(mult(terms.as_slice())),
        Expr::Num(ref num) => ops.push(Push(Value(*num))),
        Expr::Variable(ref name) => ops.push(Push(Memory(var_store.get_var_address(name)))),
    }
    ops
}

pub fn translate(block: &Vec<Statement>) -> Box<Vec<AsmOp>>{
    let mut ops =  AsmProgram::new();
    let mut var_store = VarStore::new();
    for stmt in block {
        println!("\n{:?}\nIs translated into:", stmt);
        match *stmt {
            Statement::Assign(ref name, ref expr) => {
                ops.extend(calculate_expr(expr, &mut var_store));
                ops.add(Pop(Memory(var_store.get_var_address(name))));
            }
            Statement::Output(ref expr) => {
                ops.extend(calculate_expr(expr, &mut var_store));
                ops.add(Pop(RegisterOperand(Register::RAX)));
            }
            _ => {}
        }
    }
    Box::new(ops.contents)
}
