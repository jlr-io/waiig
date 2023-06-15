use crate::lexer::Lexer;
use std::{io::{Write, BufRead}};

const PROMPT: &'static str = ">> ";

pub fn start<R: BufRead, W: Write>(mut input: R, mut output: W) {
    loop {
        write!(output, "{}", PROMPT).unwrap();
        output.flush().unwrap();

        let mut line = String::new();
        if input.read_line(&mut line).unwrap() == 0 { return; }
        
        let lexer = Lexer::new(line);
        lexer.into_iter().for_each(|token| writeln!(output, "{:#?}", token).unwrap());
    }
}