#[derive(Debug, PartialEq, Eq)]
enum Token {
    ILLEGAL, // = "ILLEGAL"
    EOF,     // = "EOF"

    // Identifiers + literals
    IDENT(String), // = "IDENT" // add, foobar, x, y, ...
    INT(i64),      // = "INT" // 1343456

    // Operators
    ASSIGN,   // = "="
    PLUS,     // = "+"
    MINUS,    // = "-"
    BANG,     // = "!"
    ASTERISK, // = "*"
    SLASH,    // = "/"
    LT,       // = "<"
    GT,       // = ">"
    EQ,       // = "=="
    NotEq,   // = "!="

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
    TRUE,     // = "TRUE"
    FALSE,    // = "FALSE"
    IF,       // = "IF"
    ELSE,     // = "ELSE"
    RETURN,   // = "RETURN"
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
        self.ch = self.input.get(self.read_position).map_or(0, |ch| *ch as u8);
        self.position = self.read_position;
        self.read_position += 1;
    }

    fn peek_char(&self) -> u8 {
        self.input.get(self.read_position).map_or(0, |ch| *ch as u8)
    }

    fn skip_whitespaces(&mut self) {
        while self.ch.is_ascii_whitespace() {
            self.read_char();
        }
    }

    fn read_word(&mut self, cond: fn(u8) -> bool) -> String {
        let start = self.position;
        while cond(self.ch) {
            self.read_char();
        }
        self.input[start..self.position].iter().collect()
    }

    fn read_identifier(&mut self) -> Token {
        let ident = self.read_word(is_letter);
        match ident.as_str() {
            "let" => Token::LET,
            "fn" => Token::FUNCTION,
            "true" => Token::TRUE,
            "false" => Token::FALSE,
            "if" => Token::IF,
            "else" => Token::ELSE,
            "return" => Token::RETURN,
            _ => Token::IDENT(ident),
        }
    }

    fn read_number(&mut self) -> Token {
        let number = self.read_word(is_digit);
        Token::INT(number.parse::<i64>().unwrap_or_default())
    }

    fn next_token(&mut self) -> Token {
        self.skip_whitespaces();
        let tok = match self.ch {
            b'=' => match self.peek_char() {
                b'=' => {
                    self.read_char();
                    Token::EQ
                }
                _ => Token::ASSIGN
            },
            b'!' => match self.peek_char() {
                b'=' => {
                    self.read_char();
                    Token::NotEq
                }
                _ => Token::BANG
            },
            b'+' => Token::PLUS,
            b'(' => Token::LPAREN,
            b')' => Token::RPAREN,
            b'{' => Token::LBRACE,
            b'}' => Token::RBRACE,
            b',' => Token::COMMA,
            b';' => Token::SEMICOLON,
            b'-' => Token::MINUS,
            b'*' => Token::ASTERISK,
            b'<' => Token::LT,
            b'>' => Token::GT,
            b'/' => Token::SLASH,
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

fn is_digit(ch: u8) -> bool {
    ch.is_ascii_digit()
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
let result = add(five, ten);
!-/*5;
5 < 10 > 5;

if (5 < 10) {
return true;
} else {
return false;
}

10 == 10;
10 != 9;"#;
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
            Token::BANG,
            Token::MINUS,
            Token::SLASH,
            Token::ASTERISK,
            Token::INT(5),
            Token::SEMICOLON,
            Token::INT(5),
            Token::LT,
            Token::INT(10),
            Token::GT,
            Token::INT(5),
            Token::SEMICOLON,
            Token::IF,
            Token::LPAREN,
            Token::INT(5),
            Token::LT,
            Token::INT(10),
            Token::RPAREN,
            Token::LBRACE,
            Token::RETURN,
            Token::TRUE,
            Token::SEMICOLON,
            Token::RBRACE,
            Token::ELSE,
            Token::LBRACE,
            Token::RETURN,
            Token::FALSE,
            Token::SEMICOLON,
            Token::RBRACE,
            Token::INT(10),
            Token::EQ,
            Token::INT(10),
            Token::SEMICOLON,
            Token::INT(10),
            Token::NotEq,
            Token::INT(9),
            Token::SEMICOLON,
            Token::EOF,
        ];

        let mut l = Lexer::new(input.to_string());

        tests.iter().for_each(|test_tok| {
            let tok = l.next_token();
            assert_eq!(tok, *test_tok);
        });
    }
}
