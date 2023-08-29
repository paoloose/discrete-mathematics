use std::ops::Not;
use crate::errors::LexerError;
use super::token::{TokenKind, Token};

pub type Result<T> = std::result::Result<T, LexerError>;

pub struct Lexer<'a> {
    src: &'a str,
    pos: usize
}

impl<'a> Lexer<'a> {
    pub fn new(expr: &'a str) -> Lexer {
        Lexer { src: expr.clone(), pos: 0 }
    }

    pub fn parse(mut self) -> Result<Vec<Token>> {
        let mut tokens = Vec::new();

        while let Some(token) = self.next_token()? {
            tokens.push(token);
        }
        Ok(tokens)
    }

    fn next_token(&mut self) -> Result<Option<Token>> {
        self.skip_whitespaces();
        let start = self.pos;

        let c = match self.peek_char() {
            Some(c) => c,
            None => return Ok(None)
        };

        let kind = match c {
            '~' | '!' => { self.consume(); TokenKind::Not },
            '(' => { self.consume(); TokenKind::OpenParen },
            ')' => { self.consume(); TokenKind::CloseParen },
            '&' => {
                match_any_or_syntax_error!(self, ["&&", "&"], TokenKind::And)
            },
            '|' => {
                match_any_or_syntax_error!(self, ["||", "|"], TokenKind::Or)
            },
            '=' | '-' => {
                match_any_or_syntax_error!(self, ["=>", "->"], TokenKind::Implies)
            },
            '<' => {
                match_any_or_syntax_error!(self, ["<->", "<=>"], TokenKind::IfAndOnlyIf)
            },
            c if c.is_alphabetic() || c == '_' => {
                self.next_word()
            },
            _ => {
                return Err(
                    LexerError::UnknownToken(c.into(), (start, start+1).into())
                )
            }
        };

        Ok(Some(Token::new(kind, (start, self.pos))))
    }

    fn consume(&mut self) -> Option<char> {
        let next = self.src[self.pos..].chars().next();
        if let Some(c) = next {
            self.pos += c.len_utf8();
        }
        next
    }

    fn skip(&mut self, n: usize) {
        for _ in 0..n {
            self.consume();
        }
    }

    fn next_matches(&self, to_match: &str) -> bool {
        let mut chars = self.src[self.pos..].chars();
        for c in to_match.chars() {
            if chars.next() != Some(c) {
                return false;
            }
        }
        true
    }

    fn peek_char(&self) -> Option<char> {
        self.src[self.pos..].chars().next()
    }

    /// A word can be a literal ([`TokenKind::Literal`]) or an identifier
    /// ([`TokenKind::Identifier`])
    fn next_word(&mut self) -> TokenKind {
        let start = self.pos;
        // We add one because we already consumed the first character
        let token_len = self.take_while(|c, _| c.is_alphanumeric() || c == '_');
        let p = &self.src[start..start + token_len];
        if p == "true" || p == "false" {
            TokenKind::Literal(p == "true")
        }
        else {
            TokenKind::Identifier(p.into())
        }
    }

    fn skip_whitespaces(&mut self) -> usize {
        self.take_while(|c, _| c == '\t' || c == ' ' || c == '\r')
    }

    fn take_while<F>(&mut self, pred: F) -> usize
    where F: Fn(char, usize) -> bool {
        let from = self.pos;

        for (i, c) in self.src[self.pos..].chars().enumerate() {
            if pred(c, i).not() {
                break;
            }
            self.pos += c.len_utf8();
        }

        self.pos - from
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn no_whitespaces_should_return_zero() {
        let expr = "testing => this";
        let n = Lexer::skip_whitespaces(&mut Lexer::new(expr));
        assert_eq!(n, 0);
    }

    #[test]
    fn literal_booleans_work() {
        let lexer = Lexer::new("false & true");
        let tokens = lexer.parse().unwrap();
        assert_eq!(
            tokens.iter().map(|t| &t.kind).collect::<Vec<&TokenKind>>(),
            vec![
                &TokenKind::Literal(false),
                &TokenKind::And,
                &TokenKind::Literal(true)
            ]
        );
    }

    #[test]
    fn take_while_returns_zero_if_no_matches() {
        let mut lexer = Lexer::new("testing");
        let n = lexer.take_while(|_, _| false);
        assert_eq!(n, 0);
    }

    #[test]
    fn take_while_returns_correct_amount() {
        let mut lexer = Lexer::new("kittens");
        let n = lexer.take_while(|_, _| true);
        assert_eq!(n, 7);
    }

    #[test]
    fn skip_whitespaces_works_properly() {
        let lexer = Lexer::new("\t\r puppies");
        let tokens = lexer.parse().unwrap();
        assert_eq!(tokens.len(), 1);
        assert_eq!(tokens[0].kind, TokenKind::Identifier("puppies".into()));
    }

    #[test]
    #[should_panic]
    fn propositions_cant_start_with_numbers() {
        let lexer = Lexer::new("pqrs");
        if !lexer.parse().is_ok() { return; }

        let lexer = Lexer::new("69p");
        let _ = lexer.parse().unwrap();
    }
}
