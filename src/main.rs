#![feature(custom_derive)]
#![feature(unboxed_closures)]
#![feature(convert)]
extern crate regex;
extern crate peruse;

use grammar::*;
use parser::program;
use lexer::token;
use ir::translate;
use compiler::compile;
use jit::get_jit;
use optimizer::optimize;

use std::fs::File;
use std::env::args;
use std::io::Read;


pub mod lexer;
pub mod grammar;
pub mod grammar_lexer;
pub mod parser;
pub mod ir;
pub mod asm_ops;
pub mod compiler;
pub mod jit;
pub mod optimizer;


fn main() {
    let args: Vec<_> = args().collect();

    let ref file = args[1];
    let max_optimizations = args[2].parse::<u8>().unwrap();

    let mut contents = String::new();
    let mut f = File::open(file.as_str()).unwrap();
    let _ = f.read_to_string(&mut contents);

    interp(contents.as_str(), max_optimizations);

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
                            let ops = translate(&stmts);
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
