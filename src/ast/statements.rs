use crate::lexer::Token;

use super::{
    expressions::{Expressions, Identifier},
    Node,
};

#[derive(Clone, Debug)]
pub enum Statements {
    LetNode(LetStatement),
    ReturnNode(ReturnStatement),
}

impl Node for Statements {
    fn token_literal(&self) -> Token {
        match self {
            Statements::LetNode(let_stm) => let_stm.token.clone(),
            Statements::ReturnNode(return_stm) => return_stm.token.clone(),
        }
    }
}

#[derive(Clone, Debug)]
pub struct LetStatement {
    token: Token,
    pub name: Option<Identifier>,
    pub value: Option<Expressions>,
}
impl LetStatement {
    pub fn new(token: Token, ident: Identifier) -> LetStatement {
        LetStatement {
            token,
            name: Some(ident),
            value: None,
        }
    }
}

#[derive(Clone, Debug)]
pub struct ReturnStatement {
    token: Token,
    pub return_value: Option<Expressions>,
}
impl ReturnStatement {
    pub fn new(token: Token) -> ReturnStatement {
        ReturnStatement {
            token,
            return_value: None,
        }
    }
}
