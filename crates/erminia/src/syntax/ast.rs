use crate::error::ast_error::ASTResult;

// ====================================================================================//
// AST                                                                                 //
// ====================================================================================//

#[derive(Debug, PartialEq)]
pub struct ObjectDecl {}

impl ObjectDecl {
    pub fn new() -> ObjectDecl {
        ObjectDecl {}
    }
}

#[derive(Debug, PartialEq)]
pub struct Program {
    id: String,
    int_const: i32,
    // stmts: Vec<Stmt>,
}

impl Program {
    pub fn new(id: String, int_const: i32, _stmts: () /* Vec<Stmt> */) -> Program {
        Program {
            id: id,
            int_const: int_const,
            // stmts: stmts,
        }
    }
}

pub trait StmtTrait {
    fn sem(&self /*, Semantic Table */) -> ASTResult<()>;
    // fn run(&self) -> Result<u32, ASTError>;
    // fn get_scope(&self);
    // fn set_scope(&self);
}

#[derive(Debug, PartialEq)]
pub enum Stmt {
    ObjectDecl(ObjectDecl),
    Program(Program),
}
