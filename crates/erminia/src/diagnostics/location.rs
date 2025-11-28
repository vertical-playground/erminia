use crate::diagnostics::code::{Code, DiagnosticLevel, FromCode};
use crate::lexer::lex::PositionalOffset;
use crate::lexer::lex::Lexer;
use crate::config::CompilerPass;

// ==================================================================================== //
// Structs                                                                              //
// ==================================================================================== //

#[derive(Debug, Clone, Copy)]
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
    pub pass: CompilerPass,
    pub message: String,
    pub window: DiagnosticWindow,
}

pub struct Accumulator {
    pub diagnostics: Vec<Diagnostic>,
}

// ==================================================================================== //
// Implementations                                                                      //
// ==================================================================================== //

impl Diagnostic {
    pub fn new(
        level: DiagnosticLevel,
        code: Code,
        pass: CompilerPass,
        message: String,
        window: DiagnosticWindow,
    ) -> Self {
        Diagnostic {
            level,
            code,
            pass,
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

pub trait ToSnippet {
    fn to_snippet(&self) -> String;
}

// ==================================================================================== //
// Macros                                                                               //
// ==================================================================================== //

pub fn create_diagnostic(pass: CompilerPass, tokens: &mut Lexer, code: Code) -> Diagnostic {
    let level = DiagnosticLevel::from_code(&code);
    let message = String::from_code(&code);
    let start = tokens.get_position();
    let end = tokens.get_position();
    let span = Span::new(start, end);
    let snippet = tokens.get_snippet(span);
    let window = DiagnosticWindow { span, snippet: snippet.to_string() };

    Diagnostic::new(level, code, pass, message, window)
}
