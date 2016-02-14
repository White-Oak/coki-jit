#![feature(custom_derive)]
#![feature(unboxed_closures)]
extern crate coki_jitter;
extern crate coki_parser;
#[macro_use] extern crate clap;

use coki_parser::{parse, Block};
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

    interp(contents.as_str(), max_optimizations);

    println!("coki has done the job");
}

fn interp<'a>(raw: &'a str, opt: u8) {
    match parse(raw) {
        Ok(Block(stmts)) => {
                let opt_ast = optimize_ast(stmts, opt);
                let ops = translate(&opt_ast);
                let _ = compile(&ops);
                let opt_ops = optimize(*ops, opt);
                let bytes = compile(&opt_ops);
                let fun = get_jit(bytes);
                println!("Output:");
                fun();
        }
        Err(err) => panic!("Parse Error: {:?}", err)
    }
}
