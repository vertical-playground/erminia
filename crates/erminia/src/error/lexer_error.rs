use crate::diagnostics::diagnostics::Location;
use crate::lexer::token::Position;
use derive_more::From;

#[derive(Debug, From)]
pub enum LexerError {
    NoTokenFoundError(Location),
    UnfinishedStringError(Location),
    OpenFileFailureToken(Location),
    TokenError(Location),
    #[from]
    SerdeJson(serde_json::Error),
}

pub type LexerResult<T> = core::result::Result<T, LexerError>;

impl std::fmt::Display for LexerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self:?}")
    }
}

impl std::error::Error for LexerError {}

impl From<std::io::Error> for LexerError {
    fn from(value: std::io::Error) -> Self {
        match value {
            _ => LexerError::OpenFileFailureToken(Location::new(Position::default())),
        }
    }
}
