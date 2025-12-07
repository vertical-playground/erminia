pub mod code;
pub mod location;
pub mod messages;

use crate::config::CompilerPass;
use crate::lexer::lex::Lexer;

fn _build_diagnostic(
    pass: CompilerPass,
    code: code::Code,
    tokens: &mut Lexer,
    span: location::Span,
    note: String,
    help: String,
) -> location::Diagnostic {
    let mut diagnostic = location::create_diagnostic(pass, tokens, code, span);

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
    code: code::Code,
    note: messages::Note,
    note_str: String,
    help: messages::Help,
    help_str: String,
    with_args: bool,
}

impl DiagnosticBuilder {
    fn new(pass: CompilerPass, code: code::Code) -> Self {
        Self {
            pass,
            code,
            note: messages::Note::default(),
            help: messages::Help::default(),
            note_str: String::new(),
            help_str: String::new(),
            with_args: false,
        }
    }

    pub fn build(pass: CompilerPass, code: code::Code) -> Self {
        Self::new(pass, code)
    }

    pub fn with_note(mut self, note: messages::Note) -> Self {
        self.note = note;
        self
    }

    pub fn with_args(mut self, norh: messages::MessageKind, args: Vec<String>) -> Self {
        if args.len() != self.note.args_count() {
            let diagnostic = _build_diagnostic(
                CompilerPass::Internal,
                code::Code::I0001,
                &mut Lexer::default(),
                location::Span::default(),
                "Note message requires arguments, but not the right amount was provided.".to_string(),
                "If you are seeing this, please raise an issue on Github at 'https://github.com/vertical-playground/erminia/issues'".to_string(),
            );

            panic!("{}", diagnostic);
        }
        match norh {
            messages::MessageKind::Note => {
                if self.note == messages::Note::default() {
                    return self;
                }

                self.note_str = self.note.stringify(args);
                self.with_args = true;
            }
            messages::MessageKind::Help => {
                if self.help == messages::Help::default() {
                    return self;
                }

                self.help_str = self.help.stringify();
            }
        };

        self
    }

    pub fn with_help(mut self, help: messages::Help) -> Self {
        self.help = help;
        self
    }

    pub fn emmit(self, tokens: &mut Lexer, span: location::Span) -> location::Diagnostic {
        if self.note.args_required() && !self.with_args {
            let diagnostic = _build_diagnostic(
                CompilerPass::Internal,
                code::Code::I0001,
                tokens,
                location::Span::default(),
                "Note message requires arguments, but none were provided.".to_string(),
                "If you are seeing this, please raise an issue on Github at 'https://github.com/vertical-playground/erminia/issues'".to_string(),
            );

            panic!("{}", diagnostic);
        }

        _build_diagnostic(
            self.pass,
            self.code,
            tokens,
            span,
            self.note_str,
            self.help_str,
        )
    }
}
