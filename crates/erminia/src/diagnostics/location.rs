use crate::lexer::lex::PositionalOffset;
use crate::lexer::token::Position;
use derive_more::Display;

// ==================================================================================== //
// Location Struct                                                                      //
// ==================================================================================== //

#[derive(Debug)]
pub struct Location {
    _position: Position,
}

impl Location {
    pub fn new(position: Position) -> Self {
        Location {
            _position: position,
        }
    }
}

#[derive(Display)]
pub enum Code {
    E000X,
    W000X,
    N000X,
    H000X,
}

pub enum DiagnosticLevel {
    Error,
    Warning,
    Note,
    Help,
}

#[derive(Debug)]
pub struct Span {
    pub start: PositionalOffset,
    pub end: PositionalOffset,
}

impl Span {
    pub fn new(start: PositionalOffset, end: PositionalOffset) -> Self {
        Span { start, end }
    }
}

pub struct DiagnosticWindow {
    pub span: Span,
    pub snippet: String,
}

pub struct Diagnostic {
    pub level: DiagnosticLevel,
    pub code: Code,
    pub message: String,
    pub window: DiagnosticWindow,
}

pub struct Accumulator {
    pub diagnostics: Vec<Diagnostic>,
}

// ==================================================================================== //
// Implementations                                                                      //
// ==================================================================================== //

impl DiagnosticLevel {
    pub fn from_code(code: &Code) -> Self {
        if code.to_string().starts_with('E') {
            DiagnosticLevel::Error
        } else if code.to_string().starts_with('W') {
            DiagnosticLevel::Warning
        } else if code.to_string().starts_with('N') {
            DiagnosticLevel::Note
        } else {
            DiagnosticLevel::Help
        }
    }
}

impl Diagnostic {
    pub fn new(
        level: DiagnosticLevel,
        code: Code,
        message: String,
        window: DiagnosticWindow,
    ) -> Self {
        Diagnostic {
            level,
            code,
            message,
            window,
        }
    }
}

impl Accumulator {
    pub fn new() -> Self {
        Accumulator {
            diagnostics: Vec::new(),
        }
    }

    pub fn add_diagnostic(&mut self, diagnostic: Diagnostic) {
        self.diagnostics.push(diagnostic);
    }

    pub fn sort_by_level(&mut self) {
        self.diagnostics.sort_by_key(|d| match d.level {
            DiagnosticLevel::Error => 0,
            DiagnosticLevel::Warning => 1,
            DiagnosticLevel::Note => 2,
            DiagnosticLevel::Help => 3,
        });
    }
}

impl Default for Accumulator {
    fn default() -> Self {
        Self::new()
    }
}

pub trait FromCode {
    fn from_code(code: &Code) -> Self;
}

impl FromCode for DiagnosticLevel {
    fn from_code(code: &Code) -> Self {
        DiagnosticLevel::from_code(code)
    }
}

impl FromCode for String {
    fn from_code(code: &Code) -> Self {
        match code {
            Code::E000X => "An error occurred.".to_string(),
            Code::W000X => "This is a warning.".to_string(),
            Code::N000X => "This is a note.".to_string(),
            Code::H000X => "This is a help message.".to_string(),
        }
    }
}

// ==================================================================================== //
// Macros                                                                               //
// ==================================================================================== //

#[macro_export]
macro_rules! diag {
    (code: $code:expr, window: $window:expr) => {{
        let level = DiagnosticLevel::from_code(&$code);
        let message = String::from_code(&$code);
        Diagnostic::new(level, $code, message, $window)
    }};
}

pub use crate::diag;
