use super::lexer;
use super::lexer::Token;

use super::util::Error;

#[derive(Debug)]
pub enum ASTNode {
    Integer(i32),
    Float(f32),
    StringLiteral(String),
    Boolean(bool),

    Name(String),
    Variable { name: String, def: Box<ASTNode> },
    Function { args: Vec<ASTNode>, body: Box<ASTNode> },

    Invocation { func: Box<ASTNode>, args: Vec<ASTNode> },
    Binary { op: lexer::Token, lhs: Box<ASTNode>, rhs: Box<ASTNode> },
    Block { vars: Vec<ASTNode>, body: Box<ASTNode> },

    Sequence(Vec<ASTNode>)
}

pub struct Parser <'a> {
    pub lexer: lexer::Lexer<'a>,
}

impl <'a> Parser <'a> {
    pub fn parse(&mut self) {
        // let res  = self.parse_inv_or_expr();
        // println!("res {:?}", res.unwrap());
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

        Ok(ASTNode::Sequence(program))
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
    }

    fn parse_conditional(&mut self) -> Result<ASTNode, Error> {
        unimplemented!();
    }

    fn parse_atom(&mut self) -> Result<ASTNode, Error> {
        self.parse_inv_or_expr(Self::parse_atom_helper)
    }

    fn parse_atom_helper(&mut self) -> Result<ASTNode, Error> {
        match self.lexer.peek()? {
            Token::Delimiter('(') => {
                self.consume(Token::Delimiter('('));
                let exp = self.parse_expression();
                self.consume(Token::Delimiter(')'));

                exp
            },
            Token::Delimiter('{') => self.parse_program(),
            Token::Keyword(ref kw) => {
                match kw.as_str() {
                    "if" => self.parse_conditional(),
                    "true" | "false" => self.parse_bool(),
                    "fn" => self.parse_declaration(),
                    "let" => self.parse_let(),
                    _ => Ok(ASTNode::Integer(3)),
                }
            },
            _ => {
                let next = self.lexer.get_token();
                match next? {
                    Token::Variable(ref name) => Ok(ASTNode::Name(name.clone())),
                    Token::Integral(val) => Ok(ASTNode::Integer(val)),
                    Token::FloatingPoint(val) => Ok(ASTNode::Float(val)),
                    Token::StringLiteral(ref val) => Ok(ASTNode::StringLiteral(val.clone())),
                    _ => Err(self.lexer.get_error(String::from(
                                "Unexpected element in parse_atom"))),
                }
            }
        }
    }

    fn parse_binary(&mut self) -> Result<ASTNode, Error> {
        unimplemented!();
    }

    fn parse_let(&mut self) -> Result<ASTNode, Error> {
        unimplemented!();
    }

    fn parse_declaration(&mut self) -> Result<ASTNode, Error> {
        self.consume(Token::Keyword(String::from("fn")));

        Ok(ASTNode::Function {
            args: self.parse_delimited(Token::Delimiter('('), Token::Delimiter(','),
                                       Token::Delimiter(')'), Self::parse_variable_name)?,
            body: Box::new(self.parse_program()?)
        })
    }

    // Returns either an invocation or an expression, depending on what follows
    fn parse_inv_or_expr<F>(&mut self, parse_function: F) -> Result<ASTNode, Error>
        where F: Fn(&mut Parser<'a>) -> Result<ASTNode, Error>
    {
        let expr = parse_function(self);

        if Token::Delimiter('(') == self.lexer.peek()? {
            return Ok(ASTNode::Invocation {
                func: Box::new(expr?),
                args: self.parse_delimited(Token::Delimiter('('), Token::Delimiter(','),
                                           Token::Delimiter(')'), Self::parse_expression)?
            })
        }

        expr
    }

    fn parse_variable_name(&mut self) -> Result<ASTNode, Error> {
        match self.lexer.get_token()? {
            Token::Variable(ref name) => Ok(ASTNode::Name(name.clone())),
            e => Err(self.lexer.get_error(format!("Expected type variable, got {:?}", e)))
        }
    }

    fn parse_bool(&mut self) -> Result<ASTNode, Error>  {
        match self.lexer.get_token()? {
            Token::Keyword(ref val) => {
                match val.as_str() {
                    "true" => Ok(ASTNode::Boolean(true)),
                    "false" => Ok(ASTNode::Boolean(false)),
                    e => Err(self.lexer.get_error(format!("Expected type boolean, got {:?}",
                                                          e)))
                }
            },
            e => Err(self.lexer.get_error(format!("Expected type boolean, got {:?}", e)))
        }
    }

    fn parse_program(&mut self) -> Result<ASTNode, Error> {
        unimplemented!();
    }

    fn get_precedence(node: ASTNode) -> u32 {
        if let ASTNode::Binary { op, lhs: _, rhs: _ } = node {
            if let Token::Operator(ref kind) = op {
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
