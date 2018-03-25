use super::lexer::Token;
use super::parser::ASTNode;

use std::fmt;

#[derive(Debug, PartialEq)]
pub struct Error {
    pub msg: String,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Error: {}", self.msg)
    }
}

pub fn emit(ast: ASTNode) -> Result<String, Error> {
    match ast {
        ASTNode::Integer(val) => Ok(val.to_string()),
        ASTNode::Float(val) => Ok(val.to_string()),
        ASTNode::StringLiteral(val) => Ok(format!("\"{}\"", val)),
        ASTNode::Boolean(val) => Ok(val.to_string()),
        ASTNode::Name(val) => Ok(val),
        ASTNode::Function { name, args, body } => emit_function(name, args, body),
        ASTNode::Invocation { func, args } => emit_invocation(func, args),
        ASTNode::Conditional {
            cond,
            if_body,
            else_body,
        } => emit_conditional(cond, if_body, else_body),
        ASTNode::Binary { op, lhs, rhs } => emit_binary(op, lhs, rhs),
        ASTNode::Sequence(vec) => emit_sequence(vec),
    }
}

fn emit_function(
    name: Box<Option<ASTNode>>,
    args: Vec<ASTNode>,
    body: Box<ASTNode>,
) -> Result<String, Error> {
    let mut function = String::from("function ");
    if let Some(ASTNode::Name(ref name_str)) = *name {
        function.push_str(name_str);
    }
    function.push('(');
    function.push_str(emit_map_helper(args, String::from(","))?.as_str());
    function.push_str(") { return (");
    function.push_str(emit(*body)?.as_str());
    function.push_str(") }");

    Ok(function)
}

fn emit_invocation(func: Box<ASTNode>, args: Vec<ASTNode>) -> Result<String, Error> {
    let mut invocation = String::new();

    invocation.push_str(emit(*func)?.as_str());
    invocation.push('(');
    invocation.push_str(emit_map_helper(args, String::from(","))?.as_str());
    invocation.push(')');

    Ok(invocation)
}

// Maps emit over a vector of nodes, joining with the delimiter as a separator
fn emit_map_helper(nodes: Vec<ASTNode>, delimiter: String) -> Result<String, Error> {
    let mut err = false;
    let name_vec: Vec<String> = nodes
        .iter()
        .map(|node| {
            if let Ok(res) = emit(node.clone()) {
                return res;
            } else {
                err = true;
                return String::new();
            }
        })
        .collect();

    if err {
        return Err(Error {
            msg: String::from("Emit failed for a node in the list"),
        });
    }

    Ok(name_vec.join(delimiter.as_str()))
}

// Because conditional is an expression, it is equivalent to JS ternary
fn emit_conditional(
    cond: Box<ASTNode>,
    if_body: Box<ASTNode>,
    else_body: Box<Option<ASTNode>>,
) -> Result<String, Error> {
    let mut conditional = String::from("(");

    conditional.push_str(emit(*cond)?.as_str());

    // Only false is falsey
    conditional.push_str("!== false ? ");

    conditional.push_str(emit(*if_body)?.as_str());

    conditional.push_str(" : ");

    if let Some(node) = *else_body {
        conditional.push_str(emit(node)?.as_str());
    } else {
        conditional.push_str(emit(ASTNode::Boolean(false))?.as_str());
    }

    conditional.push(')');

    Ok(conditional)
}

fn emit_binary(op: Token, lhs: Box<ASTNode>, rhs: Box<ASTNode>) -> Result<String, Error> {
    if let Token::Operator(op) = op {
        return Ok(format!("({} {} {})", emit(*lhs)?, op, emit(*rhs)?));
    }

    Err(Error {
        msg: String::from("Malformed binary node"),
    })
}

fn emit_sequence(exprs: Vec<ASTNode>) -> Result<String, Error> {
    Ok(emit_map_helper(exprs, String::from(","))?)
}
