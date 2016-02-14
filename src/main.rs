#![feature(custom_derive)]
#![feature(unboxed_closures)]
extern crate regex;
extern crate peruse;
extern crate jitter;
extern crate getopts;
#[macro_use] extern crate clap;

use grammar::*;
use parser::program;
use lexer::token;
use ir::translate;
use compiler::compile;
use jitter::jit::get_jit;
use optimizer::optimize;
use ast_optimizer::optimize_ast;

use std::fs::File;
use std::io::Read;
use clap::App;

pub mod lexer;
pub mod grammar;
pub mod grammar_lexer;
pub mod parser;
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
    let lexer = token();
    match lexer.parse(raw) {
        Ok((tokens, rest)) => {
            println!("{:?}\n", tokens);
            if rest != "" {
                println!("Parser error at: {:?}", rest)
            } else {
                let parser = program();
                match parser.parse(tokens.as_slice()) {
                    Ok((Block(stmts), rest)) => {
                        if rest.len() > 0 {
                            println!("Error: unexpected token {:?}", rest[0]);
                        } else {
                            let opt_ast = optimize_ast(stmts, opt);
                            let ops = translate(&opt_ast);
                            let _ = compile(&ops);
                            let opt_ops = optimize(*ops, opt);
                            let bytes = compile(&opt_ops);
                            let fun = get_jit(bytes);
                            println!("Output:");
                            fun();
                        }
                    }
                    Err(err) => {println!("Parse Error: {:?}", err);}
                };
            }
        },
        Err(err) => {
            println!("Lexer error: {:?}", err);
        }
    }

}
