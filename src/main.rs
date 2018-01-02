mod util;
mod lexer;
mod parser;
mod emitter;

fn main() {
    let mut lexer = lexer::Lexer::new("a(a,b)");
    let mut parser = parser::Parser { lexer };
    parser.parse();


    println!("Hello, world!");
}
