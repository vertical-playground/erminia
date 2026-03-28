pub mod lex;
pub mod token;

pub mod macros {
    #[macro_export]
    macro_rules! lexer_diag {
        ($code:ident, $note:ident, $args:expr, $help:ident, $tokens:expr, $diag:expr, $span:expr) => {{
            if let Some(dgn) = DB::build(LEXER_PASS, Code::$code)
                .with_note(Note::$note)
                .with_args(MessageKind::Note, $args)
                .with_help(Help::$help)
                .emmit($tokens, $span)
            {
                $diag.add_diag(dgn)
            }
        }};

        ($code:ident, $note:ident, $args:expr, $tokens:expr, $diag:expr, $span:expr) => {{
            if let Some(dgn) = DB::build(LEXER_PASS, Code::$code)
                .with_note(Note::$note)
                .with_args(MessageKind::Note, $args)
                .emmit($tokens, $span)
            {
                $diag.add_diag(dgn)
            }
        }};
    }
}
