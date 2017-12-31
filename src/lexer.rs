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
    Delimiter(char),
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
                self.get_token()
            },
            '"' => {
                self.read_string()
            },
            '0'...'9' => {
                self.read_number()
            },
            'a'...'z' | '_' => {
                self.read_identifier()
            },
            ','|';'|'('|')'|'['|']'|'{'|'}' => {
                Ok(Token::Delimiter(self.input[self.ind]))
            },
            '='|'+'|'-'|'*'|'/'|'%'|'&'|'<'|'>'|'!' => {
                unimplemented!();
            },
            _ => {
                Err(Error{
                    msg: format!("Error reading character {}", self.input[self.ind]),
                    line: 1,
                    col: 1
                })
            }
        }
    }

    fn read_delimiter(&mut self) -> Result<Token, Error> {
        unimplemented!();
    }

    fn read_identifier(&mut self) -> Result<Token, Error> {
        unimplemented!();
    }

    fn read_number(&mut self) -> Result<Token, Error> {
        unimplemented!();
    }

    fn read_string(&mut self) -> Result<Token, Error> {
        unimplemented!();
    }

    fn skip_comment(&mut self) {
        self.consume_while(|ch| {
            ch != '\n'
        });
        self.next_char(); // consume newline
    }

    fn consume_whitespace(&mut self) {
        self.consume_while(|ch| {
            ch == ' ' || ch == '\t'
        })
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

#[cfg(test)]
mod tests {

    use super::*;
}

