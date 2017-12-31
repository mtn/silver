mod lexer;

fn main() {
    let mut lexer = lexer::Lexer::new("+!");
    lexer.lex();

    println!("Hello, world!");
}
