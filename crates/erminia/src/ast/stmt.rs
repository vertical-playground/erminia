#![allow(unused)]

use crate::ast;
use crate::types;

#[derive(Debug, PartialEq, Eq)]
pub struct ObjectDecl {
    id: String,
    etype: types::Type,
}

impl ast::StmtTrait for ObjectDecl {
    fn sem(&self) -> Result<bool, ast::ASTError> {
        todo!()
    }
    fn run(&self) -> Result<u32, ast::ASTError> {
        todo!()
    }
    fn get_scope(&self) {}
    fn set_scope(&self) {}
}

// ====================

#[derive(Debug, PartialEq, Eq)]
pub struct ProblemDef {}

impl ast::StmtTrait for ProblemDef {
    fn sem(&self /*, Semantic Table */) -> Result<bool, ast::ASTError> {
        todo!()
    }
    fn run(&self) -> Result<u32, ast::ASTError> {
        todo!()
    }
    fn get_scope(&self) {}
    fn set_scope(&self) {}
}

// ====================

#[derive(Debug, PartialEq, Eq)]
pub struct Assignment {}

impl ast::StmtTrait for Assignment {
    fn sem(&self /*, Semantic Table */) -> Result<bool, ast::ASTError> {
        todo!()
    }
    fn run(&self) -> Result<u32, ast::ASTError> {
        todo!()
    }
    fn get_scope(&self) {}
    fn set_scope(&self) {}
}

// ====================

#[derive(Debug, PartialEq, Eq)]
pub struct ExampleDecl {}

impl ast::StmtTrait for ExampleDecl {
    fn sem(&self /*, Semantic Table */) -> Result<bool, ast::ASTError> {
        todo!()
    }
    fn run(&self) -> Result<u32, ast::ASTError> {
        todo!()
    }
    fn get_scope(&self) {}
    fn set_scope(&self) {}
}

// ====================

#[derive(Debug, PartialEq, Eq)]
pub struct InputDecl {}

impl ast::StmtTrait for InputDecl {
    fn sem(&self /*, Semantic Table */) -> Result<bool, ast::ASTError> {
        todo!()
    }
    fn run(&self) -> Result<u32, ast::ASTError> {
        todo!()
    }
    fn get_scope(&self) {}
    fn set_scope(&self) {}
}

// ====================

#[derive(Debug, PartialEq, Eq)]
pub struct OutputDecl {}

impl ast::StmtTrait for OutputDecl {
    fn sem(&self /*, Semantic Table */) -> Result<bool, ast::ASTError> {
        todo!()
    }
    fn run(&self) -> Result<u32, ast::ASTError> {
        todo!()
    }
    fn get_scope(&self) {}
    fn set_scope(&self) {}
}

// ====================

#[derive(Debug, PartialEq, Eq)]
pub struct SolutionDecl {}

impl ast::StmtTrait for SolutionDecl {
    fn sem(&self /*, Semantic Table */) -> Result<bool, ast::ASTError> {
        todo!()
    }
    fn run(&self) -> Result<u32, ast::ASTError> {
        todo!()
    }
    fn get_scope(&self) {}
    fn set_scope(&self) {}
}
