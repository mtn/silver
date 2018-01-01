mod util;
mod lexer;
mod parser;

fn main() {
    let mut lexer = lexer::Lexer::new("(a,b)");
    let mut parser = parser::Parser { lexer };
    parser.parse();


    println!("Hello, world!");
}
