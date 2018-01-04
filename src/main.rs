mod util;
mod lexer;
mod parser;
mod emitter;

fn main() {
    let inp = "x = y";
    let mut lexer = lexer::Lexer::new(inp);
    let mut parser = parser::Parser { lexer };

    println!("{:?}",parser.parse_top_level());


    println!("Hello, world!");
}
