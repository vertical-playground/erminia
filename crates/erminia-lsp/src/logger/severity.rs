pub(crate) enum Severity {
    Log,
    Warn,
    Error,
}

#[allow(clippy::to_string_trait_impl)]
impl ToString for Severity {
    fn to_string(&self) -> String {
        match self {
            Severity::Log => "[LOG]".to_string(),
            Severity::Warn => "[WARN]".to_string(),
            Severity::Error => "[ERROR]".to_string(),
        }
    }
}
