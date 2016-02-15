use asm_ops::*;
use coki_jitter::jit::{OUTPUT_OFFSET, PRINT_FUNCTION};

use std::error::Error;
use std::io::prelude::*;
use std::fs::File;
use std::path::Path;
use std::process::Command;

use std::fmt;
impl fmt::Display for Register {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

pub fn compile(ops: &Vec<AsmOp>) -> Vec<u8> {
    let mut string = format!(r"use64
lea r8, [rip]
sub r8, 7
add r8, {}
include 'PROC64.INC'
print =  {:?}
",
                             OUTPUT_OFFSET,
                             PRINT_FUNCTION);
    for op in ops {
        let temp_str = match op {
            &AsmOp::Add(ref dest, ref operand) => {
                match (dest, operand) {
                    (&AsmOperand::Memory(_), &AsmOperand::Value(_)) => {
                        format!("add {}, dword {}\n", dest, operand)
                    }
                    _ => format!("add {}, {}\n", dest, operand),
                }
            }
            &AsmOp::Sub(ref dest, ref operand) => format!("sub {}, {}\n", dest, operand),

            &AsmOp::Mul(ref dest, ref operand) => format!("imul {}, {}\n", dest, operand),
            &AsmOp::Div(_, _) => panic!(),
            &AsmOp::Mod(_, _) => panic!(),

            &AsmOp::Pop(ref dest) => format!("popq {}\n", dest),
            &AsmOp::Push(ref dest) => format!("pushq {}\n", dest),
            &AsmOp::Mov(ref dest, ref operand) => {
                match (dest, operand) {
                    (&AsmOperand::Memory(_), &AsmOperand::Value(_)) => {
                        format!("mov {}, dword {}\n", dest, operand)
                    }
                    (&AsmOperand::MemoryRegister(_), &AsmOperand::Value(_)) => {
                        format!("mov {}, dword {}\n", dest, operand)
                    }
                    _ => format!("mov {}, {}\n", dest, operand),
                }
            }
            &AsmOp::Label(ref name) => format!("{}:\n", name),
            &AsmOp::Loop(ref name) => format!("\rloopq {}\n", name),
            _ => format!("{}", op),
        };
        string = string + &temp_str;
    }
    println!("\nOutput assembly is:\n{}\n", string);
    write_asm(&string);
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
            panic!("couldn't write to {}: {}",
                   display,
                   Error::description(&why))
        }
        Ok(_) => println!("Successfully wrote to {}", display),
    }
}

#[cfg(windows)]
const FASM: &'static str = "./fasm.exe";

#[cfg(unix)]
const FASM: &'static str = "./fasm";

fn assemble() {
    let output = Command::new(FASM)
                     .arg("target/temp.asm")
                     .output()
                     .unwrap_or_else(|e| panic!("failed to execute process: {}", e));

    if !output.status.success() {
        println!("status: {}", output.status);
        println!("stdout: {}", String::from_utf8_lossy(&output.stdout));
        println!("stderr: {}", String::from_utf8_lossy(&output.stderr));
        panic!("Can't assemble!");
    } else {
        println!("Successfully assembled to temp.bin");
    }
}

pub fn read_bytes() -> Vec<u8> {
    let path = Path::new("target/temp.bin");
    // let path = Path::new("test.bin");
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
