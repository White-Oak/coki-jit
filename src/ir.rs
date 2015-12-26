use grammar::*;
use asm_ops::*;
use asm_ops::AsmOperand::*;
use asm_ops::AsmOp::*;
use asm_ops::Register::*;

struct AsmProgram {
    contents : Vec<AsmOp>
}

///A wrapper on Vec<AsmOp>
///Prints every addition for debugging purposes
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

///Indicates that a struct can be translated into IR code
trait AsmableExpression{
    fn get_ops(&self, var_store: &VarStore) -> Vec<AsmOp>;
}

impl AsmableExpression for Expr{
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

impl AsmableExpression for AddTerm{
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

impl AsmableExpression for MultTerm{
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

trait AsmableStatement{
    fn get_ops(&self, mut env: &mut Environment, mut program: &mut AsmProgram);
}

impl AsmableStatement for Statement{
    fn get_ops(&self, mut env: &mut Environment, mut program: &mut AsmProgram) {
        println!("\n{:?}\nIs translated into:", self);
        match *self {
            Statement::Assign(ref name, ref expr) => {
                program.extend(expr.get_ops(&env.var_store));
                program.add(Pop(Memory(env.var_store.get_var_address_l(name))));
            }
            Statement::Output(ref expr) => {
                program.extend(expr.get_ops(&env.var_store));
                program.add(Pop(Memory(env.out_store.get_next_output_adress())));
            },
            Statement::Loop(ref expr, ref block) =>{
                program.extend(expr.get_ops(&env.var_store));
                program.add(Pop(RegisterOperand(RCX)));

                let label = env.loopl_store.get_next_loop_label();
                program.add(Label(label.clone()));
                translate_stmts(&block.0, &mut env, &mut program);
                program.add(Loop(label));
            }
            _ => {}
        }
    }
}

use std::collections::HashMap;

struct VarStore{
    variables: HashMap<String, u16>,
    current_address: u16
}

///Stores addresses for variables
impl VarStore{
    fn new() -> VarStore{
        VarStore{variables: HashMap::new(), current_address: 500}
    }

    fn get_var_address_r(&self, name: &String) -> u16{
        match self.variables.get(name){
            Some(address) => *address,
            None => panic!("No variable named {}.", name)
        }
    }
    fn get_var_address_l(&mut self, name: &String) -> u16{
        if !self.variables.contains_key(name){
            let result = self.variables.insert(name.to_string(), self.current_address);
            match result{
                Some(_) => panic!(),
                None =>{
                    self.current_address += 8;
                    self.get_var_address_r(name)
                }
            }
        }else{
            self.get_var_address_r(name)
        }
    }
}

struct OutputStore(u16);

///Stores adresses for output values
impl OutputStore{
    fn get_next_output_adress(&mut self) -> u16{
        let address = self.0;
        self.0 += 8;
        address
    }
    fn new() -> OutputStore{
        OutputStore(1000)
    }
}

struct LoopLabelStore(u16);

impl LoopLabelStore{
    fn new() -> LoopLabelStore{
        LoopLabelStore(0)
    }
    fn get_next_loop_label(&mut self) -> String{
        let num = self.0;
        self.0 += 1;
        format!("label{}", num).to_string()
    }
}

struct Environment{
    var_store: VarStore,
    out_store: OutputStore,
    loopl_store: LoopLabelStore
}

impl Environment{
    fn new() -> Environment{
        Environment{var_store: VarStore::new(), out_store: OutputStore::new(), loopl_store: LoopLabelStore::new()}
    }
}

fn translate_stmts(block: &Vec<Statement>, mut env: &mut Environment, mut program: &mut AsmProgram){
    for stmt in block {
        stmt.get_ops(&mut env, &mut program);
    }
}
///Translates AST into a sequence of asm instructions
pub fn translate(block: &Vec<Statement>) -> Box<Vec<AsmOp>>{
    let mut ops =  AsmProgram::new();
    let mut env = Environment::new();
    translate_stmts(block, &mut env, &mut ops);
    Box::new(ops.contents)
}
