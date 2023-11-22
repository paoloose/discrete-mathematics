/// Macro to avoid repetition in the Lexer::next_token() method.
///
/// IMPORTANT: The array of expectations need to be ordered from the largest to
/// the shortest string.
#[macro_export]
macro_rules! match_any_or_syntax_error {
    ($lexer: expr, $expectations: expr, $kind: expr) => {{
        let expect = $expectations;

        if let Some(iff) = expect.iter().find(|m| $lexer.next_matches(m)) {
            $lexer.skip(iff.len());
            $kind
        }
        else {
            return Err(
                LexerError::SyntaxError(
                    format!("expected one of the following: {}", expect.iter().map(|s| format!("'{}'", s)).collect::<Vec<String>>().join(", ")),
                    ($lexer.pos, $lexer.pos + 1).into()
                )
            )
        }
    }
    };
}
