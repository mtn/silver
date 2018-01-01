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

    pub fn parse_top_level(&mut self) -> ASTNode {
        let mut program: Vec<ASTNode> = Vec::new();

        while !self.lexer.eof() {
            program.push(self.parse_expression());
            if !self.lexer.eof() {
                self.consume(Token::Delimiter(';'));
            }
        }

        ASTNode::Program(program)
    }

    fn consume(&mut self, token: Token) -> Result<Token, Error> {
        let next = self.lexer.get_token();
        if let Ok(tok) = next {
            if token == tok
        }


    }

    fn parse_expression(&mut self) -> ASTNode {
        unimplemented!();
    }

    fn parse_conditional(&mut self) -> ASTNode {
        unimplemented!();
    }

    fn parse_atom(&mut self) -> ASTNode {
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
