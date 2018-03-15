use super::lexer;
use super::lexer::Token;

use super::util::Error;

#[derive(Debug, Clone, PartialEq)]
pub enum ASTNode {
    Integer(i32),
    Float(f32),
    StringLiteral(String),
    Boolean(bool),

    Name(String),

    Function {
        name: Box<Option<ASTNode>>,
        args: Vec<ASTNode>,
        body: Box<ASTNode>
    },

    Invocation {
        func: Box<ASTNode>,
        args: Vec<ASTNode>
    },

    Conditional {
        cond: Box<ASTNode>,
        if_body: Box<ASTNode>,
        else_body: Box<Option<ASTNode>>
    },

    Binary {
        op: Token,
        lhs: Box<ASTNode>,
        rhs: Box<ASTNode>
    },

    Sequence(Vec<ASTNode>)
}

pub struct Parser <'a> {
    pub lexer: lexer::Lexer<'a>,
}

impl <'a> Parser <'a> {
    pub fn parse_top_level(&mut self) -> Result<ASTNode, Error> {
        let mut program: Vec<ASTNode> = Vec::new();

        while !self.lexer.eof() {
            match self.parse_expression() {
                Ok(exp) =>
                    program.push(exp),
                Err(err) =>
                    return Err(err)
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
                Err(self.lexer.get_error(format!(
                            "Unexpected token, expected {:?} given {:?}",
                            token, tok)))
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

        self.consume(end)?;
        Ok(terms)
    }

    fn parse_expression(&mut self) -> Result<ASTNode, Error> {
        self.parse_inv_or_expr(Self::parse_expression_helper)
    }

    fn parse_expression_helper(&mut self) -> Result<ASTNode, Error> {
        let next_atom = self.parse_atom()?;

        // Look ahead for operators
        self.parse_binary(next_atom, 0)
    }

    fn parse_conditional(&mut self) -> Result<ASTNode, Error> {
        self.consume(Token::Keyword(String::from("if")))?;

        let condition = self.parse_expression()?;

        if let Token::Keyword(ref kw) = self.lexer.peek()? {
            if kw == "then" {
                self.consume(Token::Keyword(String::from("then")))?;
            } else {
                return Err(self.lexer.get_error(
                        format!("Unexpected keyword {} after if, expected then", kw)));
            }
        }

        let if_body = self.parse_expression()?;
        let else_body: Option<ASTNode>;

        if let Token::Keyword(ref kw) = self.lexer.peek()? {
            if kw == "else" {
                self.consume(Token::Keyword(String::from("else")))?;
                else_body = Some(self.parse_expression()?);
            } else {
                return Err(self.lexer.get_error(
                        format!("Unexpected keyword {}, expected else or nothing", kw)));
            }
        } else {
            else_body = None;
        }

        Ok(ASTNode::Conditional {
            cond: Box::new(condition),
            if_body: Box::new(if_body),
            else_body: Box::new(else_body)
        })
    }

    fn parse_atom(&mut self) -> Result<ASTNode, Error> {
        self.parse_inv_or_expr(Self::parse_atom_helper)
    }

    fn parse_atom_helper(&mut self) -> Result<ASTNode, Error> {
        match self.lexer.peek()? {
            Token::Delimiter('(') => {
                self.consume(Token::Delimiter('('))?;
                let exp = self.parse_expression();
                self.consume(Token::Delimiter(')'))?;

                exp
            },
            Token::Delimiter('{') => self.parse_sequence(),
            Token::Keyword(ref kw) => {
                match kw.as_str() {
                    "if" =>
                        self.parse_conditional(),
                    "true" | "false" =>
                        self.parse_bool(),
                    "fn" =>
                        self.parse_declaration(),
                    _ => Err(self.lexer.get_error(format!("Unexpected keyword {}", kw)))
                }
            },
            _ => {
                let next = self.lexer.get_token();
                match next? {
                    Token::Variable(ref name) =>
                        Ok(ASTNode::Name(name.clone())),
                    Token::Integral(val) =>
                        Ok(ASTNode::Integer(val)),
                    Token::FloatingPoint(val) =>
                        Ok(ASTNode::Float(val)),
                    Token::StringLiteral(ref val) =>
                        Ok(ASTNode::StringLiteral(val.clone())),
                    _ => Err(self.lexer.get_error(
                                String::from("Unexpected element in parse_atom"))),
                }
            }
        }
    }

    // Accepts a binary ASTNode or a nonbinary node with precedence 0, and either
    // returns the expression (if it has higher precedence) or repeats, advancing one
    fn parse_binary(&mut self, lhs: ASTNode, lhs_prec: u32) -> Result<ASTNode, Error> {
        let next = self.lexer.peek();
        if let Token::Operator(ref op) = next? {
            self.lexer.get_token()?; // advance

            let rhs_prec = Self::get_precedence(op);
            if rhs_prec > lhs_prec {
                // Parse the next atom, which follows the rhs operator
                let next_atom = self.parse_atom()?;
                // Parse for subsequent binary. Either left has higher precedence, or we
                // advance right accumulating the lhs until there's only one term left
                let next_binary = self.parse_binary(next_atom, rhs_prec)?;

                return self.parse_binary(ASTNode::Binary {
                    op: Token::Operator(op.clone()),
                    lhs: Box::new(lhs),
                    rhs: Box::new(next_binary),
                }, lhs_prec)
            } else {
                return Ok(lhs)
            }
        }

        Ok(lhs)
    }

    fn parse_declaration(&mut self) -> Result<ASTNode, Error> {
        self.consume(Token::Keyword(String::from("fn")))?;

        Ok(ASTNode::Function {
            name: Box::new(match self.lexer.peek()? {
                Token::Variable(ref name) => {
                    self.lexer.get_token()?; // Consume the name
                    Some(ASTNode::Name(name.clone()))
                },
                _ => None
            }),
            args: {
                self.parse_delimited(Token::Delimiter('('),
                                           Token::Delimiter(','),
                                           Token::Delimiter(')'),
                                           Self::parse_variable_name)?
            },
            body: Box::new(self.parse_sequence()?)
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
                args: self.parse_delimited(Token::Delimiter('('),
                                           Token::Delimiter(','),
                                           Token::Delimiter(')'),
                                           Self::parse_expression)?
            })
        }

        expr
    }

    fn parse_variable_name(&mut self) -> Result<ASTNode, Error> {
        match self.lexer.get_token()? {
            Token::Variable(ref name) =>
                Ok(ASTNode::Name(name.clone())),
            e => Err(self.lexer.get_error(format!(
                        "Expected type variable, got {:?}", e)))
        }
    }

    fn parse_bool(&mut self) -> Result<ASTNode, Error>  {
        match self.lexer.get_token()? {
            Token::Keyword(ref val) => {
                match val.as_str() {
                    "true" =>
                        Ok(ASTNode::Boolean(true)),
                    "false" =>
                        Ok(ASTNode::Boolean(false)),
                    e => Err(self.lexer.get_error(format!(
                                "Expected type boolean, got {:?}", e)))
                }
            },
            e => Err(self.lexer.get_error(format!(
                        "Expected type boolean, got {:?}", e)))
        }
    }

    fn parse_sequence(&mut self) -> Result<ASTNode, Error> {
        let sequence = self.parse_delimited(Token::Delimiter('{'),
                                            Token::Delimiter(';'),
                                            Token::Delimiter('}'),
                                            Self::parse_expression)?;

        match sequence.len() {
            0 =>
                Ok(ASTNode::Boolean(false)), // empty sequences are falsey
            1 =>
                Ok(sequence[0].clone()),
            _ => Ok(ASTNode::Sequence(sequence))
        }
    }

    fn get_precedence(op: &str) -> u32 {
        match op {
            "=" => 1,
            "||" => 2,
            "&&" => 3,
            "<"|"<="|">"|">="|"=="|"!=" => 4,
            "+"|"-" => 5,
            "*"|"/"|"%" => 6,
            _ => panic!("Unexpected operator on binary ASTNode"),
        }
    }
}


#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_parse_empty() {
        let inp = "";
        let mut lexer = lexer::Lexer::new(inp);
        let mut parser = Parser { lexer };

        if let Ok(ASTNode::Sequence(ref vec)) = parser.parse_top_level() {
            assert_eq!(*vec, Vec::new())
        } else {
            panic!("Expected parse_top_level to return sequence")
        }
    }

    #[test]
    fn test_parse_primative_sequence() {
        let inp = "3; 3.1; \"stringliteralwow\"; true; false";
        let mut lexer = lexer::Lexer::new(inp);
        let mut parser = Parser { lexer };

        let expected: ASTNode = ASTNode::Sequence(vec![
                                    ASTNode::Integer(3),
                                    ASTNode::Float(3.1),
                                    ASTNode::StringLiteral(
                                        String::from("stringliteralwow")),
                                    ASTNode::Boolean(true),
                                    ASTNode::Boolean(false)
                                ]);

        if let Ok(res) = parser.parse_top_level() {
            assert_eq!(res, expected);
        } else {
            panic!("Primative sequence failed to parse");
        }
    }

    #[test]
    fn test_parse_name_sequence() {
        // Lots of extra whitespace, and none
        let inp = "foo;          bar;baz";
        let mut lexer = lexer::Lexer::new(inp);
        let mut parser = Parser { lexer };

        let expected = ASTNode::Sequence(vec![ASTNode::Name(String::from("foo")),
                                              ASTNode::Name(String::from("bar")),
                                              ASTNode::Name(String::from("baz"))]);

        if let Ok(res) = parser.parse_top_level() {
            assert_eq!(res, expected);
        } else {
            panic!("Name failed to parse");
        }
    }

    #[test]
    fn test_parse_if() {
        let inp = "if x then y";
        let mut lexer = lexer::Lexer::new(inp);
        let mut parser = Parser { lexer };

        let expected = ASTNode::Sequence(vec![
                           ASTNode::Conditional {
                               cond: Box::new(
                                   ASTNode::Name(String::from("x"))),
                                       if_body: Box::new(
                                           ASTNode::Name(String::from("y"))),
                                       else_body: Box::new(None)
                                   }
                       ]);

        if let Ok(res) = parser.parse_top_level() {
            assert_eq!(res, expected);
        } else {
            panic!("Conditional failed to parse");
        }
    }

    #[test]
    fn test_parse_invocation() {
        let inp = "x(a,b,   c)";
        let mut lexer = lexer::Lexer::new(inp);
        let mut parser = Parser { lexer };

        let expected = ASTNode::Sequence(vec![
                           ASTNode::Invocation {
                               func: Box::new(ASTNode::Name(String::from("x"))),
                               args: vec![
                                   ASTNode::Name(String::from("a")),
                                   ASTNode::Name(String::from("b")),
                                   ASTNode::Name(String::from("c")),
                               ]
                           }
                       ]);

        if let Ok(res) = parser.parse_top_level() {
            assert_eq!(res, expected);
        } else {
            panic!("Invocation failed to parse");
        }
    }

    #[test]
    fn test_parse_simple_binary() {
        let inp = "x = y";
        let mut lexer = lexer::Lexer::new(inp);
        let mut parser = Parser { lexer };

        let expected = ASTNode::Sequence(vec![
                           ASTNode::Binary {
                               op: lexer::Token::Operator(String::from("=")),
                               lhs: Box::new(ASTNode::Name(String::from("x"))),
                               rhs: Box::new(ASTNode::Name(String::from("y"))),
                           }
                       ]);

        if let Ok(res) = parser.parse_top_level() {
            assert_eq!(res, expected);
        } else {
            panic!(format!("Simple binary failed to parse"));
        }
    }

    #[test]
    fn test_parse_complex_parenthesized_binary() {
        let inp = "a = (b + c) * d";
        let mut lexer = lexer::Lexer::new(inp);
        let mut parser = Parser { lexer };

        let expected = ASTNode::Sequence(vec![
                           ASTNode::Binary {
                               op: lexer::Token::Operator(String::from("=")),
                               lhs: Box::new(ASTNode::Name(String::from("a"))),
                               rhs: Box::new(ASTNode::Binary {
                                        op: lexer::Token::Operator(String::from("*")),
                                        lhs: Box::new(ASTNode::Binary {
                                            op: lexer::Token::Operator(String::from("+")),
                                            lhs: Box::new(ASTNode::Name(String::from("b"))),
                                            rhs: Box::new(ASTNode::Name(String::from("c"))),
                                        }),
                                        rhs: Box::new(ASTNode::Name(String::from("d")))
                                        })
                           }
                       ]);

        if let Ok(res) = parser.parse_top_level() {
            assert_eq!(res, expected);
        } else {
            panic!(format!("Simple binary failed to parse"));
        }
    }

    #[test]
    fn test_function_declaration() {
        let inp = "fn a (b,c) {
                       if b {
                           c = b
                       } else {
                           b = c
                       };
                       b
                   }";
        let mut lexer = lexer::Lexer::new(inp);
        let mut parser = Parser { lexer };

        let expected = ASTNode::Sequence(vec![
                           ASTNode::Function {
                               name: Box::new(Some(ASTNode::Name(String::from("a")))),
                               args: vec![ASTNode::Name(String::from("b")),
                                          ASTNode::Name(String::from("c"))],
                               body: Box::new(ASTNode::Sequence(vec![
                                   ASTNode::Conditional {
                                       cond: Box::new(ASTNode::Name(String::from("b"))),
                                       if_body: Box::new(ASTNode::Binary {
                                           op: lexer::Token::Operator(String::from("=")),
                                           lhs: Box::new(ASTNode::Name(String::from("c"))),
                                           rhs: Box::new(ASTNode::Name(String::from("b"))),
                                       }),
                                       else_body: Box::new(Some(ASTNode::Binary {
                                           op: lexer::Token::Operator(String::from("=")),
                                           lhs: Box::new(ASTNode::Name(String::from("b"))),
                                           rhs: Box::new(ASTNode::Name(String::from("c"))),
                                       })),
                                   },
                                   ASTNode::Name(String::from("b")),
                               ])),
                           }
                       ]);

        if let Ok(res) = parser.parse_top_level() {
            assert_eq!(res, expected);
        } else {
            panic!(String::from("Function declaration failed to parse"));
        }
    }
}
