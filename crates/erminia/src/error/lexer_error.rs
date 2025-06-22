use derive_more::From;

#[derive(Debug, From)]
pub enum LexerError {
    NoTokenFoundError,
    UnfinishedStringError,
    OpenFileFailureToken,
    TokenError,
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
            _ => LexerError::OpenFileFailureToken
        }
    }
}
