pub struct Lexer <'a> {
    input: Vec<char>,
    ind: usize,
    line: u32,
    col: u32,
    keywords: Vec<&'a str>,
}

pub enum Token {
    Variable(String),
    Operator(String),
    Keyword(String),
    StringLiteral(String),
    Integral(i32),
    FloatingPoint(f32),
}

impl <'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Lexer {
        Lexer {
            input: input.chars().collect(),
            ind: 0,
            line: 1,
            col: 0,
            keywords: vec!["fn", "true", "false", "if", "then", "else"],
        }
    }

    fn next_char(&mut self) -> char {
        let ch = self.input[self.ind];
        self.ind += 1;
        if ch == '\n' {
            self.line += 1;
            self.col = 0;
        } else {
            self.col += 1;
        }

        ch
    }
}
