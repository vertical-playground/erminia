use crate::ast::ast::{ASTError, BoxAST, AST};
use crate::diagnostics::location::Span;

pub type BoxExpr<'a> = Box<dyn ExprTrait<'a> + 'a>;

// ==================================================================================== //
//  Traits                                                                              //
// ==================================================================================== //

pub trait ExprTrait<'a>: AST<'a> {
    fn eval(&self) -> Result<u32, ASTError>;
}

// ==================================================================================== //
//  Structs                                                                             //
// ==================================================================================== //

pub struct FuncCall<'a> {
    pub id: String,
    pub exprs: Vec<BoxAST<'a>>,
    pub span: Span,
    pub is_poisoned: bool,
}

pub struct ObjectCall<'a> {
    pub id: String,
    pub tuple: Option<BoxAST<'a>>,
    pub span: Span,
    pub is_poisoned: bool,
}

#[derive(Debug)]
pub enum RValue {
    Int(i32),
    Id(String),
}

// ==================================================================================== //
//  Implementations                                                                     //
// ==================================================================================== //

impl<'a> FuncCall<'a> {
    pub fn boxed(id: String, exprs: Vec<BoxAST<'a>>, span: Span, is_poisoned: bool) -> BoxAST<'a> {
        Box::new(FuncCall { id, exprs, span, is_poisoned }) as BoxAST<'a>
    }
}

impl<'a> ObjectCall<'a> {
    pub fn boxed(id: String, tuple: Option<BoxAST<'a>>, span: Span, is_poisoned: bool) -> BoxAST<'a> {
        Box::new(ObjectCall { id, tuple, span, is_poisoned }) as BoxAST<'a>
    }
}

impl<'a> RValue {
    pub fn boxed_int(value: i32) -> BoxAST<'a> {
        Box::new(RValue::Int(value))
    }

    pub fn boxed_id(name: String) -> BoxAST<'a> {
        Box::new(RValue::Id(name))
    }
}
