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
    Expression(Option<Box<ASTNode>>)
}

impl<'a> Parser<'a> {
    pub fn new(tokens: &Vec<Token>) -> Parser {
        Parser { tokens, pos: 0 }
    }
    pub fn parse(&mut self) -> Result<ASTNode> {
        let mut stack: Vec<ASTNode> = Vec::with_capacity(4);
        let mut ast = ASTNode::Expression(None);

        loop {
            let token = match self.peek() {
                Some(token) => token,
                None => break
            };

            match token {
                TokenKind::Identifier(ref p) => {
                    stack.push(ASTNode::Identifier(p.clone()))
                },
                TokenKind::Literal(ref l) => {
                    stack.push(ASTNode::Literal(l.clone()))
                },
                TokenKind::Not => {},
                TokenKind::And => {
                    // in the stack we should only have the left side of the
                    // expression
                    let left = stack.pop().unwrap();
                },
                TokenKind::Or => {},
                TokenKind::Implies => {},
                TokenKind::OpenParen => {},
                TokenKind::CloseParen => {},
            }
        }

        println!("{:#?}", stack);
        Ok(ASTNode::Literal(true))
    }

    fn next(&mut self) -> Option<&Token> {
        let token = self.tokens.get(self.pos);
        if token.is_some() {
            self.pos += 1;
        }
        token
    }

    fn peek(&mut self) -> Option<&TokenKind> {
        self.tokens.get(self.pos).map(|t| &t.kind)
    }

    // fn parse_identifier(&mut self) -> Result<ASTNode> {
    //     match self.next() {
    //         Some(Token { kind: TokenKind::Identifier(ref p), .. }) => {
    //             Ok(ASTNode::Identifier(p.to_owned()))
    //         },
    //         _ => return Err(
    //             ParserError::UnexpectedToken("Expected an identifier".into())
    //         )
    //     }
    // }
}
