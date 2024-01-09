use crate::lexer::Token;

use super::Node;

#[derive(Clone, Debug, PartialEq)]
pub enum Expressions {
    Identifier(Identifier),
}
impl Node for Expressions {
    fn token_literal(&self) -> Token {
        match self {
            Expressions::Identifier(ident) => ident.token.clone(),
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Identifier {
    token: Token,
    pub value: String,
}
impl Identifier {
    pub fn new(tok: &Token) -> Option<Identifier> {
        match tok {
            Token::IDENT(ident) => Some(Identifier {
                token: tok.clone(),
                value: ident.clone(),
            }),
            _ => None,
        }
    }
}
