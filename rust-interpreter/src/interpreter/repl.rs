use crate::interpreter::lexer::Lexer;
use std::{io::{Write, BufRead}};

pub fn start<R: BufRead, W: Write>(mut input: R, mut output: W) {
    loop {
        write!(output, "{}", ">> ").unwrap();
        output.flush().unwrap();

        let mut line = String::new();
        if input.read_line(&mut line).unwrap() == 0 { return; }

        let lexer = Lexer::new(&line);
        lexer.into_iter().for_each(|token| writeln!(output, "{:#?}", token).unwrap());
    }
}