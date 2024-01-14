mod interpreter;
use std::io;

fn main() {
    println!("Hello! This is the Monkey programming language!");
    println!("Feel free to type in commands");
    interpreter::repl::start(io::stdin().lock(), io::stdout());
}