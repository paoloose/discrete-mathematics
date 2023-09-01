use crate::errors::ParserError;
use crate::lexing::token::{Token, TokenKind};
use ParserError::{UnexpectedToken, UnexpectedEOF};

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
    ///
    /// ```yaml
    /// expr: term [(<-> | ->) expr]
    /// term: prop [(|| | &&) term]
    /// prop: [~] (true | false | "name" | LPAREN expr RPAREN)
    /// ```
    pub fn parse(&mut self) -> Result<ASTNode> {
        let ast = self.parse_expression()?;
        // If expression was not completedly parsed, return an error
        if let Some(t) = self.consume() {
            return Err(UnexpectedToken(format!("'{t}'", t=t.kind), t.span))
        }
        Ok(ast)
    }

    fn parse_expression(&mut self) -> Result<ASTNode> {
        let l_term = self.parse_term()?;

        match self.peek().cloned() {
            Some(TokenKind::Implies) => {
                self.consume();
                Ok(ASTNode::Implies(
                    Box::new(l_term),
                    Box::new(self.parse_expression()?))
                )
            },
            Some(TokenKind::IfAndOnlyIf) => {
                self.consume();
                Ok(ASTNode::IfAndOnlyIf(
                    Box::new(l_term),
                    Box::new(self.parse_expression()?))
                )
            },
            Some(_) => Ok(l_term),
            None => Ok(l_term)
        }
    }

    fn parse_term(&mut self) -> Result<ASTNode> {
        let l_term = self.parse_proposition()?;

        match self.peek().cloned() {
            Some(TokenKind::And) => {
                self.consume();
                Ok(ASTNode::And(
                    Box::new(l_term),
                    Box::new(self.parse_term()?))
                )
            },
            Some(TokenKind::Or) => {
                self.consume();
                Ok(ASTNode::Or(
                    Box::new(l_term),
                    Box::new(self.parse_term()?))
                )
            },
            Some(_) => Ok(l_term),
            None => Ok(l_term)
        }
    }

    fn parse_proposition(&mut self) -> Result<ASTNode> {
        let next_token = match self.consume().cloned() {
            Some(t) => t,
            None => {
                // Gets the last token span, otherwise (start: 0, end: 0)
                let last_span = self.tokens.last().map(|t| t.span).unwrap_or((0, 0).into());
                return Err(
                    UnexpectedEOF("Expected [~] (true | false | variable | (...))".into(), last_span)
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

    fn peek(&self) -> Option<&TokenKind> {
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
    use super::*;
    use crate::lexing::Lexer;
    use std::error::Error;
    use std::result::Result;

    #[test]
    fn complex_json_rendered_properly() -> Result<(), Box<dyn Error>> {
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

    #[test]
    fn unmatched_paren_left_results_on_error() {
        let tokens = Lexer::new("((a => b) <=> c").parse().unwrap();
        let parse_error = Parser::new(&tokens).parse().unwrap_err();

        match parse_error {
            ParserError::UnexpectedToken(_, span) => {
                assert_eq!(span, (0, 1).into())
            },
            _ => unreachable!()
        }
    }

    #[test]
    fn unmatched_paren_right_results_on_error() {
        let tokens = Lexer::new("(a => b)) <=> c").parse().unwrap();

        let parse_error = Parser::new(&tokens).parse().unwrap_err();
        match parse_error {
            ParserError::UnexpectedToken(_, span) => {
                assert_eq!(span, (8, 9).into())
            },
            _ => unreachable!()
        }
    }
}
