use asm_ops::*;

use std::error::Error;
use std::io::prelude::*;
use std::fs::File;
use std::path::Path;
use std::process::Command;

pub fn compile(ops: &Vec<AsmOp>) -> Vec<u8>{
    let mut str = "use64\n".to_string();
    for op in ops{
        match *op {
            AsmOp::Add(ref dest, ref operand) => match *operand{
                AsmOperand::RegisterOperand(ref source) => str = str + &format!("add {:?}, {:?}\n", dest, source),
                AsmOperand::Value(ref i) => str = str + &format!("add {:?}, {:?}\n", dest, i)
            },
            AsmOp::Mul(ref dest, ref operand) => match *operand{
                AsmOperand::RegisterOperand(ref source) => str = str + &format!("imul {:?}, {:?}\n", dest, source),
                AsmOperand::Value(ref i) => str = str + &format!("imul {:?}, {:?}\n", dest, i)
            },
            AsmOp::Pop(ref dest) => match *dest {
                AsmOperand::RegisterOperand(ref dest) => str = str + &format!("pop {:?}\n", dest),
                AsmOperand::Value(ref dest) => str = str + &format!("pop {:?}\n", dest)
            },
            AsmOp::Push(ref dest) => match *dest {
                AsmOperand::RegisterOperand(ref dest) => str = str + &format!("push {:?}\n", dest),
                AsmOperand::Value(ref dest) => str = str + &format!("push {:?}\n", dest)
            },
            AsmOp::Mov(ref dest, ref operand) => match *operand{
                AsmOperand::RegisterOperand(ref source) => str = str + &format!("mov {:?}, {:?}\n", dest, source),
                AsmOperand::Value(ref i) => str = str + &format!("mov {:?}, {:?}\n", dest, i)
            },
            // _ => {}
        }
    }
    str = str + "pop rax\n";
    println!("\nOutput assembly is:\n{}\n",str);

    write_asm(&str);
    assemble();
    read_bytes()
}

fn write_asm(str: &String) {
    let path = Path::new("target/temp.asm");
    let display = path.display();

    let mut file = match File::create(&path) {
        Err(why) => panic!("couldn't create {}: {}", display, Error::description(&why)),
        Ok(file) => file,
    };

    match file.write_all(str.as_bytes()) {
        Err(why) => {
            panic!("couldn't write to {}: {}", display, Error::description(&why))
        },
        Ok(_) => println!("Successfully wrote to {}", display),
    }
}

fn assemble() {
    let output = Command::new("fasm").arg("target/temp.asm").output().unwrap_or_else(|e| {
        panic!("failed to execute process: {}", e)
    });

    if !output.status.success() {
        println!("status: {}", output.status);
        println!("stdout: {}", String::from_utf8_lossy(&output.stdout));
        println!("stderr: {}", String::from_utf8_lossy(&output.stderr));
    } else {
        println!("Successfully assembled to temp.bin");
    }
}

fn read_bytes() -> Vec<u8>{
    let path = Path::new("target/temp.bin");
    let display = path.display();

    let mut file = match File::open(&path) {
       Err(why) => panic!("couldn't open {}: {}", display, Error::description(&why)),
       Ok(file) => file,
   };

    let mut contents: Vec<u8> = Vec::new();
    // Returns amount of bytes read and append the result to the buffer
    let result = file.read_to_end(&mut contents).unwrap();
    println!("Program consists of {} bytes", result);
    contents
}
