use crate::error::lexer_error::LexerError;
use derive_more::From;

#[derive(Debug, From)]
pub enum ParserError {
    ParserError,
    ExpectedKeyWordError,
    ExpectedIdentifierError,
    ExpectedIntegerConstError,
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
            _ => ParserError::ParserError,
        }
    }
}

impl From<std::io::Error> for ParserError {
    fn from(value: std::io::Error) -> Self {
        match value {
            _ => ParserError::ParserError,
        }
    }
}
