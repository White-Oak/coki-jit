extern crate peruse;
extern crate coki_grammar;

mod lexer;
mod parser;

use parser::program;
use lexer::token;

pub use coki_grammar::grammar::Block;

pub fn parse(raw: &str) -> Result<Block, String> {
    match token().parse(raw) {
        Ok((tokens, rest)) => {
            println!("{:?}\n", tokens);
            if rest != "" {
                Err(format!("Parser error at: {:?}", rest))
            } else {
                match program().parse(&tokens) {
                    Ok((block, rest)) => {
                        if rest.len() > 0 {
                            Err(format!("Error: unexpected token {:?}", rest[0]))
                        } else {
                            Ok(block)
                        }
                    }
                    Err(err) => Err(format!("Parser error: {:?}", err)),
                }
            }
        }
        Err(err) => Err(format!("Lexer error: {:?}", err)),
    }
}
