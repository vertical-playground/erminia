#![allow(unused)]

use logos::Logos;

#[derive(Debug, Copy, Clone, PartialEq, Logos)]
pub(crate) enum SyntaxKind {
    #[regex(" +")]
    Whitespace,

    #[token("fn")]
    FnKwd
}

#[cfg(test)]
mod tests {
    use super::*;

    fn check(
        input: &str,
        kind: Result<SyntaxKind, ()>
        ) {
        let mut lexer = SyntaxKind::lexer(input);

        assert_eq!(lexer.next(), Some(kind));
        assert_eq!(lexer.slice(), input);

    }

    #[test]
    fn lex_spaces() {
        check("  ", Ok(SyntaxKind::Whitespace));
    }

    #[test]
    fn lex_fn_keyword() {
        check("fn", Ok(SyntaxKind::FnKwd));
    }

}

