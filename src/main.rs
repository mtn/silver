mod lexer;

fn main() {
    let mut lexer = lexer::Lexer::new("1.2.3");
    lexer.lex();

    println!("Hello, world!");
}
