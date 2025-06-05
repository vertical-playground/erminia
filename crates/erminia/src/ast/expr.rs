#![allow(unused)]

use crate::types;
use crate::ast;

#[derive(Debug, PartialEq, Eq)]
pub struct Param {

}

impl ast::ExprTrait for Param {
    fn sem(&self /*, Semantic Table */) -> Result<bool, ast::ASTError> { todo!() }
    fn eval(&self) -> Result<u32, ast::ASTError> { todo!() }
    fn get_scope(&self) { todo!() }
    fn set_scope(&self) { todo!() }
}


// ====================

#[derive(Debug, PartialEq, Eq)]
pub struct Int {

}

impl ast::ExprTrait for Int {
    fn sem(&self /*, Semantic Table */) -> Result<bool, ast::ASTError> { todo!() }
    fn eval(&self) -> Result<u32, ast::ASTError> { todo!() }
    fn get_scope(&self) { todo!() }
    fn set_scope(&self) { todo!() }
}

// ====================

#[derive(Debug, PartialEq, Eq)]
pub struct String {

}

impl ast::ExprTrait for String {
    fn sem(&self /*, Semantic Table */) -> Result<bool, ast::ASTError> { todo!() }
    fn eval(&self) -> Result<u32, ast::ASTError> { todo!() }
    fn get_scope(&self) { todo!() }
    fn set_scope(&self) { todo!() }
}

// ====================

#[derive(Debug, PartialEq, Eq)]
pub struct ObjectCall {

}

impl ast::ExprTrait for ObjectCall {
    fn sem(&self /*, Semantic Table */) -> Result<bool, ast::ASTError> { todo!() }
    fn eval(&self) -> Result<u32, ast::ASTError> { todo!() }
    fn get_scope(&self) { todo!() }
    fn set_scope(&self) { todo!() }
}

// ====================

#[derive(Debug, PartialEq, Eq)]
pub struct FuncCall {

}

impl ast::ExprTrait for FuncCall {
    fn sem(&self /*, Semantic Table */) -> Result<bool, ast::ASTError> { todo!() }
    fn eval(&self) -> Result<u32, ast::ASTError> { todo!() }
    fn get_scope(&self) { todo!() }
    fn set_scope(&self) { todo!() }
}

