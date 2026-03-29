pub(crate) mod builder;
pub(crate) mod code;
pub(crate) mod diagnostic;
pub(crate) mod location;
pub(crate) mod messages;

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
                .with_note(Some($crate::diagnostics::Note::$note($($args),*)))
                .with_help(Some($crate::diagnostics::Help::$help))
                .emit($tokens, $span)
            {
                $diag.add_diag(dgn)
            }
        }};

        ($pass:ident, $code:ident, $note:ident($($args:expr), *), $tokens:expr, $diag:expr, $span:expr) => {{
            if let Some(dgn) = $crate::diagnostics::DiagnosticBuilder::build(
                    $crate::config::CompilerPass::$pass,
                    $crate::diagnostics::Code::$code,
                )
                .with_note(Some($crate::diagnostics::Note::$note($($args),*)))
                .emit($tokens, $span)
            {
                $diag.add_diag(dgn)
            }
        }};
    }
}
