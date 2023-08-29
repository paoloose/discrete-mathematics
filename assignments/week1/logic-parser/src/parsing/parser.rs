use crate::errors::ParserError;
use crate::lexing::token::{Token, TokenKind};

pub type Result<T> = std::result::Result<T, ParserError>;

#[derive(Debug)]
pub struct Parser<'a> {
    tokens: &'a Vec<Token>,
    pos: usize
}

#[derive(Debug)]
pub enum ASTNode {
    Identifier(String),
    Literal(bool),
    Not(Box<ASTNode>),
    And(Box<ASTNode>, Box<ASTNode>),
    Or(Box<ASTNode>, Box<ASTNode>),
    Implies(Box<ASTNode>, Box<ASTNode>),
    IfAndOnlyIf(Box<ASTNode>, Box<ASTNode>),
}

impl Parser<'_> {
    pub fn new(tokens: &Vec<Token>) -> Parser {
        Parser { tokens, pos: 0 }
    }

    /// Logic expressions parser
    /// ```md
    /// expr: term ((=> | <=>) term)
    /// term: prop ((|, &) prop)
    /// prop: (~) (true | false | "name" | LPAREN expr RPAREN)
    /// ```
    pub fn parse(&mut self) -> Result<ASTNode> {
        let ast = self.parse_expression()?;
        Ok(ast)
    }

    fn parse_expression(&mut self) -> Result<ASTNode> {
        let l_term = self.parse_term()?;

        match self.peek() {
            Some(TokenKind::Implies) => {
                self.consume();
                let r_term = self.parse_term()?;
                Ok(ASTNode::Implies(Box::new(l_term), Box::new(r_term)))
            },
            Some(TokenKind::IfAndOnlyIf) => {
                self.consume();
                let r_term = self.parse_term()?;
                Ok(ASTNode::IfAndOnlyIf(Box::new(l_term), Box::new(r_term)))
            }
            Some(_) => Ok(l_term),
            None => Ok(l_term)
        }
    }

    fn parse_term(&mut self) -> Result<ASTNode> {
        let left = self.parse_proposition()?;

        match self.peek() {
            Some(TokenKind::And) => {
                self.consume();
                Ok(ASTNode::And(
                    Box::new(left),
                    Box::new(self.parse_proposition()?))
                )
            },
            Some(TokenKind::Or) => {
                self.consume();
                Ok(ASTNode::Or(
                    Box::new(left),
                    Box::new(self.parse_proposition()?))
                )
            },
            Some(_) => Ok(left),
            None => Ok(left)
        }
    }

    fn parse_proposition(&mut self) -> Result<ASTNode> {
        use ParserError::UnexpectedToken;

        let next_token = match self.consume().cloned() {
            Some(t) => t,
            None => {
                // Gets the last token span, otherwise (start: 0, end: 0)
                let last_span = self.tokens.last().map(|t| t.span.clone()).unwrap_or((0, 0).into());
                return Err(
                    ParserError::UnexpectedEOF("Expected [~] (true | false | variable | (...))".into(), last_span)
                )
            },
        };

        match next_token.kind {
            TokenKind::Identifier(name) => {
                Ok(ASTNode::Identifier(name.to_owned()))
            },
            TokenKind::Literal(boolean) => {
                Ok(ASTNode::Literal(boolean))
            },
            TokenKind::Not => {
                let prop = self.parse_proposition()?;
                Ok(ASTNode::Not(Box::new(prop)))
            },
            TokenKind::OpenParen => {
                let expr = self.parse_expression()?;
                if let Some(TokenKind::CloseParen) = self.peek() {
                    self.consume();
                    Ok(expr)
                }
                else {
                    Err(UnexpectedToken("R_PAREN expected".into(), next_token.span))
                }
            },
            TokenKind::CloseParen => {
                Err(UnexpectedToken("R_PAREN".into(), next_token.span))
            },
            other @ (TokenKind::And | TokenKind::Or | TokenKind::Implies | TokenKind::IfAndOnlyIf) => {
                Err(UnexpectedToken(format!("'{other}'"), next_token.span))
            }
        }
    }

    fn consume(&mut self) -> Option<&Token> {
        let token = self.tokens.get(self.pos);
        if token.is_some() {
            self.pos += 1;
        }
        token
    }

    fn peek(&mut self) -> Option<&TokenKind> {
        self.tokens.get(self.pos).map(|t| &t.kind)
    }
}

impl ASTNode {
    pub fn as_string(&self) -> String {
        format!("{:#?}", self)
    }

    pub fn as_json(&self) -> String {
        match self {
            ASTNode::Identifier(s) => {
                format!(r###"{{
                    "type": "identifier",
                    "name": "{s}"
                }}"###)
            },
            ASTNode::Literal(boolean) => {
                format!(r###"{{
                    "type": "literal",
                    "value": {boolean}
                }}"###)
            },
            ASTNode::Not(expr) => {
                format!(r###"{{
                    "type": "operator",
                    "name": "not",
                    "expr": {expr}
                }}"###, expr=expr.as_json())
            },
            ASTNode::And(left, right) => {
                format!(r###"{{
                    "type": "operator",
                    "name": "and",
                    "left": {left},
                    "right": {right}
                }}"###, left=left.as_json(), right=right.as_json())
            },
            ASTNode::Or(left, right) => {
                format!(r###"{{
                    "type": "operator",
                    "name": "or",
                    "left": {left},
                    "right": {right}
                }}"###, left=left.as_json(), right=right.as_json())
            },
            ASTNode::Implies(left, right) => {
                format!(r###"{{
                    "type": "operator",
                    "name": "implies",
                    "left": {left},
                    "right": {right}
                }}"###, left=left.as_json(), right=right.as_json())
            },
            ASTNode::IfAndOnlyIf(left, right) => {
                format!(r###"{{
                    "type": "operator",
                    "name": "iff",
                    "left": {left},
                    "right": {right}
                }}"###, left=left.as_json(), right=right.as_json())
            }
        }
    }
}

#[cfg(test)]
mod test {
    use std::error::Error;
    use crate::lexing::Lexer;
    use crate::parsing::Parser;

    #[test]
    fn json_rendered_properly() -> Result<(), Box<dyn Error>> {
        use assert_json::assert_json;
        let tokens = Lexer::new("((p || q)) => (q & ~(r))").parse()?;
        let ast = Parser::new(&tokens).parse()?;
        let result = ast.as_json();

        assert_json!(result.as_str(), {
            "type": "operator",
            "name": "implies",
            "left": {
                "type": "operator",
                "name": "or",
                "left": {
                    "type": "identifier",
                    "name": "p"
                },
                "right": {
                    "type": "identifier",
                    "name": "q"
                }
            },
            "right": {
                "type": "operator",
                "name": "and",
                "left": {
                    "type": "identifier",
                    "name": "q"
                },
                "right": {
                    "type": "operator",
                    "name": "not",
                    "expr": {
                        "type": "identifier",
                        "name": "r"
                    }
                }
            }
        });
        Ok(())
    }

    #[test]
    fn multiple_negation_works() -> Result<(), Box<dyn Error>> {
        use assert_json::assert_json;
        let tokens = Lexer::new("~~~negate_me").parse()?;
        let ast = Parser::new(&tokens).parse()?;
        let result = ast.as_json();

        assert_json!(result.as_str(), {
            "type": "operator",
            "name": "not",
            "expr": {
                "type": "operator",
                "name": "not",
                "expr": {
                    "type": "operator",
                    "name": "not",
                    "expr": {
                        "type": "identifier",
                        "name": "negate_me"
                    }
                }
            }
        });
        Ok(())
    }

    #[test]
    fn iff_and_implies_work_together() -> Result<(), Box<dyn Error>> {
        use assert_json::assert_json;
        let tokens = Lexer::new("(a => b) <=> c").parse()?;
        let ast = Parser::new(&tokens).parse()?;
        let result = ast.as_json();

        assert_json!(result.as_str(), {
            "type": "operator",
            "name": "iff",
            "left": {
                "type": "operator",
                "name": "implies",
                "left": {
                    "type": "identifier",
                    "name": "a"
                },
                "right": {
                    "type": "identifier",
                    "name": "b"
                }
            },
            "right": {
                "type": "identifier",
                "name": "c"
            }
        });
        Ok(())
    }

    #[test]
    fn alternative_syntax_work() -> Result<(), Box<dyn Error>> {
        use assert_json::assert_json;
        let tokens = Lexer::new("(a & b) && ((b | c) || b)").parse()?;
        let ast = Parser::new(&tokens).parse()?;
        let result = ast.as_json();

        assert_json!(result.as_str(), {
            "type": "operator",
            "name": "and",
            "left": {
                "type": "operator",
                "name": "and",
                "left": {
                    "type": "identifier",
                    "name": "a"
                },
                "right": {
                    "type": "identifier",
                    "name": "b"
                }
            },
            "right": {
                "type": "operator",
                "name": "or",
                "left": {
                    "type": "operator",
                    "name": "or",
                    "left": {
                        "type": "identifier",
                        "name": "b"
                    },
                    "right": {
                        "type": "identifier",
                        "name": "c"
                    }
                },
                "right": {
                    "type": "identifier",
                    "name": "b"
                }
            }
        });
        Ok(())
    }
}
