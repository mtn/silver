mod util;
mod lexer;
mod parser;
mod emitter;

fn main() {
    let inp = "fn a (b,c) {
                   if b {
                       c = b
                   } else {
                       b = c
                   };
                   b
               }";
    let mut lexer = lexer::Lexer::new(inp);
    let mut parser = parser::Parser { lexer };

    if let Ok(res) = parser.parse_top_level() {
        println!("{:?}", emitter::emit(res));
    } else {
        println!("Hello, world!");
    }

}
