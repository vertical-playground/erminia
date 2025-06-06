use logos::{Logos, Skip};
use num_derive::{FromPrimitive, ToPrimitive};

fn newline_callback(lex: &mut logos::Lexer<SyntaxKind>) -> Skip {
    lex.extras.line += 1;
    lex.extras.span_end = lex.span().end;
    Skip
}

#[derive(Default, Clone)]
pub struct TokenLocation {
    line: usize,
    span_end: usize,
}

#[derive(Default, Debug, Clone, PartialEq)]
pub enum LexingError {
    #[default]
    NonAcceptableCharacter,
}

#[derive(
    Debug, Copy, Clone, FromPrimitive, Eq, Ord, Hash, PartialEq, PartialOrd, Logos, ToPrimitive,
)]
#[logos(skip r" +")]
#[logos(subpattern digit = r"[0-9]")]
#[logos(extras = TokenLocation)]
#[logos(error = LexingError)]
pub(crate) enum SyntaxKind {
    #[token("def")]
    ProblemDef,
    #[token("let")]
    LetKwd,
    #[token("object")]
    Object,
    #[token("superobject")]
    SuperObject,
    #[token("shape")]
    ObjectShape,
    #[token("color")]
    ObjectColor,
    #[token("example")]
    ProblemExample,
    #[token("input")]
    ProblemInput,
    #[token("output")]
    ProblemOutput,
    #[token("=")]
    Equals,
    #[token("(")]
    LeftPar,
    #[token(")")]
    RightPar,
    #[token("[")]
    LeftBracket,
    #[token("]")]
    RightBracket,
    #[token("{")]
    LeftBrace,
    #[token("}")]
    RightBrace,
    #[token(",")]
    Comma,
    #[token(":")]
    Colon,
    #[token(";")]
    SemiColon,
    #[token("..")]
    Range,
    #[token("(*")]
    CommentStart,
    #[token("*)")]
    CommentEnd,
    #[token("\n", newline_callback)]
    NewLine,
    #[token("\t")]
    Tab,
    #[regex("(?&digit)+")]
    Number,
    #[regex(r"[+-]?(?&digit)*\.(?&digit)+")]
    Float,
    #[regex("[a-zA-Z0-9]*")]
    Ident,
}

#[derive(Clone)]
pub(crate) struct Lexer<'a> {
    inner: logos::Lexer<'a, SyntaxKind>,
}

impl<'a> Lexer<'a> {
    pub(crate) fn new(input: &'a str) -> Self {
        Self {
            inner: SyntaxKind::lexer(input),
        }
    }
}

impl<'a> Iterator for Lexer<'a> {
    type Item = (SyntaxKind, &'a str);

    fn next(&mut self) -> Option<Self::Item> {
        let kind = self.inner.next()?;
        let input = self.inner.slice();

        Some((kind.expect(""), input))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn check(input: &str, kind: Result<SyntaxKind, ()>) {
        let mut lexer = Lexer::new(input);
        assert_eq!(lexer.next(), Some((kind.expect(""), input)));
    }

    #[test]
    #[should_panic]
    fn lex_invalid() {
        let input = "ƒ";
        let mut lexer = Lexer::new(input);

        lexer.next();
    }

    #[test]
    fn lex_multiples() {
        let input = "def ident12 { let }";
        let mut lexer = Lexer::new(input);

        let expected = vec![
            (SyntaxKind::ProblemDef, "def"),
            (SyntaxKind::Ident, "ident12"),
            (SyntaxKind::LeftBrace, "{"),
            (SyntaxKind::LetKwd, "let"),
            (SyntaxKind::RightBrace, "}"),
        ];

        for (kind, slice) in expected {
            assert_eq!(lexer.next(), Some((kind, slice)));
        }

        assert_eq!(lexer.next(), None);
    }

    #[test]
    fn lex_nl() {
        check("\n", Ok(SyntaxKind::NewLine))
    }

    #[test]
    fn lex_tab() {
        check("\t", Ok(SyntaxKind::Tab))
    }

    #[test]
    fn lex_problem_def() {
        check("def", Ok(SyntaxKind::ProblemDef));
    }

    #[test]
    fn lex_let_kwd() {
        check("let", Ok(SyntaxKind::LetKwd));
    }

    #[test]
    fn lex_object() {
        check("object", Ok(SyntaxKind::Object));
    }

    #[test]
    fn lex_superobject() {
        check("superobject", Ok(SyntaxKind::SuperObject));
    }

    #[test]
    fn lex_object_shape() {
        check("shape", Ok(SyntaxKind::ObjectShape));
    }

    #[test]
    fn lex_object_color() {
        check("color", Ok(SyntaxKind::ObjectColor));
    }

    #[test]
    fn lex_problem_example() {
        check("example", Ok(SyntaxKind::ProblemExample));
    }

    #[test]
    fn lex_problem_input() {
        check("input", Ok(SyntaxKind::ProblemInput));
    }

    #[test]
    fn lex_problem_output() {
        check("output", Ok(SyntaxKind::ProblemOutput));
    }

    #[test]
    fn lex_equals() {
        check("=", Ok(SyntaxKind::Equals));
    }

    #[test]
    fn lex_leftpar() {
        check("(", Ok(SyntaxKind::LeftPar));
    }

    #[test]
    fn lex_rightpar() {
        check(")", Ok(SyntaxKind::RightPar));
    }

    #[test]
    fn lex_leftbracket() {
        check("[", Ok(SyntaxKind::LeftBracket));
    }

    #[test]
    fn lex_rightbracket() {
        check("]", Ok(SyntaxKind::RightBracket));
    }

    #[test]
    fn lex_leftbrace() {
        check("{", Ok(SyntaxKind::LeftBrace));
    }

    #[test]
    fn lex_rightbrace() {
        check("}", Ok(SyntaxKind::RightBrace));
    }

    #[test]
    fn lex_comma() {
        check(",", Ok(SyntaxKind::Comma));
    }

    #[test]
    fn lex_colon() {
        check(":", Ok(SyntaxKind::Colon));
    }

    #[test]
    fn lex_semicolon() {
        check(";", Ok(SyntaxKind::SemiColon));
    }

    #[test]
    fn lex_range() {
        check("..", Ok(SyntaxKind::Range));
    }

    #[test]
    fn lex_identifier() {
        check("hello", Ok(SyntaxKind::Ident))
    }

    #[test]
    fn lex_number() {
        check("123", Ok(SyntaxKind::Number))
    }

    #[test]
    fn lex_float() {
        check("123.2232", Ok(SyntaxKind::Float))
    }

    #[test]
    fn lex_float_no_int() {
        check(".2232", Ok(SyntaxKind::Float))
    }
}
