use super::lexer;

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
    tokens: lexer::Lexer<'a>,
}

impl <'a> Parser <'a> {
    pub fn new() -> Parser<'a> {
        unimplemented!();
    }

    pub fn parse_top_level(&mut self) -> ASTNode {
        unimplemented!();
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
