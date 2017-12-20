pub mod ast;

use parser::ast::Expr;
use lexer::tokens::{Token, TokenType};
use std::collections::HashMap;
use std::rc::Rc;

pub struct Parser {
    ast: Vec<Expr>,
    gramar: Grammar,
}

#[derive(Clone)]
pub struct Grammar {
    prefixes: HashMap<TokenType, Rc<PrefixParser>>,
    infixes: HashMap<TokenType, Rc<InfixParser>>,
}

impl Default for Grammar {
    fn default() -> Self {
        Grammar {
            prefixes: HashMap::new(),
            infixes: HashMap::new(),
        }
    }
}

pub trait InfixParser {
    fn precedence(&self) -> usize;
    fn parse(&self, parser: Parser, left: Expr, token: Token) -> Expr;
}

pub trait PrefixParser {
    fn parse(&self, parser: Parser, token: Token) -> Expr;
}