use crate::lexer::{Lexer, Token};
use std::io::{self, Write};

static EXIT: &str = ".exit\n";
static PROMPT: &str = ">> ";

pub fn start() {
    loop {
        io::stdout()
            .write(PROMPT.as_bytes())
            .expect("Failed to write to stdout");
        io::stdout().flush().expect("Failed to flush stdout");
        let mut input = String::new();

        match io::stdin().read_line(&mut input) {
            Ok(_) => {
                if input == EXIT {
                    break;
                }
                let mut lexer = Lexer::new(input);
                let mut tok = lexer.next_token();
                while tok != Token::EOF {
                    io::stdout()
                        .write(format!("{:?}\n", tok).as_bytes())
                        .expect("Failed to write to stdout");
                    tok = lexer.next_token();
                }
            }
            _ => break,
        }
    }
}
