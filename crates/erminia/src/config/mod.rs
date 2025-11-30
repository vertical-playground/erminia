use derive_more::Display;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Display, PartialOrd, Ord)]
pub enum CompilerPass {
    Internal,
    Lexer,
    Parser,
    AST,
    Semantics,
    ALL,
}
