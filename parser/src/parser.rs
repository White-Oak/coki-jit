use coki_grammar::grammar::*;
use coki_grammar::grammar_lexer::*;

use peruse::slice_parsers::*;

pub fn program() -> Box<SliceParser<I = [Token], O = Block>> {

    fn code_block() -> Box<SliceParser<I = [Token], O = Block>> {
        Box::new(lit(Token::OpenBrace)
                     .then_r(recursive(|| program()))
                     .then_l(lit(Token::CloseBrace)))
    }

    fn if_stmt() -> Box<SliceParser<I = [Token], O = Statement>> {

        let comparator = || {
            matcher(|token| {
                match token {
                    Token::Cmp(cmp) => Some(cmp),
                    _ => None,
                }
            })
        };


        let else_block = lit(Token::ElseKeyword).then_r(recursive(if_stmt)
                                                            .map(|i| Block(vec![i]))
                                                            .or(boxed(code_block())));

        let p = lit(Token::IfKeyword)
                    .then_r(recursive(|| expression()))
                    .then(comparator())
                    .then(recursive(expression))
                    .then_l(lit(Token::OpenBrace))
                    .then(recursive(program))
                    .then_l(lit(Token::CloseBrace))
                    .then(opt(else_block))
                    .map(|((((l, cmp), r), block), else_opt)| {
                        Statement::If(l, cmp, r, block, else_opt)
                    });

        Box::new(p)
    }

    fn expression() -> Box<SliceParser<I = [Token], O = Expr>> {
        let simple_term = matcher(|token| {
            match token {
                Token::Ident(name) => Some(Expr::Variable(name)),
                Token::Number(i) => Some(Expr::Num(i)),
                _ => None,
            }
        });

        let paren_expr = || {
            lit(Token::OpenParen).then_r(recursive(|| expression())).then_l(lit(Token::CloseParen))
        };

        let factor = paren_expr().or(simple_term);

        let multop = matcher(|token| {
            match token {
                Token::MultSign => Some(MultOp::Multiply),
                Token::DivideSign => Some(MultOp::Divide),
                Token::ModuloSign => Some(MultOp::Modulo),
                _ => None,
            }
        });

        let addop = matcher(|token| {
            match token {
                Token::PlusSign => Some(AddOp::Add),
                Token::MinusSign => Some(AddOp::Subtract),
                _ => None,
            }
        });

        let mult = {
            let p = factor.clone().then(multop.then(factor).repeat()).map(|(first, seq)| {
                let mut ops = Vec::new();
                ops.push(MultTerm(MultOp::Start, first));
                for &(ref op, ref value) in &seq {
                    ops.push(MultTerm(op.clone(), value.clone())); //maybe box the value instead
                }
                if ops.len() == 1 {
                    ops[0].1.clone()
                } else {
                    Expr::MultDiv(ops)
                }
            });
            boxed(Box::new(p))
        };

        let plus = {
            let p = mult.clone().then(addop.then(mult).repeat()).map(|(first, seq)| {
                let mut ops = Vec::new();
                ops.push(AddTerm(AddOp::Start, first));
                for &(ref op, ref value) in &seq {
                    ops.push(AddTerm(op.clone(), value.clone()));
                }
                if ops.len() == 1 {
                    ops[0].1.clone()
                } else {
                    Expr::AddSub(ops)
                }
            });
            boxed(Box::new(p))
        };

        Box::new(plus)
    }

    let assignment = {
        let target = matcher(|token| {
            match token {
                Token::Ident(name) => Some(name),
                _ => None,
            }
        });
        boxed(Box::new(target.then_l(lit(Token::Equals))
                             .then(recursive(|| expression()))
                             .map(|(target, expr)| Statement::Assign(target, expr))))
    };

    let comparator = || {
        matcher(|token| {
            match token {
                Token::Cmp(cmp) => Some(cmp),
                _ => None,
            }
        })
    };

    let while_stmt = {
        let p = lit(Token::WhileKeyword)
                    .then_r(recursive(expression))
                    .then(comparator())
                    .then(recursive(expression))
                    .then_l(lit(Token::OpenBrace))
                    .then(recursive(program))
                    .then_l(lit(Token::CloseBrace))
                    .map(|(((l, cmp), r), block)| Statement::While(l, cmp, r, block));
        boxed(Box::new(p))
    };

    let loop_stmt = {
        let p = lit(Token::LoopKeyword)
                    .then_r(recursive(expression))
                    .then_l(lit(Token::OpenBrace))
                    .then(recursive(program))
                    .then_l(lit(Token::CloseBrace))
                    .map(|(l, block)| Statement::Loop(l, block));
        boxed(Box::new(p))
    };
    let output = boxed(Box::new(lit(Token::OutputCmd)
                                    .then_r(recursive(|| expression()))
                                    .map(Statement::Output)));

    let statements = one_of(vec![assignment, output, boxed(if_stmt()), while_stmt, loop_stmt])
                         .then_l(lit(Token::NewLine))
                         .repeat();

    Box::new(statements.map(Block))
}
