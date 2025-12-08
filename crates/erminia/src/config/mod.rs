use derive_more::Display;

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, Display, PartialOrd, Ord)]
pub enum CompilerPass {
    #[default]
    Internal,
    Lexer,
    Parser,
    AST,
    Semantics,
    ALL,
}
