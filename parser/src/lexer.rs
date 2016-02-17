use peruse::slice_parsers::*;
use peruse::string_parsers::*;

use coki_grammar::grammar_lexer::*;
use coki_grammar::grammar::Comparator;
use std::str::FromStr;

// type Lexer = SliceParser<I=str, O=Token>;


pub fn token() -> Box<SliceParser<I = str, O = Vec<Token>>> {

    fn lt(s: &str, t: Token) -> RegexLiteralParser<Token> {
        str_lit((String::from_str(r"^[ \t]*").unwrap() + s).as_str(), t)
    }

    let ident = capture(r"^[ \t]*([a-zA-Z]\w*)[ \t]*",
                        |caps| Token::Ident(String::from_str(caps.at(1).unwrap()).unwrap()));

    let number = capture(r"^[ \t]*(\d+)[ \t]*",
                         |caps| Token::Number(FromStr::from_str(caps.at(1).unwrap()).unwrap()));

    let lits = one_of(vec![lt("out", Token::OutputCmd),
                           lt("if", Token::IfKeyword),
                           lt("else", Token::ElseKeyword),
                           lt("while", Token::WhileKeyword),
                           lt("loop", Token::LoopKeyword),
                           lt(r"\r?\n\s*", Token::NewLine),
                           lt(r"\(\s*", Token::OpenParen),
                           lt(r"\)", Token::CloseParen),
                           lt(r"\}", Token::CloseBrace),
                           lt("==", Token::Cmp(Comparator::Eq)),
                           lt("!=", Token::Cmp(Comparator::Neq)),
                           lt(">=", Token::Cmp(Comparator::Geq)),
                           lt(r"\{\s*", Token::OpenBrace),
                           lt("<=", Token::Cmp(Comparator::Leq)),
                           lt(">", Token::Cmp(Comparator::Gt)),
                           lt("<", Token::Cmp(Comparator::Lt)),
                           lt(r"\+", Token::PlusSign),
                           lt("-", Token::MinusSign),
                           lt("=", Token::Equals),
                           lt(r"\*", Token::MultSign),
                           lt("/", Token::DivideSign),
                           lt(r"%", Token::ModuloSign)]);

    let options = lits.or(number).or(ident).repeat();
    Box::new(options)

}
