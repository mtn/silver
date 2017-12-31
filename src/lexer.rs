pub struct Lexer <'a> {
    input: Vec<char>,
    ind: usize,
    line: u32,
    col: u32,
    keywords: Vec<&'a str>,
}

#[derive(Debug)]
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

    pub fn lex(&mut self) {
        if let Ok(inside) = self.read_number() {
            println!("{:?}", inside);
        }
        println!("self.ind {}", self.ind);
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
                self.read_operator()
            },
            _ => {
                Err(Error{
                    msg: format!("Error reading character {}", self.input[self.ind]),
                    line: self.line,
                    col: self.col
                })
            }
        }
    }

    fn read_operator(&mut self) -> Result<Token, Error> {
        let operator_chars = "=+-*/%&<>!";
        let op_string = self.read_while(|ch| {
            operator_chars.contains(ch)
        });

        Ok(Token::Operator(op_string))
    }

    fn read_identifier(&mut self) -> Result<Token, Error> {
        let special_id_chars = "?!-<>=_";
        let id = self.read_while(|ch| {
            match ch {
                '0'...'9' | 'a'...'z' | 'A'...'Z' => true,
                _ => special_id_chars.contains(ch)
            }
        });

        for _ in 0..id.len() {
            self.next_char();
        }

        if self.keywords.contains(&id.as_str()) {
            return Ok(Token::Keyword(id))
        }
        Ok(Token::Variable(id))
    }

    fn read_number(&mut self) -> Result<Token, Error> {
        let mut dotted = false;
        let mut digits = String::new();

        for (i,ch) in self.input[self.ind..].iter().enumerate() {
            match *ch {
                '0'...'9' => digits.push(self.input[self.ind + i]),
                '.' => {
                    if dotted {
                        break
                    }
                    dotted = true;
                    digits.push(self.input[self.ind + i])
                }
                _ => break
            }
        }

        for _ in 0..digits.len() {
            self.next_char();
        }

        if digits.contains('.') {
            match digits.parse::<f32>() {
                Ok(floating) => Ok(Token::FloatingPoint(floating)),
                Err(err) => Err(Error{
                    msg: format!("Error parsing float: {}", err),
                    line: self.line,
                    col: self.col,
                })
            }
        } else {
            match digits.parse::<i32>() {
                Ok(integral) => Ok(Token::Integral(integral)),
                Err(err) => Err(Error{
                    msg: format!("Error parsing integer: {}", err),
                    line: self.line,
                    col: self.col,
                })
            }
        }
    }

    fn read_string(&mut self) -> Result<Token, Error> {
        unimplemented!();
        // let mut ret_str = String::new();
        // let mut escaped = false;
        // self.next_char(); // consume opening '"'

        // for (i,ch) in self.input[self.ind..].iter().enumerate() {
        //     if escaped {
        //         ret_str.push(ch)
        //     }
        // }

        // self.read_escaped();
    }

    fn skip_comment(&mut self) {
        self.read_while(|ch| {
            ch != '\n'
        });
        self.next_char(); // consume newline
    }

    fn consume_whitespace(&mut self) {
        self.read_while(|ch| {
            ch == ' ' || ch == '\t'
        });
    }

    fn read_while<F>(&mut self, func: F) -> String
        where F: Fn(char) -> bool
    {
        let mut to_advance = 0;
        let mut ret_str = String::new();

        for (i,ch) in self.input[self.ind..].iter().enumerate() {
            if func(*ch) {
                to_advance += 1;
                ret_str.push(*ch)
            } else {
                break;
            }
        }

        for _ in 0..to_advance {
            self.next_char();
        }

        ret_str
    }
}

#[cfg(test)]
mod tests {

    use super::*;
}

