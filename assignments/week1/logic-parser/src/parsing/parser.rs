use crate::parsing::errors::ParserError;
use crate::parsing::lexer::{Token, TokenKind};

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
}

impl<'a> Parser<'a> {
    pub fn new(tokens: &Vec<Token>) -> Parser {
        Parser { tokens, pos: 0 }
    }

    /// Logic expressions parser
    /// ```md
    /// expr: term ((=>) term)
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
        let next_token = match self.consume() {
            Some(t) => t,
            None => return Err(
                ParserError::UnexpectedEOF("Proposition expected".into())
            ),
        };

        match &next_token.kind {
            TokenKind::Identifier(name) => {
                Ok(ASTNode::Identifier(name.to_owned()))
            },
            TokenKind::Literal(boolean) => {
                Ok(ASTNode::Literal(*boolean))
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
                    Err(ParserError::UnexpectedToken("R_PAREN expected".into()))
                }
            },
            TokenKind::CloseParen => {
                Err(ParserError::UnexpectedToken("Unexpected R_PAREN".into()))
            },
            other => {
                Err(ParserError::UnexpectedToken(format!("Unexpected Token {other}").into()))
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
    pub fn as_str(&self) -> String {
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
        }
    }
}

#[cfg(test)]
mod test {
    use std::error::Error;
    use crate::lexer::Lexer;
    use crate::parser::Parser;

    #[test]
    fn json_rendered_properly() -> Result<(), Box<dyn Error>> {
        use assert_json::assert_json;
        let tokens = Lexer::new("((p)) => (q & ~(r))").parse()?;
        let ast = Parser::new(&tokens).parse()?;
        let result = ast.as_json();

        assert_json!(result.as_str(), {
            "type": "operator",
            "name": "implies",
            "left": {
                "type": "identifier",
                "name": "p"
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
}
