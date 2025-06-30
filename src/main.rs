mod token;
mod lexer;

use lexer::lex;

fn main() {
    let source = std::fs::read_to_string("gurt/yo.gurt").expect("couldn't read file");
    lex(&source);
}
