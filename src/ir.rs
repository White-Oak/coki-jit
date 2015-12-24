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

trait Asmable{
    fn get_ops(&self, var_store: &VarStore) -> Vec<AsmOp>;
}

impl Asmable for Expr{
    fn get_ops(&self, var_store: &VarStore) -> Vec<AsmOp>{
        let mut ops = Vec::new();
        fn add(terms: &[AddTerm], var_store: &VarStore) -> Vec<AsmOp>{
            let mut ops = Vec::new();
            for term in terms {
                ops.extend(term.get_ops(var_store));
            }
            ops.push(Push(RegisterOperand(Register::RAX)));
            ops
        }
        fn mult(terms: &[MultTerm], var_store: &VarStore) -> Vec<AsmOp>{
            let mut ops = Vec::new();
            for term in terms {
                ops.extend(term.get_ops(var_store));
            }
            ops.push(Push(RegisterOperand(Register::RAX)));
            ops
        }
        match *self {
            Expr::AddSub(ref terms) => ops.extend(add(terms.as_slice(), var_store)),
            Expr::MultDiv(ref terms) => ops.extend(mult(terms.as_slice(), var_store)),
            Expr::Num(ref num) => ops.push(Push(Value(*num))),
            Expr::Variable(ref name) => ops.push(Push(Memory(var_store.get_var_address_r(name)))),
        }
        ops
    }
}

impl Asmable for AddTerm{
    fn get_ops(&self, var_store: &VarStore) -> Vec<AsmOp>{
        let &AddTerm(ref op, ref expr) = self;
        let mut ops = Vec::new();
        match *op{
            AddOp::Start => {
                ops.extend(expr.get_ops(var_store));
                ops.push(Pop(RegisterOperand(Register::RAX)));
            },
            _ => {
                ops.push(Push(RegisterOperand(Register::RAX)));
                ops.extend(expr.get_ops(var_store));
                ops.push(Pop(RegisterOperand(Register::RBX)));
                ops.push(Pop(RegisterOperand(Register::RAX)));
                match *op{
                    AddOp::Add => ops.push(Add(Register::RAX, RegisterOperand(Register::RBX))),
                    AddOp::Subtract => ops.push(Sub(Register::RAX, RegisterOperand(Register::RBX))),
                    _ => panic!()
                }
            }
        }
        ops
    }
}

impl Asmable for MultTerm{
    fn get_ops(&self, var_store: &VarStore) -> Vec<AsmOp>{
        let &MultTerm(ref op, ref expr) = self;
        let mut ops = Vec::new();
        match *op{
            MultOp::Start => {
                ops.extend(expr.get_ops(var_store));
                ops.push(Pop(RegisterOperand(Register::RAX)));
            },
            _ => {
                ops.push(Push(RegisterOperand(Register::RAX)));
                ops.extend(expr.get_ops(var_store));
                ops.push(Pop(RegisterOperand(Register::RBX)));
                ops.push(Pop(RegisterOperand(Register::RAX)));
                match *op{
                    MultOp::Multiply => ops.push(Mul(Register::RAX, RegisterOperand(Register::RBX))),
                    MultOp::Divide => ops.push(Div(Register::RAX, RegisterOperand(Register::RBX))),
                    MultOp::Modulo => ops.push(Mod(Register::RAX, RegisterOperand(Register::RBX))),
                    _ => panic!()
                }
            }
        }
        ops
    }
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

pub fn translate(block: &Vec<Statement>) -> Box<Vec<AsmOp>>{
    let mut ops =  AsmProgram::new();
    let mut var_store = VarStore::new();
    for stmt in block {
        println!("\n{:?}\nIs translated into:", stmt);
        match *stmt {
            Statement::Assign(ref name, ref expr) => {
                ops.extend(expr.get_ops(&var_store));
                ops.add(Pop(Memory(var_store.get_var_address_l(name))));
            }
            Statement::Output(ref expr) => {
                ops.extend(expr.get_ops(&var_store));
                ops.add(Pop(RegisterOperand(Register::RAX)));
            }
            _ => {}
        }
    }
    Box::new(ops.contents)
}
