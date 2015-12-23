use asm_ops::*;

use std::error::Error;
use std::io::prelude::*;
use std::fs::File;
use std::path::Path;

pub fn compile(ops: &Vec<AsmOp>){
    let mut str = "use64\nmov RAX, 1\n".to_string();
    for op in ops{
        match *op {
            AsmOp::Add(ref dest, ref operand) => match *operand{
                AsmOperand::Register(ref source) => str = str + &format!("add {:?}, {:?}\n", dest, source),
                AsmOperand::Value(ref i) => str = str + &format!("add {:?}, {:?}\n", dest, i)
            },
            _ => {}
        }
    }
    println!("\nOutput assembly is:\n{}",str);
}
