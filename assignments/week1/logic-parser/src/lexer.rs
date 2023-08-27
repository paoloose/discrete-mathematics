use std::ops::Not;
use crate::errors::LexerError;

pub type Result<T> = std::result::Result<T, LexerError>;

#[derive(Debug)]
pub enum TokenKind {
    Proposition(String),
    Literal(bool),
    Not,
    And,
    Or,
    Implies,
    OpenParen,
    CloseParen
}

#[derive(Debug)]
pub struct Token {
    kind: TokenKind,
    start: usize,
    len: usize
}

pub struct LexerBuilder<'a> {
    expr: &'a str,
    pos: usize
}

impl<'a> LexerBuilder<'a> {
    pub fn new(expr: &'a str) -> LexerBuilder {
        LexerBuilder { expr: expr.clone(), pos: 0 }
    }

    fn next_token(&mut self) -> Result<Option<Token>> {
        self.skip_whitespaces();
        let start = self.pos;

        let c = match self.next_char() {
            Some(c) => c,
            None => return Ok(None)
        };

        let token = match c {
            '(' => Token { kind: TokenKind::OpenParen, start, len: 1 },
            ')' => Token { kind: TokenKind::CloseParen, start, len: 1 },
            '!' => Token { kind: TokenKind::Not, start, len: 1 },
            '&' => Token { kind: TokenKind::And, start, len: 1 },
            '|' => Token { kind: TokenKind::Or, start, len: 1 },
            '=' => {
                match self.next_char() {
                    Some('>') => Token { kind: TokenKind::Implies, start, len: 1 },
                    _ => return Err(LexerError::SyntaxError(format!("Unexpected character (expected '=>'), got '{}'", c)))
                }
            },
            c @ '_' | c if c.is_alphabetic() => {
                // We add one because we already consumed the first character
                let token_len = self.take_while(|c| c.is_alphanumeric()) + 1;
                let p = &self.expr[start..start + token_len];
                if p == "false" || p == "true" {
                    Token { kind: TokenKind::Literal(if p == "true" { true } else { false }), start, len: token_len }
                }
                else {
                    Token { kind: TokenKind::Proposition(p.into()), start, len: token_len }
                }
            },
            _ => return Err(LexerError::UnkownToken(format!("Unkown Token: '{}'", c)))
        };

        Ok(Some(token))
    }

    fn next_char(&mut self) -> Option<char> {
        let c = self.expr[self.pos..].chars().next();
        match c {
            Some(c) => {
                self.pos += c.len_utf8();
                Some(c)
            },
            None => None,
        }
    }

    fn skip_whitespaces(&mut self) {
        self.take_while(|c| c == '\t' || c == ' ');
    }

    fn take_while<F>(&mut self, pred: F) -> usize
    where F: Fn(char) -> bool {
        let from = self.pos;

        for c in self.expr[self.pos..].chars() {
            if pred(c).not() {
                break;
            }
            self.pos += c.len_utf8();
        }

        self.pos - from
    }

    pub fn parse(mut self) -> Result<Vec<Token>> {
        let mut tokens = Vec::new();

        while let Some(token) = self.next_token()? {
            tokens.push(token);
        }

        return Ok(tokens);
    }
}
