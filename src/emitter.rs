use super::lexer;
use super::lexer::Token;

use super::parser;
use super::parser::ASTNode;

use super::util::Error;


pub struct Emitter<'a> {
    pub parser: parser::Parser<'a>
}

impl<'a> Emitter<'a> {
    fn emit(&self, ast: ASTNode) -> Result<String, Error> {
        match ast {
            ASTNode::Integer(val)
                => Ok(val.to_string()),
            ASTNode::Float(val)
                => Ok(val.to_string()),
            ASTNode::StringLiteral(val)
                => Ok(format!("\"{}\"", val)),
            ASTNode::Boolean(val)
                => Ok(val.to_string()),
            ASTNode::Name(val)
                => Ok(val),
            ASTNode::Function { name, args, body }
                => Self::emit_function(name, args, body),
            ASTNode::Invocation { func, args }
                => Self::emit_invocation(func, args),
            ASTNode::Conditional { cond, if_body, else_body }
                => Self::emit_conditional(cond, if_body, else_body),
            ASTNode::Binary { op, lhs, rhs }
                => Self::emit_binary(op, lhs, rhs),
            ASTNode::Sequence(vec)
                => Self::emit_sequence(vec),
            _ => Err(self.parser.lexer.get_error(String::from(
                        "Unexpected node type encountered during code emission"),
            ))
        }
    }

    fn emit_function(name: Box<Option<ASTNode>>, args: Vec<ASTNode>,
                     body: Box<ASTNode>) -> Result<String, Error> {

        unimplemented!();
    }

    fn emit_invocation(func: Box<ASTNode>, args: Vec<ASTNode>)
        -> Result<String, Error>
    {
        unimplemented!();
    }

    fn emit_conditional(cond: Box<ASTNode>, if_body: Box<ASTNode>,
                        else_body: Box<Option<ASTNode>>) -> Result<String, Error>
    {
        unimplemented!();
    }

    fn emit_binary(op: Token, lhs: Box<ASTNode>, rhs: Box<ASTNode>)
        -> Result<String, Error>
    {
        unimplemented!();
    }

    fn emit_sequence(exprs: Vec<ASTNode>) -> Result<String, Error> {
        unimplemented!();
    }
}
