use crate::ast::stmt::*;
use crate::ast::expr::*;

pub type BoxAST = Box<dyn ASTTrait>;
pub type ASTError = String;

pub trait ASTTrait {
    fn sem(&self /*, Semantic Table */) -> Result<bool, ASTError>;
    fn print_on(&self) -> Result<(), ASTError>;
}

impl std::fmt::Debug for dyn ASTTrait {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ASTTrait")
    }
}

pub struct ASTDefault {}

impl ASTDefault {
    pub fn new() -> BoxAST {
        Box::new(ASTDefault {})
    }
}

// ==================================================================================== //
//  Implementations                                                                     //
// ==================================================================================== //

impl ASTTrait for ASTDefault {
    fn sem(&self) -> Result<bool, ASTError> {
        Ok(true)
    }

    fn print_on(&self) -> Result<(), ASTError> {
        todo!()
    }
}

impl ASTTrait for GenericTupleOption {
    fn sem(&self) -> Result<bool, ASTError> {
        todo!()
    }

    fn print_on(&self) -> Result<(), ASTError> {
        todo!()
    }
}

impl ASTTrait for ProblemExample {
    fn sem(&self) -> Result<bool, ASTError> {
        todo!()
    }

    fn print_on(&self) -> Result<(), ASTError> {
        todo!()
    }
}

impl ASTTrait for Program { 
    fn sem(&self) -> Result<bool, ASTError> {
        todo!()
    }

    fn print_on(&self) -> Result<(), ASTError> {
        todo!()
    }
}

impl ASTTrait for GenericTuple {
    fn sem(&self) -> Result<bool, ASTError> {
        todo!()
    }

    fn print_on(&self) -> Result<(), ASTError> {
        todo!()
    }
}

impl ASTTrait for Tuple {
    fn sem(&self) -> Result<bool, ASTError> {
        todo!()
    }

    fn print_on(&self) -> Result<(), ASTError> {
        todo!()
    }
}

impl ASTTrait for Shape {
    fn sem(&self) -> Result<bool, ASTError> {
        todo!()
    }

    fn print_on(&self) -> Result<(), ASTError> {
        todo!()
    }
}

impl ASTTrait for ObjectShape {
    fn sem(&self) -> Result<bool, ASTError> {
        todo!()
    }

    fn print_on(&self) -> Result<(), ASTError> {
        todo!()
    }
}

impl ASTTrait for ObjectColor {
    fn sem(&self) -> Result<bool, ASTError> {
        todo!()
    }

    fn print_on(&self) -> Result<(), ASTError> {
        todo!()
    }
}

impl ASTTrait for ObjectDesc {
    fn sem(&self) -> Result<bool, ASTError> {
        todo!()
    }

    fn print_on(&self) -> Result<(), ASTError> {
        todo!()
    }
}

impl ASTTrait for ObjectDecl {
    fn sem(&self) -> Result<bool, ASTError> {
        todo!()
    }

    fn print_on(&self) -> Result<(), ASTError> {
        todo!()
    }
}

impl ASTTrait for VarDef {
    fn sem(&self) -> Result<bool, ASTError> {
        todo!()
    }

    fn print_on(&self) -> Result<(), ASTError> {
        todo!()
    }
}

impl ASTTrait for FuncCall { 
    fn sem(&self) -> Result<bool, ASTError> {
        todo!()
    }

    fn print_on(&self) -> Result<(), ASTError> {
        todo!()
    }
}

impl ASTTrait for ObjectCall { 
    fn sem(&self) -> Result<bool, ASTError> {
        todo!()
    }

    fn print_on(&self) -> Result<(), ASTError> {
        todo!()
    }
}

impl ASTTrait for RValue { 
    fn sem(&self) -> Result<bool, ASTError> {
        todo!()
    }

    fn print_on(&self) -> Result<(), ASTError> {
        todo!()
    }
}
