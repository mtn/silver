mod lexer;

fn main() {
    let mut lexer = lexer::Lexer::new("if");
    lexer.lex();

    println!("Hello, world!");
}
