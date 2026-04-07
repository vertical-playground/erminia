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
        let secs = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();

        let s = secs % 60;
        let m = (secs / 60) % 60;
        let h = (secs / 3600) % 24;
        let days = secs / 86400;

        let (year, month, day) = {
            let mut y = 1970u32;
            let mut d = days as u32;
            loop {
                let days_in_y =
                    if y.is_multiple_of(4) && (!y.is_multiple_of(100) || y.is_multiple_of(400)) {
                        366
                    } else {
                        365
                    };
                if d < days_in_y {
                    break;
                }
                d -= days_in_y;
                y += 1;
            }

            let leap = y.is_multiple_of(4) || y.is_multiple_of(400);

            let month_days = [
                31u32,
                if leap { 29 } else { 28 },
                31,
                30,
                31,
                30,
                31,
                31,
                30,
                31,
                30,
                31,
            ];
            let mut mo = 1u32;
            for &md in &month_days {
                if d < md {
                    break;
                }
                d -= md;
                mo += 1;
            }
            (y, mo, d + 1)
        };

        format!("[{year}-{month:02}-{day:02} {h:02}:{m:02}:{s:02} UTC]")
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
