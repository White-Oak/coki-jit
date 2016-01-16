use asm_ops::*;
use jitter::jit::OUTPUT_OFFSET;

use std::error::Error;
use std::io::prelude::*;
use std::fs::File;
use std::path::Path;
use std::process::Command;

use std::fmt;
impl fmt::Display for Register{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }

}
impl fmt::Display for AsmOperand {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            AsmOperand::RegisterOperand(ref dest) =>  write!(f, "{}", dest),
            AsmOperand::Value(ref dest) => write!(f, "{}", dest),
            AsmOperand::Memory(ref mem) => write!(f, "[{}]", mem),
            AsmOperand::MemoryRegister(ref mem) => write!(f, "[{}]", mem)
        }
    }
}

pub fn compile(ops: &Vec<AsmOp>) -> Vec<u8>{
    let mut str = format!("use64\nlea r8, [rip]\nsub r8, 7\nadd r8, {}\n", OUTPUT_OFFSET);
    let mut block_counter = 0;
    for op in ops{
        for _ in 0..block_counter{
            str = str + &"  ";
        }
        let temp_str = match *op {
            AsmOp::Add(ref dest, ref operand) =>  match (dest, operand) {
                (&AsmOperand::Memory(_), &AsmOperand::Value(_)) =>
                    format!("add {}, dword {}\n", dest, operand),
                _ => format!("add {}, {}\n", dest, operand)
            },
            AsmOp::Sub(ref dest, ref operand) => format!("sub {}, {}\n", dest, operand),

            AsmOp::Mul(ref dest, ref operand) => format!("imul {}, {}\n", dest, operand),
            AsmOp::Div(_, _) => panic!(),
            AsmOp::Mod(_, _) => panic!(),

            AsmOp::Pop(ref dest) => format!("popq {}\n", dest),
            AsmOp::Push(ref dest) => format!("pushq {}\n", dest),
            AsmOp::Mov(ref dest, ref operand) => match (dest, operand) {
                (&AsmOperand::Memory(_), &AsmOperand::Value(_)) =>
                    format!("mov {}, dword {}\n", dest, operand),
                _ => format!("mov {}, {}\n", dest, operand)
            },
            AsmOp::Out => "ret".to_string() ,

            AsmOp::Label(ref name) => {
                block_counter += 1;
                format!("{}:\n", name)
            },
            AsmOp::Loop(ref name) => {
                block_counter -= 1;
                format!("\rloopq {}\n", name)
            },
            _ => "\r".to_string()
            // _ => {}
        };
        str = str + &temp_str;
    }
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

#[cfg(windows)]
const FASM: &'static str = "./fasm.exe";

#[cfg(unix)]
const FASM: &'static str = "./fasm";

fn assemble() {
    let output = Command::new(FASM).arg("target/temp.asm").output().unwrap_or_else(|e| {
        panic!("failed to execute process: {}", e)
    });

    if !output.status.success() {
        println!("status: {}", output.status);
        println!("stdout: {}", String::from_utf8_lossy(&output.stdout));
        println!("stderr: {}", String::from_utf8_lossy(&output.stderr));
        panic!("Can't assemble!");
    } else {
        println!("Successfully assembled to temp.bin");
    }
}

pub fn read_bytes() -> Vec<u8>{
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
