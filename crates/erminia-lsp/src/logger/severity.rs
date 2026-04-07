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
            Severity::Warn => "[WAR]".to_string(),
            Severity::Error => "[ERR]".to_string(),
        }
    }
}
