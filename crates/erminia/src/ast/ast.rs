#![allow(unused)]

use crate::expr;
use crate::stmt;

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
    ObjectDecl(stmt::ObjectDecl),
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
    FuncCall(expr::FuncCall),
}
