use crate::error::lexer_error::LexerError;
use derive_more::From;

#[derive(Debug, From)]
pub enum ASTError {
    ASTError,
    ExpectedKeyWordError,
    ExpectedIdentifierError,
    ExpectedIntegerConstError,
}

pub type ASTResult<T> = core::result::Result<T, ASTError>;

impl std::fmt::Display for ASTError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self:?}")
    }
}

impl std::error::Error for ASTError {}

impl From<LexerError> for ASTError {
    fn from(value: LexerError) -> Self {
        match value {
            _ => ASTError::ASTError,
        }
    }
}

impl From<std::io::Error> for ASTError {
    fn from(value: std::io::Error) -> Self {
        match value {
            _ => ASTError::ASTError,
        }
    }
}
