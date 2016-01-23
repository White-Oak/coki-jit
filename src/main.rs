#![feature(custom_derive)]
#![feature(unboxed_closures)]
#![feature(convert)]
extern crate regex;
extern crate peruse;
extern crate jitter;
extern crate getopts;

use grammar::*;
use parser::program;
use lexer::token;
use ir::translate;
use compiler::compile;
use jitter::jit::get_jit;
use optimizer::optimize;
use ast_optimizer::optimize_ast;

use std::fs::File;
use std::env::args;
use std::io::Read;
use getopts::Options;


pub mod lexer;
pub mod grammar;
pub mod grammar_lexer;
pub mod parser;
pub mod ir;
pub mod asm_ops;
pub mod compiler;
pub mod optimizer;
pub mod ast_optimizer;


fn print_usage(program: &str, opts: Options) {
    let brief = format!("Usage: {} FILE [options]", program);
    print!("{}", opts.usage(&brief));
}
fn main() {
    let args: Vec<String> = args().collect();
    let program = args[0].clone();

    let mut opts = Options::new();
    opts.optopt("", "max-optimizations", "set a maximum number of optimization passes", "NUMBER");
    opts.optflagopt("", "bin", "outputs a binary executable", "FILE");
    opts.optflag("h", "help", "print this help menu");
    let matches = match opts.parse(&args[1..]) {
        Ok(m) => { m }
        Err(f) => { panic!(f.to_string()) }
    };
    if matches.opt_present("h") {
        print_usage(&program, opts);
        return;
    }

    let file = if !matches.free.is_empty() {
        &matches.free[0] as &str
    } else {
        "examples/oka.coki"
    };

    let max_optimizations = if matches.opt_present("max-optimizations") {
        matches.opt_str("max-optimizations").unwrap().parse::<u8>().unwrap()
    } else {
        100
    };

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
