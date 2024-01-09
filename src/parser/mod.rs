use crate::{
    ast::{
        expressions::Identifier,
        statements::{LetStatement, ReturnStatement, Statements},
        Program,
    },
    lexer::{Lexer, Token},
};

pub struct Parser {
    lexer: Lexer,
    curr_token: Token,
    peek_token: Token,
    pub errors: Vec<String>,
}

impl Parser {
    pub fn new(lexer: Lexer) -> Parser {
        let mut parser = Parser {
            lexer,
            curr_token: Token::EOF,
            peek_token: Token::EOF,
            errors: vec![],
        };
        parser.next_token();
        parser.next_token();
        parser
    }

    fn peek_error(&mut self, expected: Token) {
        self.errors.push(format!(
            "Expected {} but found {}",
            expected, self.peek_token
        ));
    }

    fn has_error(&self) -> bool {
        !self.errors.is_empty()
    }

    fn get_errors(&self) -> String {
        self.errors.join(" ")
    }

    fn next_token(&mut self) {
        self.curr_token = self.peek_token.clone();
        self.peek_token = self.lexer.next_token();
    }

    pub fn parse_program(&mut self) -> Option<Program> {
        let mut program = Program::new();

        while self.curr_token != Token::EOF {
            if let Some(stm) = self.parse_statment() {
                program.statements.push(stm);
            }
            self.next_token();
        }

        // TODO: return a result with the errors
        if self.has_error() {
            dbg!(self.get_errors());
            return None;
        }
        Some(program)
    }

    fn parse_statment(&mut self) -> Option<Statements> {
        match self.curr_token {
            Token::LET => self.parse_let_statement(),
            Token::RETURN => self.parse_return_statement(),
            _ => None,
        }
    }

    fn parse_let_statement(&mut self) -> Option<Statements> {
        if !self.peek_token(Token::IDENT(String::new())) {
            return None;
        }

        let ident = match Identifier::new(&self.curr_token) {
            Some(ident) => ident,
            None => return None,
        };

        let stm = LetStatement::new(Token::LET, ident);

        if !self.peek_token(Token::ASSIGN) {
            return None;
        }

        while self.curr_token != Token::SEMICOLON {
            self.next_token();
        }

        Some(Statements::LetNode(stm))
    }

    fn parse_return_statement(&mut self) -> Option<Statements> {
        let stm = ReturnStatement::new(Token::RETURN);

        self.next_token();

        while self.curr_token != Token::SEMICOLON {
            self.next_token();
        }
        Some(Statements::ReturnNode(stm))
    }

    fn peek_token(&mut self, expected: Token) -> bool {
        if std::mem::discriminant(&self.peek_token) == std::mem::discriminant(&expected) {
            self.next_token();
            return true;
        }
        self.peek_error(expected);
        return false;
    }
}

#[cfg(test)]
mod test {
    use crate::{
        ast::{statements::Statements, Node},
        lexer::{Lexer, Token},
    };

    use super::Parser;

    #[test]
    fn test_let_statments() {
        let input = r#"
let x = 5;
let y = 10;
let foobar = 838383;"#;
        let lexer = Lexer::new(input.to_string());
        let mut parser = Parser::new(lexer);

        let program = parser.parse_program();
        assert!(program.is_some());
        let program = program.unwrap();
        assert_eq!(
            program.statements.len(),
            3,
            "The program does not contain 3 statements"
        );
        let expected_identifiers = ["x", "y", "foobar"];
        for (i, ident) in expected_identifiers.into_iter().enumerate() {
            let statement = program.statements.get(i).unwrap();
            test_let_statment(statement, ident);
        }
    }

    fn test_let_statment(stm: &Statements, ident: &str) {
        assert_eq!(stm.token_literal(), Token::LET, "Not a let token");
        match stm {
            Statements::LetNode(let_stm) => {
                match &let_stm.name {
                    Some(name) => assert_eq!(name.value, ident),
                    None => todo!(),
                }
                // match &let_stm.value {
                //     Some(value) => assert_eq!(value.token_literal(), Token::IDENT(ident.to_string())),
                //     None => todo!(),
                // }
            }
            _ => unreachable!(),
        }
    }

    #[test]
    fn test_return_statments() {
        let input = r#"
return 5;
return 10;
return 993322;"#;
        let lexer = Lexer::new(input.to_string());
        let mut parser = Parser::new(lexer);

        let program = parser.parse_program();
        assert!(program.is_some());
        let program = program.unwrap();
        assert_eq!(
            program.statements.len(),
            3,
            "The program does not contain 3 statements"
        );

        for statement in program.statements {
            assert_eq!(
                statement.token_literal(),
                Token::RETURN,
                "Not a return statment"
            );
        }
    }
}
