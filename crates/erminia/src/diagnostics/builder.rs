use crate::config::CompilerPass;
use crate::diagnostics::{create_diagnostic, Code, Diagnostic, Help, Note, Span};
use crate::lexer::lex::Lexer;

fn _build_diagnostic(
    pass: CompilerPass,
    code: Code,
    tokens: &mut Lexer,
    span: Span,
    note: String,
    help: String,
) -> Diagnostic {
    let mut diagnostic = create_diagnostic(pass, tokens, code, span);

    if !note.is_empty() {
        diagnostic.add_note(note);
    }

    if !help.is_empty() {
        diagnostic.add_help(help);
    }

    diagnostic
}

pub struct DiagnosticBuilder {
    pass: CompilerPass,
    code: Code,
    note: Option<Note>,
    help: Option<Help>,
}

impl DiagnosticBuilder {
    fn new(pass: CompilerPass, code: Code) -> Self {
        Self {
            pass,
            code,
            note: None,
            help: None,
        }
    }

    pub fn build(pass: CompilerPass, code: Code) -> Self {
        Self::new(pass, code)
    }

    pub fn with_note(mut self, note: Option<Note>) -> Self {
        self.note = note;
        self
    }

    pub fn with_help(mut self, help: Option<Help>) -> Self {
        self.help = help;
        self
    }

    pub fn emit(self, tokens: &mut Lexer, span: Span) -> Option<Diagnostic> {
        if tokens.is_poisoned() {
            return None;
        }

        let mut note_str: String = String::default();
        let mut help_str: String = String::default();

        if let Some(ref nt) = self.note {
            note_str = nt.stringify();
        }

        if let Some(ref hp) = self.help {
            help_str = hp.stringify();
        }

        Some(_build_diagnostic(
            self.pass, self.code, tokens, span, note_str, help_str,
        ))
    }
}
