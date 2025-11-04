use crate::diagnostics::diagnostics::Location;
use crate::error::lexer_error::LexerError;
use crate::lexer::token::{Position, TokenKind};
// use derive_more::From;

#[derive(Debug)]
pub struct ParserErrorInfo {
    _loc: Location,
    _expected: TokenKind,
    _actual: TokenKind,
}

impl Default for ParserErrorInfo {
    fn default() -> Self {
        ParserErrorInfo {
            _loc: Location::new(Position::default()),
            _expected: TokenKind::START,
            _actual: TokenKind::EOF,
        }
    }
}

impl ParserErrorInfo {
    pub fn new(loc: Location, expected: TokenKind, actual: TokenKind) -> Self {
        ParserErrorInfo {
            _loc: loc,
            _expected: expected,
            _actual: actual,
        }
    }
}

#[derive(Debug /*, From*/)]
pub enum ParserError {
    ExpectedLeftInclusivity(ParserErrorInfo),
    ExpectedRightInclusivity(ParserErrorInfo),
    ParserError(ParserErrorInfo),
    ExpectedKeyWordError(ParserErrorInfo),
    ExpectedIdentifierError(ParserErrorInfo),
    ExpectedIntegerConstError(ParserErrorInfo),
    IoError(std::io::Error, ParserErrorInfo),
}

pub type ParserResult<T> = core::result::Result<T, ParserError>;

impl Default for ParserError {
    fn default() -> Self {
        ParserError::ParserError(ParserErrorInfo::default())
    }
}

impl std::fmt::Display for ParserError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self:?}")
    }
}

impl std::error::Error for ParserError {}

impl From<LexerError> for ParserError {
    fn from(value: LexerError) -> Self {
        match value {
            _ => ParserError::default(),
        }
    }
}

impl From<std::io::Error> for ParserError {
    fn from(value: std::io::Error) -> Self {
        match value {
            _ => ParserError::default(),
        }
    }
}
