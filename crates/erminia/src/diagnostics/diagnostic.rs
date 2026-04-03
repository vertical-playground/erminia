use crate::config::CompilerPass;
use crate::diagnostics::code::{Code, DiagnosticLevel, FromCode};
use crate::diagnostics::{DiagnosticWindow, Span};
use crate::lexer::lex::Lexer;

use colored::*;
use std::fmt;

#[derive(Default, Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
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
pub struct DiagnosticAccumulator {
    pub diagnostics: Vec<Diagnostic>,
}

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
            " [{}] {} {}",
            self.code.to_string().red().bold(),
            self.level,
            self.message.bold()
        )?;
        writeln!(f, "  {} {}", "pass:".dimmed(), self.pass)?;

        if self.pass != CompilerPass::Internal {
            writeln!(
                f,
                "  {} {}:{}::{}",
                "span:".dimmed(),
                self.window.span.start.get_line(),
                self.window.span.start.get_cursor(),
                self.window.span.end.get_cursor()
            )?;
        }

        writeln!(f, "  │")?;
        for line in self.window.snippet.lines() {
            writeln!(
                f,
                "{} │   {}",
                self.window.span.start.get_line().to_string().green().bold(),
                line.dimmed()
            )?;
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

impl DiagnosticAccumulator {
    pub fn new() -> Self {
        DiagnosticAccumulator {
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

impl fmt::Display for DiagnosticAccumulator {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for d in &self.diagnostics {
            writeln!(f, "{}", d)?;
        }

        Ok(())
    }
}

impl Default for DiagnosticAccumulator {
    fn default() -> Self {
        Self::new()
    }
}

#[allow(unused)]
pub trait ToSnippet {
    fn to_snippet(&self) -> String;
}

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
