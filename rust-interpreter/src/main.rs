mod interpreter;
use interpreter::repl;
use std::io;

fn main() {
    println!("Hello! This is the Monkey programming language!");
    println!("Feel free to type in commands");
    repl::start(io::stdin().lock(), io::stdout());
}