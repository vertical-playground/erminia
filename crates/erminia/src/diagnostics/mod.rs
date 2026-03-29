pub mod builder;
pub mod code;
pub mod diagnostic;
pub mod location;
pub mod messages;

pub use builder::DiagnosticBuilder;
pub use code::Code;
pub use diagnostic::{create_diagnostic, Diagnostic, DiagnosticAccumulator};
pub use location::{DiagnosticWindow, Span};
pub use messages::{Help, MessageKind, Note};

pub mod macros {
    #[macro_export]
    macro_rules! diag {
        ($pass:ident, $code:ident, $note:ident($($args:expr), *), $help:ident, $tokens:expr, $diag:expr, $span:expr) => {{
            if let Some(dgn) = $crate::diagnostics::DiagnosticBuilder::build(
                    $crate::config::CompilerPass::$pass,
                    $crate::diagnostics::Code::$code,
                )
                .with_note($crate::diagnostics::Note::$note($($args),*))
                .with_args($crate::diagnostics::MessageKind::Note)
                .with_help($crate::diagnostics::Help::$help)
                .emmit($tokens, $span)
            {
                $diag.add_diag(dgn)
            }
        }};
    }
}
