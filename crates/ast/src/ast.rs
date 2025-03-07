use crate::expr;
use crate::stmt;

pub enum ASTNode {
    Stmt(Stmt),
    Expr(Expr)
}

pub enum Stmt { 
    ObjectDecl(stmt::ObjectDecl),
    SuperObjectDecl(stmt::SuperObjectDecl),
    ProblemDef(stmt::ProblemDef),
    Assignment(stmt::Assignment),
    ExampleDecl(stmt::ExampleDecl),
    InputDecl(stmt::InputDecl),
    OutputDecl(stmt::OutputDecl),
    SolutionDecl(stmt::SolutionDecl)
}

pub enum Expr {
    Param(expr::Param), 
    Int(expr::Int),
    String(expr::String),
    ObjectCall(expr::ObjectCall),
    SuperObjectCall(expr::SuperObjectCall),
    FuncCall(expr::FuncCall),
}
