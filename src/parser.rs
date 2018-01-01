use super::lexer;
use super::lexer::Token;

use super::util::Error;

pub enum ASTNode {
    Integer(i32),
    Float(f32),
    StringLiteral(String),
    Boolean(bool),

    Variable { name: String, def: Box<ASTNode> },
    Function { args: Vec<ASTNode>, body: Box<ASTNode> },
    Invocation { func: Box<ASTNode>, args: Vec<ASTNode> },
    Binary { op: lexer::Token, lhs: Box<ASTNode>, rhs: Box<ASTNode> },
    Block { vars: Vec<ASTNode>, body: Box<ASTNode> },

    Program(Vec<ASTNode>)
}

pub struct Parser <'a> {
    lexer: lexer::Lexer<'a>,
}

impl <'a> Parser <'a> {
    pub fn new(lexer: lexer::Lexer<'a>) -> Parser<'a> {
        Parser { lexer}
    }

    pub fn parse_top_level(&mut self) -> Result<ASTNode, Error> {
        let mut program: Vec<ASTNode> = Vec::new();

        while !self.lexer.eof() {
            match self.parse_expression() {
                Ok(exp) => program.push(exp),
                Err(err) => return Err(err)
            }
            if !self.lexer.eof() {
                self.consume(Token::Delimiter(';'));
            }
        }

        Ok(ASTNode::Program(program))
    }

    fn consume(&mut self, token: Token) -> Result<Token, Error> {
        let next = self.lexer.get_token();
        if let Ok(tok) = next {
            if token == tok {
                Ok(tok)
            } else {
                Err(self.lexer.get_error(format!("Unexpected token, expected {:?} given {:?}",
                                                 tok, token)))
            }
        } else {
            Err(self.lexer.get_error(String::from("get_token failed")))
        }
    }

    fn parse_delimited<F>(&mut self, start: Token, separator: Token,
                       end: Token, parse_function: F) -> Result<ASTNode, Error>
        where F: Fn(&mut Parser<'a>) -> Result<ASTNode, Error>
    {
        self.consume(start);

        let mut first = true;
        while !self.lexer.eof() {
            if let Ok(next) = self.lexer.get_token() {

            } else {
            }
        }

        Ok(ASTNode::Integer(5))
    }

    fn parse_expression(&mut self) -> Result<ASTNode, Error> {
        self.parse_delimited(Token::EOF, Token::EOF, Token::EOF, Self::parse_conditional)
    }

    fn parse_conditional(&mut self) -> Result<ASTNode, Error> {
        unimplemented!();
    }

    fn parse_atom(&mut self) -> Result<ASTNode, Error> {
        unimplemented!();
    }

    fn get_precedence(node: ASTNode) -> u32 {
        if let ASTNode::Binary { op, lhs: _, rhs: _ } = node {
            if let lexer::Token::Operator(ref kind) = op {
                match kind.as_str() {
                    "=" => 1,
                    "||" => 2,
                    "&&" => 3,
                    "<"|"<="|">"|">="|"=="|"!=" => 4,
                    "+"|"-" => 5,
                    "*"|"/"|"%" => 6,
                    _ => panic!("Unexpected operator on binary ASTNode"),
                };
            } else {
                panic!("Improperly formatted binary ASTNode");
            }
        }

        panic!("Unexpected call to get_precedence");
    }
}
