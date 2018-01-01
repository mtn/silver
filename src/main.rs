mod util;
mod lexer;
mod parser;

fn main() {
    let mut lexer = lexer::Lexer::new("\"1.2.1\"");
    lexer.lex();

    println!("Hello, world!");
}
