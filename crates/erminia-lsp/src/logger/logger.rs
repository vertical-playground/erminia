use crate::logger::severity::Severity;

use std::io::Write;

pub(crate) struct Logger<'a> {
    file: &'a std::fs::File,
}

impl<'a> Logger<'a> {
    pub fn new(file: &'a std::fs::File) -> Self {
        Logger { file }
    }

    fn _wrap_message_with_time() -> String {
        format!("[{:?}]", std::time::SystemTime::now())
    }

    fn _wrap_message(message: String, severity: Severity) -> String {
        format!(
            "{} {}: {}",
            severity.to_string(),
            &Logger::_wrap_message_with_time(),
            message
        )
    }

    pub fn log(&mut self, message: &str) -> std::io::Result<()> {
        writeln!(
            self.file,
            "{}",
            Logger::_wrap_message(message.to_string(), Severity::Log)
        )
    }

    pub fn warn(&mut self, message: &str) -> std::io::Result<()> {
        writeln!(
            self.file,
            "{}",
            Logger::_wrap_message(message.to_string(), Severity::Warn)
        )
    }

    pub fn error(&mut self, message: &str) -> std::io::Result<()> {
        writeln!(
            self.file,
            "{}",
            Logger::_wrap_message(message.to_string(), Severity::Error)
        )
    }
}
