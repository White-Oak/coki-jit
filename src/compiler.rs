use asm_ops::*;
use coki_jitter::jit::OUTPUT_OFFSET;

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
impl fmt::Display for AsmOperand {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            AsmOperand::RegisterOperand(ref dest) => write!(f, "{}", dest),
            AsmOperand::Value(ref dest) => write!(f, "{}", dest),
            AsmOperand::Memory(ref mem) => write!(f, "[{}]", mem),
            AsmOperand::MemoryRegister(ref mem) => write!(f, "[{}]", mem),
        }
    }
}

pub unsafe extern "fastcall" fn do_it(a: &u64){
    use libc::{c_void, write};
    let ptr: *const c_void = a as *const _ as *const c_void;
    write(1, ptr, 8);
}

extern crate winapi;
extern crate kernel32;
pub fn compile(ops: &Vec<AsmOp>) -> Vec<u8> {
    let mut str = format!("use64\nlea r8, [rip]\nsub r8, 7\nadd r8, {}\n",
                          OUTPUT_OFFSET);
    let mut block_counter = 0;
    for op in ops {
        for _ in 0..block_counter {
            str = str + &"  ";
        }
        let temp_str = match *op {
            AsmOp::Add(ref dest, ref operand) => {
                match (dest, operand) {
                    (&AsmOperand::Memory(_), &AsmOperand::Value(_)) => {
                        format!("add {}, dword {}\n", dest, operand)
                    }
                    _ => format!("add {}, {}\n", dest, operand),
                }
            }
            AsmOp::Sub(ref dest, ref operand) => format!("sub {}, {}\n", dest, operand),

            AsmOp::Mul(ref dest, ref operand) => format!("imul {}, {}\n", dest, operand),
            AsmOp::Div(_, _) => panic!(),
            AsmOp::Mod(_, _) => panic!(),

            AsmOp::Pop(ref dest) => format!("popq {}\n", dest),
            AsmOp::Push(ref dest) => format!("pushq {}\n", dest),
            AsmOp::Mov(ref dest, ref operand) => {
                match (dest, operand) {
                    (&AsmOperand::Memory(_), &AsmOperand::Value(_)) => {
                        format!("mov {}, dword {}\n", dest, operand)
                    }
                    _ => format!("mov {}, {}\n", dest, operand),
                }
            }
            AsmOp::Out => "ret".to_string(),

            AsmOp::Label(ref name) => {
                block_counter += 1;
                format!("{}:\n", name)
            }
            AsmOp::Loop(ref name) => {
                block_counter -= 1;
                format!("\rloopq {}\n", name)
            }
            _ => "\r".to_string(),
            // _ => {}
        };
        str = str + &temp_str;
    }
    println!("\nOutput assembly is:\n{}\n", str);
    unsafe {
        let hmod = kernel32::GetModuleHandleA("kernel32.dll".as_ptr() as *const i8);
        let getstd = kernel32::GetProcAddress(hmod, "GetStdHandle".as_ptr()  as *const i8);
        let write = kernel32::GetProcAddress(hmod, "WriteConsoleA".as_ptr()  as *const i8);
        let fun_ptr = do_it as *const u8;
        let string = format!(r"use64
        lea r10, [rip]
        sub r10, 7
        add r10, 2000
        include 'proc64.inc'


        mov [r10], byte 50
        mov [r10 + 1], byte 50
        mov r11, {:?}
        fastcall r11, r10
    ", fun_ptr);
        // let string = format!(r"use64
        // lea r10, [rip]
        // sub r10, 7
        // add r10, 2000
        // include 'include/win64a.inc'
        //
        // mov [r10], byte 50
        // mov [r10 + 1], byte 50
        //
        // mov r11, {:?}
        // fastcall r11, -11
        //
        // mov r11, {:?}
        // fastcall r11, rax, r10, 2, 0, 0
        // ret
        // ", getstd, write );
        println!("{}", string);
        write_asm(&string);
        assemble();
        read_bytes()
    }
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
