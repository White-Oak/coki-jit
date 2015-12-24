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

fn move_into_expr(expr: &Expr, var_store: &VarStore) -> Vec<AsmOp>{
    let mut ops = Vec::new();
    match *expr {
        Expr::Num(ref num) => ops.push(Add(Register::RAX, Value(*num))),
        Expr::AddSub(ref terms)  => {
            ops.push(Push(RegisterOperand(Register::RAX)));
            ops.extend(add(terms.as_slice(), var_store));
            ops.push(Pop(RegisterOperand(Register::RBX)));
            ops.push(Pop(RegisterOperand(Register::RAX)));
            ops.push(Add(Register::RAX, RegisterOperand(Register::RBX)));
        },
        Expr::MultDiv(ref terms) => {
            ops.push(Push(RegisterOperand(Register::RAX)));
            ops.extend(mult(terms.as_slice(), var_store));
            ops.push(Pop(RegisterOperand(Register::RBX)));
            ops.push(Pop(RegisterOperand(Register::RAX)));
            ops.push(Mul(Register::RAX, RegisterOperand(Register::RBX)));
        },
        Expr::Variable(ref name) => ops.push(Add(Register::RAX, Memory(var_store.get_var_address_r(name))))
    }
    ops
}

fn move_into_first_term_expr(expr: &Expr, var_store: &VarStore) -> Vec<AsmOp>{
    let mut ops = Vec::new();
    match *expr {
        Expr::Num(ref num) => ops.push(Mov(Register::RAX, Value(*num))),
        Expr::AddSub(ref terms) => {
            ops.extend(add(terms.as_slice(), var_store));
            ops.push(Pop(RegisterOperand(Register::RAX)));
        },
        Expr::MultDiv(ref terms) => {
            ops.extend(mult(terms.as_slice(), var_store));
            ops.push(Pop(RegisterOperand(Register::RAX)));
        },
        Expr::Variable(ref name) => ops.push(Mov(Register::RAX, Memory(var_store.get_var_address_r(name))))
    }
    ops
}

fn add(terms: &[AddTerm], var_store: &VarStore) -> Vec<AsmOp>{
    let mut ops = Vec::new();
    let mut counter = 0;
    for term in terms {
        let &AddTerm(_, ref expr) = term;
        if counter == 0{
            ops.extend(move_into_first_term_expr(expr, var_store))
        } else {
            ops.extend(move_into_expr(expr, var_store));
        }
        counter +=1;
    }
    ops.push(Push(RegisterOperand(Register::RAX)));
    ops
}

fn mult(terms: &[MultTerm], var_store: &VarStore) -> Vec<AsmOp>{
    let mut ops = Vec::new();
    let mut counter = 0;
    for term in terms {
        let &MultTerm(_, ref expr) = term;
        if counter == 0{
            ops.extend(move_into_first_term_expr(expr, var_store))
        } else {
            ops.extend(move_into_expr(expr, var_store));
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
        VarStore{variables: HashMap::new(), current_address: 0}
    }

    fn get_var_address_r(&self, name: &String) -> u16{
        match self.variables.get(name){
            Some(address) => *address,
            None => panic!("No variable named {}.", name)
        }
    }
    fn get_var_address_l(&mut self, name: &String) -> u16{
        if !self.variables.contains_key(name){
            let result = self.variables.insert(name.to_string(), self.current_address + 500);
            match result{
                Some(_) => panic!(),
                None =>{
                    self.current_address += 8;
                    self.get_var_address_r(name)
                }
            }
        }else{
            self.get_var_address_r( name)
        }
    }
}

fn calculate_expr(expr: &Expr, var_store: &VarStore) -> Vec<AsmOp>{
    let mut ops = Vec::new();
    match *expr {
        Expr::AddSub(ref terms) => ops.extend(add(terms.as_slice(), var_store)),
        Expr::MultDiv(ref terms) => ops.extend(mult(terms.as_slice(), var_store)),
        Expr::Num(ref num) => ops.push(Push(Value(*num))),
        Expr::Variable(ref name) => ops.push(Push(Memory(var_store.get_var_address_r(name)))),
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
                ops.extend(calculate_expr(expr, &var_store));
                ops.add(Pop(Memory(var_store.get_var_address_l(name))));
            }
            Statement::Output(ref expr) => {
                ops.extend(calculate_expr(expr, &var_store));
                ops.add(Pop(RegisterOperand(Register::RAX)));
            }
            _ => {}
        }
    }
    Box::new(ops.contents)
}
