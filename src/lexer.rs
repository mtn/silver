use super::util::Error;

pub struct Lexer<'a> {
    input: Vec<char>,
    ind: usize,
    line: u32,
    col: u32,
    keywords: Vec<&'a str>,
    peeked: Option<Token>,
}

#[derive(Debug, PartialEq, Clone)]
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

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Lexer {
        Lexer {
            input: input.chars().collect(),
            ind: 0,
            line: 1,
            col: 0,
            keywords: vec!["fn", "true", "false", "if", "then", "else"],
            peeked: None,
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

    pub fn eof(&self) -> bool {
        self.ind >= self.input.len()
    }

    pub fn get_token(&mut self) -> Result<Token, Error> {
        let ret: Result<Token, Error>;
        if let Some(_) = self.peeked {
            ret = Ok(self.peeked.clone().unwrap());
            self.peeked = None;
            return ret;
        }

        self.consume_whitespace();
        if self.eof() {
            return Ok(Token::EOF);
        }

        match self.input[self.ind] {
            '#' => {
                self.skip_comment();
                self.get_token()
            }
            '"' => self.read_string(),
            '0'...'9' => self.read_number(),
            'a'...'z' | '_' => self.read_identifier(),
            ',' | ';' | '(' | ')' | '[' | ']' | '{' | '}' => Ok(Token::Delimiter(self.next_char())),
            '=' | '+' | '-' | '*' | '/' | '%' | '&' | '<' | '>' | '!' => self.read_operator(),
            _ => Err(self.get_error(format!("Error reading character {}", self.input[self.ind]))),
        }
    }

    pub fn peek(&mut self) -> Result<Token, Error> {
        if let None = self.peeked {
            self.peeked = Some(self.get_token()?);
        }
        Ok(self.peeked.clone().unwrap())
    }

    fn read_operator(&mut self) -> Result<Token, Error> {
        let operator_chars = "=+-*/%&<>!";
        let op_string = self.read_while(|ch| operator_chars.contains(ch));

        Ok(Token::Operator(op_string))
    }

    fn read_identifier(&mut self) -> Result<Token, Error> {
        let special_id_chars = "?!-<>=_";
        let id = self.read_while(|ch| match ch {
            '0'...'9' | 'a'...'z' | 'A'...'Z' => true,
            _ => special_id_chars.contains(ch),
        });

        if self.keywords.contains(&id.as_str()) {
            return Ok(Token::Keyword(id));
        }
        Ok(Token::Variable(id))
    }

    fn read_number(&mut self) -> Result<Token, Error> {
        let mut dotted = false;
        let mut digits = String::new();

        for (i, ch) in self.input[self.ind..].iter().enumerate() {
            match *ch {
                '0'...'9' => digits.push(self.input[self.ind + i]),
                '.' => {
                    if dotted {
                        break;
                    }
                    dotted = true;
                    digits.push(self.input[self.ind + i])
                }
                _ => break,
            }
        }

        for _ in 0..digits.len() {
            self.next_char();
        }

        if digits.contains('.') {
            match digits.parse::<f32>() {
                Ok(floating) => Ok(Token::FloatingPoint(floating)),
                Err(err) => Err(self.get_error(format!("Error parsing float: {}", err))),
            }
        } else {
            match digits.parse::<i32>() {
                Ok(integral) => Ok(Token::Integral(integral)),
                Err(err) => Err(self.get_error(format!("Error parsing integer: {}", err))),
            }
        }
    }

    fn read_string(&mut self) -> Result<Token, Error> {
        let mut ret_str = String::new();
        let mut escaped = false;
        self.next_char(); // consume opening '"'

        for ch in self.input[self.ind..].iter() {
            if escaped {
                ret_str.push(*ch);
                escaped = false;
            } else if *ch == '\\' {
                escaped = true;
            } else if *ch == '"' {
                break;
            } else {
                ret_str.push(*ch);
            }
        }

        for _ in 0..ret_str.len() {
            self.next_char();
        }
        self.next_char();

        Ok(Token::StringLiteral(ret_str))
    }

    fn skip_comment(&mut self) {
        self.read_while(|ch| ch != '\n');
        self.next_char(); // consume newline
    }

    fn consume_whitespace(&mut self) {
        self.read_while(|ch| ch.is_whitespace());
    }

    fn read_while<F>(&mut self, func: F) -> String
    where
        F: Fn(char) -> bool,
    {
        let mut ret_str = String::new();

        for ch in self.input[self.ind..].iter() {
            if func(*ch) {
                ret_str.push(*ch)
            } else {
                break;
            }
        }

        for _ in 0..ret_str.len() {
            self.next_char();
        }

        ret_str
    }

    pub fn get_error(&self, msg: String) -> Error {
        Error {
            msg,
            line: self.line,
            col: self.col,
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_lex_variable() {
        let mut lexer = Lexer::new("varname");

        assert_eq!(
            lexer.get_token().unwrap(),
            Token::Variable(String::from("varname"))
        );
        assert_eq!(lexer.get_token().unwrap(), Token::EOF);
    }

    #[test]
    fn test_lex_keyword() {
        let mut lexer = Lexer::new("if");

        assert_eq!(
            lexer.get_token().unwrap(),
            Token::Keyword(String::from("if"))
        );
        assert_eq!(lexer.get_token().unwrap(), Token::EOF);
    }

    #[test]
    fn test_lex_string_literal() {
        let mut lexer = Lexer::new("\"string literal wow\"");

        assert_eq!(
            lexer.get_token().unwrap(),
            Token::StringLiteral(String::from("string literal wow"))
        );
        assert_eq!(lexer.get_token().unwrap(), Token::EOF);
    }

    #[test]
    fn test_lex_integral() {
        let mut lexer = Lexer::new("22312");

        assert_eq!(lexer.get_token().unwrap(), Token::Integral(22312));
        assert_eq!(lexer.get_token().unwrap(), Token::EOF);
    }

    #[test]
    fn test_lex_floating_point() {
        let mut lexer = Lexer::new("22.312");

        assert_eq!(lexer.get_token().unwrap(), Token::FloatingPoint(22.312));
        assert_eq!(lexer.get_token().unwrap(), Token::EOF);

        let mut lexer = Lexer::new("22.312.2");

        assert_eq!(lexer.get_token().unwrap(), Token::FloatingPoint(22.312));
        assert!(!lexer.eof());
    }

    #[test]
    fn test_lex_delimiter() {
        let mut lexer = Lexer::new(")");

        assert_eq!(lexer.get_token().unwrap(), Token::Delimiter(')'));
        assert_eq!(lexer.get_token().unwrap(), Token::EOF);
    }

    #[test]
    fn test_lex_empty() {
        let mut lexer = Lexer::new("");

        assert_eq!(lexer.get_token().unwrap(), Token::EOF);
    }

    #[test]
    fn test_peek_and_get_token() {
        let mut lexer = Lexer::new("abc\"bc\"");

        // Peek should be idempotent
        assert_eq!(lexer.peek().unwrap(), Token::Variable(String::from("abc")));
        assert_eq!(lexer.peek().unwrap(), Token::Variable(String::from("abc")));

        // get_token should equal the last result of peek
        assert_eq!(
            lexer.get_token().unwrap(),
            Token::Variable(String::from("abc"))
        );

        assert_eq!(
            lexer.peek().unwrap(),
            Token::StringLiteral(String::from("bc"))
        );
        assert_eq!(
            lexer.get_token().unwrap(),
            Token::StringLiteral(String::from("bc"))
        );

        assert_eq!(lexer.peek().unwrap(), Token::EOF);
        assert_eq!(lexer.get_token().unwrap(), Token::EOF);
        assert!(lexer.eof());
    }
}
