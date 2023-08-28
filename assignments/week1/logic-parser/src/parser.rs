use crate::lexer::{Token, TokenKind};
use crate::errors::ParserError;

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
    /// ```
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
