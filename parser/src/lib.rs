extern crate peruse;

mod grammar_lexer;
mod grammar;
mod lexer;
mod parser;

pub use grammar::*;
use parser::program;
use lexer::token;

pub fn parse<'a>(raw: &'a str) -> Result<Block, String> {
    match token().parse(raw) {
        Ok((tokens, rest)) => {
            println!("{:?}\n", tokens);
            if rest != "" {
                Err(format!("Parser error at: {:?}", rest))
            } else {
                match program().parse(tokens.as_slice()) {
                    Ok((block, rest)) => {
                        if rest.len() > 0 {
                            Err(format!("Error: unexpected token {:?}", rest[0]))
                        } else {
                            Ok(block)
                        }
                    }
                    Err(err) => Err(format!("Parser error: {:?}", err))
                }
            }
        },
        Err(err) => {
            Err(format!("Lexer error: {:?}", err))
        }
    }
}
