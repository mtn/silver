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
    EOF,
}

pub struct Error {
    msg: String,
    line: u32,
    col: u32,
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

    fn eof(&self) -> bool {
        self.ind >= self.input.len()
    }

    fn get_token(&mut self) -> Result<Token, Error> {
        self.consume_whitespace();
        if self.eof() {
            return Ok(Token::EOF)
        }

        match self.input[self.ind] {
            '#' => {
                self.skip_comment();
                Err(Error{ msg: String::from("hi"), line: 1, col: 1 })
            }
            _ => Err(Error{ msg: String::from("hi"), line: 1, col: 1 })
        }
    }

    fn consume_while<F>(&mut self, func: F)
        where F: Fn(char) -> bool
        {
            let mut to_advance = 0;
            for ch in self.input[self.ind..].iter() {
                if func(*ch) {
                    to_advance += 1;
                }
            }

            for _ in 0..to_advance {
                self.next_char();
            }
        }
}
