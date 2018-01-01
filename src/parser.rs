use super::lexer;
use super::lexer::Token;

use super::util::Error;

#[derive(Debug)]
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
    pub lexer: lexer::Lexer<'a>,
}

impl <'a> Parser <'a> {
    pub fn parse(&mut self) {
        let res = self.parse_delimited(Token::Delimiter('('), Token::Delimiter(','), Token::Delimiter(')'), Self::parse_variable_name);
        println!("res {:?}", res);
    }

    pub fn parse_top_level(&mut self) -> Result<ASTNode, Error> {
        let mut program: Vec<ASTNode> = Vec::new();

        while !self.lexer.eof() {
            match self.parse_expression() {
                Ok(exp) => program.push(exp),
                Err(err) => return Err(err)
            }
            if !self.lexer.eof() {
                self.consume(Token::Delimiter(';'))?;
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
                       end: Token, parse_function: F) -> Result<Vec<ASTNode>, Error>
        where F: Fn(&mut Parser<'a>) -> Result<ASTNode, Error>
    {
        self.consume(start)?;

        let mut first = true;
        let mut terms: Vec<ASTNode> = Vec::new();

        while !self.lexer.eof() {
            if self.lexer.peek()? == end {
                break;
            }

            if first {
                first = false;
            } else {
                self.consume(separator.clone())?;
            }

            if self.lexer.peek()? == end {
                break;
            }

            terms.push(parse_function(self)?)
        }

        Ok(terms)
    }

    fn parse_expression(&mut self) -> Result<ASTNode, Error> {
        unimplemented!();
        // self.parse_delimited(Token::EOF, Token::EOF, Token::EOF, Self::parse_conditional)
    }

    fn parse_conditional(&mut self) -> Result<ASTNode, Error> {
        unimplemented!();
    }

    fn parse_atom(&mut self) -> Result<ASTNode, Error> {
        unimplemented!();
    }

    fn parse_variable_name(&mut self) -> Result<ASTNode, Error> {
        let var_token = self.lexer.get_token()?;
        match var_token {
            Token::Variable(ref name) => Ok(ASTNode::StringLiteral(name.clone())),
            e => Err(self.lexer.get_error(format!("Expected type variable, got {:?}", e)))
        }
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
