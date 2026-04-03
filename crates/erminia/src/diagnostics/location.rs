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

    pub fn from_line(line: usize) -> Self {
        let start = PositionalOffset::new(0, 1, line);
        let end = PositionalOffset::new(0, 1, line + 1);
        Span::new(start, end)
    }
}

#[derive(Default, Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct DiagnosticWindow {
    pub span: Span,
    pub snippet: String,
}
