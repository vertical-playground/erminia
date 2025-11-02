use crate::ast::ast::{BoxAST, ASTTrait, ASTError};

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
    id: String,
    expr: Vec<BoxAST>
}

pub struct ObjectCall {
    id: String,
    tuple: Option<BoxAST> 
}

pub enum RValue {
    Int(i32),
    Id(String),
}

// ==================================================================================== //
//  Implementations                                                                     //
// ==================================================================================== //

impl FuncCall {
    pub fn new(id: String, exprs: Vec<BoxAST>) -> BoxAST {
        Box::new(FuncCall { id, expr: exprs })
    }
}

impl ObjectCall {
    pub fn new(id: String, tuple: Option<BoxAST>) -> BoxAST {
        Box::new(ObjectCall { id, tuple })
    }
}

impl RValue {
    pub fn new_int(value: i32) -> BoxAST {
        Box::new(RValue::Int(value))
    }

    pub fn new_id(name: String) -> BoxAST {
        Box::new(RValue::Id(name))
    }
}
