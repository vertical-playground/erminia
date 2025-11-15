use crate::ast::ast::{ASTError, BoxAST, AST};
use crate::diagnostics::location::Span;

pub type BoxExpr = Box<dyn ExprTrait>;

// ==================================================================================== //
//  Traits                                                                              //
// ==================================================================================== //

pub trait ExprTrait: AST {
    fn eval(&self) -> Result<u32, ASTError>;
}

// ==================================================================================== //
//  Structs                                                                             //
// ==================================================================================== //

pub struct FuncCall {
    pub id: String,
    pub exprs: Vec<BoxAST>,
    pub span: Span,
}

pub struct ObjectCall {
    pub id: String,
    pub tuple: Option<BoxAST>,
    pub span: Span,
}

#[derive(Debug)]
pub enum RValue {
    Int(i32),
    Id(String),
}

// ==================================================================================== //
//  Implementations                                                                     //
// ==================================================================================== //

impl FuncCall {
    pub fn boxed(id: String, exprs: Vec<BoxAST>, span: Span) -> BoxAST {
        Box::new(FuncCall { id, exprs, span })
    }
}

impl ObjectCall {
    pub fn boxed(id: String, tuple: Option<BoxAST>, span: Span) -> BoxAST {
        Box::new(ObjectCall { id, tuple, span })
    }
}

impl RValue {
    pub fn boxed_int(value: i32) -> BoxAST {
        Box::new(RValue::Int(value))
    }

    pub fn boxed_id(name: String) -> BoxAST {
        Box::new(RValue::Id(name))
    }
}
