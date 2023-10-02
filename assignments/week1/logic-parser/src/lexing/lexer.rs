use std::ops::Not;
use crate::errors::LexerError;
use super::token::{TokenKind, Token};

pub type Result<T> = std::result::Result<T, LexerError>;

pub struct Lexer<'a> {
    is_in_alphabet: fn(char) -> bool,
    is_in_start_chars_alphabet: fn(char) -> bool,
    src: &'a str,
    pos: usize
}

pub const DEFAULT_ALPHABET: fn(char) -> bool = |c| { char::is_alphanumeric(c) || c == '_' };
pub const DEFAULT_START_ALPHABET: fn(char) -> bool = |c| { char::is_alphabetic(c) || c == '_' };

impl<'a> Lexer<'a> {
    pub fn new() -> Self {
        Lexer {
            is_in_alphabet: DEFAULT_ALPHABET,
            is_in_start_chars_alphabet: DEFAULT_START_ALPHABET,
            src: "",
            pos: 0
        }
    }

    pub fn with_alphabet(alphabet: fn(char) -> bool, start_chars_alphabet: fn(char) -> bool) -> Self {
        Lexer {
            is_in_alphabet: alphabet,
            is_in_start_chars_alphabet: start_chars_alphabet,
            src: "",
            pos: 0
        }
    }

    pub fn tokenize(&mut self, src: &'a str) -> Result<Vec<Token>> {
        let mut tokens = Vec::new();
        self.src = src;
        self.pos = 0;

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
            c if (self.is_in_start_chars_alphabet)(c) => {
                self.next_word()
            },
            _ => {
                return Err(
                    LexerError::UnknownToken(c, (start, start+1).into())
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
        let token_len = self.take_while(self.is_in_alphabet);
        let p = &self.src[start..start + token_len];
        if p == "true" || p == "false" {
            TokenKind::Literal(p == "true")
        }
        else {
            TokenKind::Identifier(p.into())
        }
    }

    fn skip_whitespaces(&mut self) -> usize {
        self.take_while(|c| {
            let result = c == '\t' || c == ' ' || c == '\r';
            result
        })
    }

    fn take_while<F>(&mut self, pred: F) -> usize
    where F: Fn(char) -> bool {
        let from = self.pos;

        for c in self.src[self.pos..].chars() {
            if pred(c).not() {
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
        let mut lexer = Lexer::new();
        lexer.src = "testing => this";
        let n = Lexer::skip_whitespaces(&mut lexer);
        assert_eq!(n, 0);
    }

    #[test]
    fn literal_booleans_work() {
        let mut lexer = Lexer::new();
        let tokens = lexer.tokenize("false & true").unwrap();
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
        let mut lexer = Lexer::new();
        lexer.src = "kittens";
        let n = lexer.take_while(|_| false);
        assert_eq!(n, 0);
    }

    #[test]
    fn take_while_returns_correct_amount() {
        let mut lexer = Lexer::new();
        lexer.src = "kittens";
        let n = lexer.take_while(|_| true);
        assert_eq!(n, 7);
    }

    #[test]
    fn skip_whitespaces_works_properly() {
        let mut lexer = Lexer::new();
        let tokens = lexer.tokenize("\t\r puppies").unwrap();
        assert_eq!(tokens.len(), 1);
        assert_eq!(tokens[0].kind, TokenKind::Identifier("puppies".into()));
    }

    #[test]
    fn error_is_returned_when_alphabet_doesnt_match() {
        let mut lexer = Lexer::with_alphabet(
            |c| ['a', 'b', 'c', '1', '2', '3'].contains(&c),
            |c| ['a', 'b', 'c'].contains(&c)
        );
        match lexer.tokenize("abcf").unwrap_err() {
            LexerError::UnknownToken(token, span) => {
                assert_eq!(token, 'f');
                assert_eq!(span, (3, 4).into());
            },
            _ => unreachable!()
        };

        match lexer.tokenize("123abc").unwrap_err() {
            LexerError::UnknownToken(token, span) => {
                assert_eq!(token, '1');
                assert_eq!(span, (0, 1).into());
            },
            _ => unreachable!()
        };
    }

    #[test]
    #[should_panic]
    fn propositions_cant_start_with_numbers() {
        let mut lexer = Lexer::new();
        if !lexer.tokenize("pqrs").is_ok() { return }

        let mut lexer = Lexer::new();
        let _ = lexer.tokenize("69p").unwrap();
    }
}
