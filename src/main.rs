#![allow(warnings)]

mod token;
mod lexer;
mod parser;
mod interpreter;

use lexer::lex;
use parser::parse;
use interpreter::interpret;

fn main() {
    let source = std::fs::read_to_string("gurt/yo.gurt").expect("couldn't read file");
    
    let tokens = lex(&source);
    let ast = parse(tokens);

    interpret(ast);
}
