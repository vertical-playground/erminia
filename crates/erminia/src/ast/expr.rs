use crate::ast::ast::{ASTError, BoxAST, AST};
use crate::diagnostics::location::Span;
use crate::types::ErminiaType;

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
    pub id: ErminiaType,
    pub exprs: Vec<BoxAST<'a>>,
    pub span: Span,
    pub is_poisoned: bool,
    pub unique_ast_id: u32,
}

pub struct ObjectCall<'a> {
    pub id: ErminiaType,
    pub tuple: Option<BoxAST<'a>>,
    pub span: Span,
    pub is_poisoned: bool,
    pub unique_ast_id: u32,
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
    pub fn boxed(id: ErminiaType, exprs: Vec<BoxAST<'a>>, span: Span) -> BoxAST<'a> {
        let unique_ast_id = 0;
        let mut is_poisoned = false;

        if id.is_poisoned() {
            is_poisoned = true;
        }

        if exprs.iter().any(|e| e.is_err()) {
            is_poisoned = true;
        }

        Box::new(FuncCall {
            id,
            exprs,
            span,
            is_poisoned,
            unique_ast_id,
        }) as BoxAST<'a>
    }
}

impl<'a> ObjectCall<'a> {
    pub fn boxed(id: ErminiaType, tuple: Option<BoxAST<'a>>, span: Span) -> BoxAST<'a> {
        let unique_ast_id = 0;
        let mut is_poisoned = false;

        if let Some(t) = &tuple {
            if t.is_err() {
                is_poisoned = true;
            }
        }

        if id.is_poisoned() {
            is_poisoned = true;
        }

        Box::new(ObjectCall {
            id,
            tuple,
            span,
            is_poisoned,
            unique_ast_id,
        }) as BoxAST<'a>
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
