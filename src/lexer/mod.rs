#[derive(Debug, PartialEq, Eq)]
enum Token {
    ILLEGAL, // = "ILLEGAL"
    EOF,     // = "EOF"
    // Identifiers + literals
    IDENT(String), // = "IDENT" // add, foobar, x, y, ...
    INT(i64),      // = "INT" // 1343456
    // Operators
    ASSIGN, // = "="
    PLUS,   // = "+"
    // Delimiters
    COMMA,     // = ","
    SEMICOLON, // = ";"
    LPAREN,    // = "("
    RPAREN,    // = ")"
    LBRACE,    // = "{"
    RBRACE,    // = "}"
    // Keywords
    FUNCTION, // = "FUNCTION"
    LET,      //= "LET"
}

#[derive(Debug)]
struct Lexer {
    input: Vec<char>,
    /// current position in the input (points to current char)
    position: usize,
    /// current reading position in the input (after the reading char)
    read_position: usize,
    /// current char
    ch: u8,
}

impl Lexer {
    fn new(input: String) -> Lexer {
        let mut lex = Lexer {
            input: input.chars().collect(),
            position: 0,
            read_position: 0,
            ch: 0,
        };
        lex.read_char();
        lex
    }

    fn read_char(&mut self) {
        self.ch = if let Some(ch) = self.input.get(self.read_position) {
            *ch as u8
        } else {
            0
        };
        self.position = self.read_position;
        self.read_position += 1;
    }

    fn skip_whitespace(&mut self) {
        if self.ch.is_ascii_whitespace() {
            self.read_char();
        }
    }

    fn read_identifier(&mut self) -> Token {
        let start = self.position;
        while is_letter(self.ch) {
            self.read_char();
        }
        let ident: String = self.input[start..self.position].iter().collect();
        match ident.as_str() {
            "let" => Token::LET,
            "fn" => Token::FUNCTION,
            _ => Token::IDENT(ident),
        }
    }

    fn read_number(&mut self) -> Token {
        let start = self.position;
        dbg!(self.read_position);
        while self.ch.is_ascii_digit() {
            self.read_char();
        }
        let number: String = self.input[start..self.position].iter().collect();
        dbg!(self.read_position);
        Token::INT(number.parse::<i64>().unwrap_or_default())
    }

    fn next_token(&mut self) -> Token {
        self.skip_whitespace();
        let tok = match self.ch {
            b'=' => Token::ASSIGN,
            b'+' => Token::PLUS,
            b'(' => Token::LPAREN,
            b')' => Token::RPAREN,
            b'{' => Token::LBRACE,
            b'}' => Token::RBRACE,
            b',' => Token::COMMA,
            b';' => Token::SEMICOLON,
            b'0'..=b'9' => {
                return self.read_number();
            }
            b'a'..=b'z' | b'A'..=b'Z' | b'_' => {
                return self.read_identifier();
            }
            0 => Token::EOF,
            _ => Token::ILLEGAL,
        };
        self.read_char();
        tok
    }
}

fn is_letter(ch: u8) -> bool {
    match ch {
        b'a'..=b'z' | b'A'..=b'Z' | b'_' => true,
        _ => false,
    }
}

#[cfg(test)]
mod tests {
    use super::{Lexer, Token};

    #[test]
    fn test_next_token_simple() {
        let input = "=+(){},;";
        let tests = [
            Token::ASSIGN,
            Token::PLUS,
            Token::LPAREN,
            Token::RPAREN,
            Token::LBRACE,
            Token::RBRACE,
            Token::COMMA,
            Token::SEMICOLON,
            Token::EOF,
        ];

        let mut l = Lexer::new(input.to_string());

        tests.iter().for_each(|test_tok| {
            let tok = l.next_token();
            assert_eq!(tok, *test_tok);
        });
    }

    #[test]
    fn test_next_token_complex() {
        let input = r#"let five = 5;
let ten = 10;
let add = fn(x, y) {
x + y;
};
let result = add(five, ten);"#;
        let tests = [
            Token::LET,
            Token::IDENT(String::from("five")),
            Token::ASSIGN,
            Token::INT(5),
            Token::SEMICOLON,
            Token::LET,
            Token::IDENT(String::from("ten")),
            Token::ASSIGN,
            Token::INT(10),
            Token::SEMICOLON,
            Token::LET,
            Token::IDENT(String::from("add")),
            Token::ASSIGN,
            Token::FUNCTION,
            Token::LPAREN,
            Token::IDENT(String::from("x")),
            Token::COMMA,
            Token::IDENT(String::from("y")),
            Token::RPAREN,
            Token::LBRACE,
            Token::IDENT(String::from("x")),
            Token::PLUS,
            Token::IDENT(String::from("y")),
            Token::SEMICOLON,
            Token::RBRACE,
            Token::SEMICOLON,
            Token::LET,
            Token::IDENT(String::from("result")),
            Token::ASSIGN,
            Token::IDENT(String::from("add")),
            Token::LPAREN,
            Token::IDENT(String::from("five")),
            Token::COMMA,
            Token::IDENT(String::from("ten")),
            Token::RPAREN,
            Token::SEMICOLON,
            Token::EOF,
        ];

        let mut l = Lexer::new(input.to_string());

        tests.iter().for_each(|test_tok| {
            let tok = l.next_token();
            dbg!(&tok);
            assert_eq!(tok, *test_tok);
        });
    }
}
