use crate::ast::ast_trait::{ASTError, ASTTrait, BoxAST};

pub type BoxExpr = Box<dyn ExprTrait>;

// ==================================================================================== //
//  Traits                                                                              //
// ==================================================================================== //

pub trait ExprTrait: ASTTrait {
    fn eval(&self) -> Result<u32, ASTError>;
}

// ==================================================================================== //
//  Structs                                                                             //
// ==================================================================================== //

pub struct FuncCall {
    pub id: String,
    pub exprs: Vec<BoxAST>,
}

pub struct ObjectCall {
    pub id: String,
    pub tuple: Option<BoxAST>,
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
    pub fn boxed(id: String, exprs: Vec<BoxAST>) -> BoxAST {
        Box::new(FuncCall { id, exprs })
    }
}

impl ObjectCall {
    pub fn boxed(id: String, tuple: Option<BoxAST>) -> BoxAST {
        Box::new(ObjectCall { id, tuple })
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
