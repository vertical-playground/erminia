use crate::ast::ast::BoxAST;
use crate::diagnostics::DiagnosticAccumulator;
use crate::lexer::lex::Lexer;
use crate::syntax::parse::parse_program;
mod consumers;
mod parse;

// ==================================================================================== //
// Parser Object                                                                        //
// ==================================================================================== //

pub struct Parser<'a> {
    lexer: Lexer<'a>,
    diagnostics: DiagnosticAccumulator,
}

impl<'a> Parser<'a> {
    pub fn new<'input>(input: &'input str) -> Parser<'input> {
        let lexer = Lexer::new(input);
        let diagnostics = DiagnosticAccumulator::new();
        Parser { lexer, diagnostics }
    }

    pub fn parse(&mut self) -> BoxAST<'a> {
        parse_program(&mut self.lexer, &mut self.diagnostics)
    }

    pub fn get_diagnostics(&self) -> &DiagnosticAccumulator {
        &self.diagnostics
    }
}

pub mod macros {
    #[macro_export]
    macro_rules! parser_diag {
        ($code:ident, $note:ident, $args:expr, $help:ident, $tokens:expr, $diag:expr, $span:expr) => {{
            if let Some(dgn) = DB::build(PARSER_PASS, Code::$code)
                .with_note(Note::$note)
                .with_args(MessageKind::Note, $args)
                .with_help(Help::$help)
                .emmit($tokens, $span)
            {
                $diag.add_diag(dgn)
            }
        }};

        ($code:ident, $note:ident, $args:expr, $tokens:expr, $diag:expr, $span:expr) => {{
            if let Some(dgn) = DB::build(PARSER_PASS, Code::$code)
                .with_note(Note::$note)
                .with_args(MessageKind::Note, $args)
                .emmit($tokens, $span)
            {
                $diag.add_diag(dgn)
            }
        }};
    }
}
