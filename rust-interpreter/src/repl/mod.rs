use crate::token::Token;
use crate::lexer::Lexer;
use std::io::{Write, BufRead};

const PROMPT: &'static str = ">> ";

pub fn start<R: BufRead, W: Write>(mut input: R, mut output: W) {
    loop {
        write!(output, "{}", PROMPT).unwrap();
        output.flush().unwrap();

        let mut line = String::new();
        if input.read_line(&mut line).unwrap() == 0 { return; }

        let mut lexer = Lexer::new(line);
        let mut token = lexer.next_token();
        while token != Token::Eof {
            writeln!(output, "{:#?}", token).unwrap();
            token = lexer.next_token();
        }
    }
}