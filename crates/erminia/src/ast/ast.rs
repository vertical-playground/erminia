use crate::ast::expr;
use crate::ast::stmt;

pub type ASTError = String;

pub trait StmtTrait {
    fn sem(&self /*, Semantic Table */) -> Result<bool, ASTError>;
    fn run(&self) -> Result<u32, ASTError>;
    fn get_scope(&self);
    fn set_scope(&self);
}

pub trait ExprTrait {
    fn sem(&self /*, Semantic Table */) -> Result<bool, ASTError>;
    fn eval(&self) -> Result<u32, ASTError>;
    fn get_scope(&self);
    fn set_scope(&self);
}

pub enum Stmt {
    ObjectDecl(ObjectDecl),
    ProblemDef(ProblemDef),
    Assignment(Assignment),
    ExampleDecl(ExampleDecl),
    InputDecl(InputDecl),
    OutputDecl(OutputDecl),
    SolutionDecl(SolutionDecl),
}

pub enum Expr {
    Param(Param),
    Int(Int),
    String(String),
    ObjectCall(ObjectCall),
    FuncCall(FuncCall),
}
