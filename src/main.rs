#![feature(custom_derive)]
#![feature(unboxed_closures)]
extern crate coki_jitter;
extern crate coki_parser;
extern crate coki_grammar;
#[macro_use]
extern crate clap;

use coki_parser::*;
use ir::translate;
use compiler::compile;
use coki_jitter::jit::get_jit;
use optimizer::optimize;
use ast_optimizer::optimize_ast;

use std::fs::File;
use std::io::Read;
use clap::App;

pub mod ir;
pub mod asm_ops;
pub mod compiler;
pub mod optimizer;
pub mod ast_optimizer;

fn main() {
    let yaml = load_yaml!("cli.yml");
    let matches = App::from_yaml(yaml).get_matches();

    let file = matches.value_of("INPUT").unwrap_or("examples/oka.coki");

    let max_optimizations = matches.value_of("opts").unwrap_or("100").parse::<u8>().unwrap();

    let mut contents = String::new();
    let mut f = File::open(file).unwrap();
    let _ = f.read_to_string(&mut contents);

    let bytes = interp(contents.as_str(), max_optimizations);
    if !matches.is_present("bin") {
        run(&bytes);
    } else {
        save_for_wrapper(&bytes);
    }
    println!("coki has done the job");
}

use std::error::Error;
use std::io::Write;
use std::path::Path;
use std::process::Command;
fn save_for_wrapper(bytes: &[u8]) {
    let path = Path::new("wrapper/temp.bin");
    let display = path.display();

    let mut file = match File::create(&path) {
        Err(why) => panic!("couldn't create {}: {}", display, Error::description(&why)),
        Ok(file) => file,
    };

    match file.write_all(bytes) {
        Err(why) => {
            panic!("couldn't write to {}: {}",
                   display,
                   Error::description(&why))
        }
        Ok(_) => println!("Successfully wrote to {}", display),
    }

    let mut child = Command::new("cargo")
                     .arg("build")
                     .arg("--release")
                     .current_dir("wrapper")
                     .spawn()
                     .unwrap_or_else(|e| panic!("failed to build binary: {}", e));
    child.wait().unwrap();
    println!("Successfully wrapped to wrapper/target/release/coki_wrapper");
}

fn run(bytes: &[u8]) {
    let fun = get_jit(bytes);
    println!("Output:");
    fun();
}

fn interp(raw: &str, opt: u8) -> Vec<u8> {
    match parse(raw) {
        Ok(Block(stmts)) => {
            let opt_ast = optimize_ast(stmts, opt);
            let ops = translate(&opt_ast);
            let _ = compile(&ops);
            let opt_ops = optimize(ops, opt);
            compile(&opt_ops)
        }
        Err(err) => panic!("Parse Error: {:?}", err),
    }
}
