use crate::diagnostics::diagnostics::Location;
use crate::error::lexer_error::LexerError;
use crate::lexer::token::{Position, TokenKind};
// use derive_more::From;

#[derive(Debug /*, From*/)]
pub enum ParserError {
    ExpectedLeftInclusivity(Location, TokenKind),
    ExpectedRightInclusivity(Location, TokenKind),
    ParserError(Location, TokenKind),
    ExpectedKeyWordError(Location, TokenKind),
    ExpectedIdentifierError(Location, TokenKind),
    ExpectedIntegerConstError(Location, TokenKind),

    IoError(std::io::Error, Location, TokenKind),
}

pub type ParserResult<T> = core::result::Result<T, ParserError>;

impl std::fmt::Display for ParserError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self:?}")
    }
}

impl std::error::Error for ParserError {}

impl From<LexerError> for ParserError {
    fn from(value: LexerError) -> Self {
        match value {
            _ => ParserError::ParserError(Location::new(Position::default()), TokenKind::EOF),
        }
    }
}

impl From<std::io::Error> for ParserError {
    fn from(value: std::io::Error) -> Self {
        match value {
            _ => ParserError::ParserError(Location::new(Position::default()), TokenKind::EOF),
        }
    }
}
