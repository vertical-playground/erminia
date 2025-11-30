use crate::config::CompilerPass;
use crate::diagnostics::code::{Code, DiagnosticLevel, FromCode};
use crate::lexer::lex::Lexer;
use crate::lexer::lex::PositionalOffset;

use colored::*;
use std::fmt;

// ==================================================================================== //
// Structs                                                                              //
// ==================================================================================== //

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Span {
    pub start: PositionalOffset,
    pub end: PositionalOffset,
}

impl Span {
    pub fn new(start: PositionalOffset, end: PositionalOffset) -> Self {
        Span { start, end }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct DiagnosticWindow {
    pub span: Span,
    pub snippet: String,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Diagnostic {
    pub level: DiagnosticLevel,
    pub code: Code,
    pub pass: CompilerPass,
    pub message: String,
    pub window: DiagnosticWindow,
    pub note: String,
    pub help: String,
}

#[derive(Debug, Clone)]
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
            note: String::new(),
            help: String::new(),
        }
    }

    pub fn add_note(&mut self, note: String) {
        self.note = note;
    }

    pub fn add_help(&mut self, help: String) {
        self.help = help;
    }
}

impl fmt::Display for Diagnostic {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(
            f,
            " {}[{}] {}",
            self.level,
            self.code.to_string().red().bold(),
            self.message.bold()
        )?;
        writeln!(f, "  {} {:?}", "pass:".dimmed(), self.pass)?;

        if self.pass != CompilerPass::Internal {
            writeln!(
                f,
                "  {} {}..{}",
                "span:".dimmed(),
                self.window.span.start,
                self.window.span.end
            )?;
        }

        writeln!(f, "  │")?;
        for line in self.window.snippet.lines() {
            writeln!(f, "  │   {}", line)?;
        }
        writeln!(f, "  │")?;

        if !self.note.is_empty() {
            writeln!(f, "  = {} {}", "note:".bold(), self.note)?;
        }

        if !self.help.is_empty() {
            writeln!(f, "  * {} {}", "help:".bold(), self.help)?;
        }

        writeln!(f)?;
        Ok(())
    }
}

impl Accumulator {
    pub fn new() -> Self {
        Accumulator {
            diagnostics: Vec::new(),
        }
    }

    pub fn add_diag(&mut self, diagnostic: Diagnostic) {
        self.diagnostics.push(diagnostic);
    }

    pub fn sort(&mut self) {
        self.diagnostics.sort_by_key(|d| match d.level {
            DiagnosticLevel::Internal => -1,
            DiagnosticLevel::Error => 0,
            DiagnosticLevel::Warning => 1,
            DiagnosticLevel::Note => 2,
            DiagnosticLevel::Help => 3,
        });
    }

    pub fn is_blocking(&mut self, next: CompilerPass) -> bool {
        self.diagnostics.sort();
        for diag in &self.diagnostics {
            if diag.pass < next && diag.level == DiagnosticLevel::Error {
                return true;
            }
        }
        false
    }

    pub fn get(&self, pass: CompilerPass) -> Vec<Diagnostic> {
        if pass == CompilerPass::ALL {
            return self.diagnostics.clone();
        }

        self.diagnostics
            .iter()
            .filter(|d| d.pass == pass)
            .cloned()
            .collect()
    }
}

impl fmt::Display for Accumulator {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for d in &self.diagnostics {
            writeln!(f, "{}", d)?;
        }

        Ok(())
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

pub fn create_diagnostic(
    pass: CompilerPass,
    tokens: &mut Lexer,
    code: Code,
    span: Span,
) -> Diagnostic {
    let level = DiagnosticLevel::from_code(&code);
    let message = String::from_code(&code);
    let snippet = tokens.get_snippet(span);
    let window = DiagnosticWindow {
        span,
        snippet: snippet.to_string(),
    };

    Diagnostic::new(level, code, pass, message, window)
}
