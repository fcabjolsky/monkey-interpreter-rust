use self::statements::Statements;
use crate::lexer::Token;

pub mod expressions;
pub mod statements;

pub trait Node {
    fn token_literal(&self) -> Token;
}

pub struct Program {
    pub statements: Vec<Statements>,
}

impl Program {
    pub fn new() -> Program {
        Program { statements: vec![] }
    }
}

impl Node for Program {
    fn token_literal(&self) -> Token {
        match self.statements.first() {
            Some(root) => root.token_literal(),
            _ => Token::ILLEGAL,
        }
    }
}
