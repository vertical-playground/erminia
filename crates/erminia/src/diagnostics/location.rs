use crate::lexer::lex::PositionalOffset;

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

#[derive(Default, Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct DiagnosticWindow {
    pub span: Span,
    pub snippet: String,
}
