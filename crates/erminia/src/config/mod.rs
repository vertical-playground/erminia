use derive_more::Display;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Display, PartialOrd, Ord)]
pub enum CompilerPass {
    Lexer,
    Parser,
    AST,
    Semantics,
    ALL,
}
