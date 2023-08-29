/// Enum representing all the tokens the Lexer can produce
#[derive(Debug, PartialEq, Clone)]
pub enum TokenKind {
    Identifier(String),
    Literal(bool),
    Not,
    And,
    Or,
    Implies,
    IfAndOnlyIf,
    OpenParen,
    CloseParen
}

/// An [`Span`] represents a range of characters in the source code
#[derive(Debug, Clone)]
pub struct Span {
    pub start: usize,
    pub end: usize
}

/// Represents the tokens that the [`Lexer`] will produce and the [`Parser`]
/// will consume
#[derive(Debug, Clone)]
pub struct Token {
    pub kind: TokenKind,
    pub span: Span
}

impl Token {
    pub fn new<T>(kind: TokenKind, span: T) -> Token
    where T: Into<Span> {
        Token { kind, span: span.into() }
    }
}

// -> Other traits implementations

impl std::fmt::Display for TokenKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TokenKind::Identifier(i) => write!(f, "Identifier({})", i),
            TokenKind::Literal(l) => write!(f, "Literal({})", l),
            TokenKind::Not => write!(f, "Not()"),
            TokenKind::And => write!(f, "And()"),
            TokenKind::Or => write!(f, "Or()"),
            TokenKind::Implies => write!(f, "Implies()"),
            TokenKind::IfAndOnlyIf => write!(f, "IfAndOnlyIf()"),
            TokenKind::OpenParen => write!(f, "OpenParen()"),
            TokenKind::CloseParen => write!(f, "CloseParen()"),
        }
    }
}

impl From<(usize, usize)> for Span {
    fn from(value: (usize, usize)) -> Self {
        Span { start: value.0, end: value.1 }
    }
}
