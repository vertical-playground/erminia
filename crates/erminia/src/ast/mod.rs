#[allow(clippy::module_inception)]
pub mod ast;
pub mod expr;
pub mod stmt;

pub mod macros {
    #[macro_export]
    macro_rules! ast_diag {
        ($self:expr, $tokens:expr, $diag:expr, $span:expr) => {{
            let diagnostic = DB::build(AST_PASS, Code::E0004)
                .with_note(Note::ExpectedASTNode)
                .with_args(
                    MessageKind::Note,
                    vec![$self.to_string(), $self.get_ast_id().to_string()],
                )
                .emmit($tokens, $span);

            if let Some(dgn) = diagnostic {
                $diag.add_diag(dgn)
            }
        }};
    }
}
