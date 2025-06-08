use super::*;

pub enum TokenKind {
    ProblemDef,
    LetKwd,
    Object,
    SuperObject,
    ObjectShape,
    ObjectColor,
    ProblemExample,
    ProblemInput,
    ProblemOutput,
    Equals,
    LeftPar,
    RightPar,
    LeftBracket,
    RightBracket,
    LeftBrace,
    RightBrace,
    Comma,
    Colon,
    SemiColon,
    Range,
    CommentStart,
    CommentEnd,
    NewLine,
    Tab,
    Number,
    Float,
    Ident,
}

impl std::str::FromStr for TokenKind {
    type Err = error::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {

    }
}
