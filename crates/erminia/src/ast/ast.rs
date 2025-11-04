use crate::ast::stmt::*;
use crate::ast::expr::*;

pub type BoxAST = Box<dyn ASTTrait>;
pub type ASTError = String;

pub trait ASTTrait {
    fn sem(&self /*, Semantic Table */) -> Result<bool, ASTError>;
    fn print_on(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result;
}

impl std::fmt::Debug for dyn ASTTrait {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.print_on(f)?;
        Ok(())
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

    fn print_on(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "<ASTDefault>")?;
        Ok(())
    }
}

impl ASTTrait for GenericTupleOption {
    fn sem(&self) -> Result<bool, ASTError> {
        todo!()
    }

    fn print_on(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "<GenericTupleOption>")?;
        Ok(())
    }
}

impl ASTTrait for ProblemExample {
    fn sem(&self) -> Result<bool, ASTError> {
        todo!()
    }

    fn print_on(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "<ProblemExample>")?;
        for stmt in &self.stmts { 
            stmt.print_on(f)?;
        }
        Ok(())
    }
}

impl ASTTrait for Program { 
    fn sem(&self) -> Result<bool, ASTError> {
        todo!()
    }

    fn print_on(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "<Program>")?;
        for stmt in &self.stmts { 
            stmt.print_on(f)?;
        }
        Ok(())
    }
}

impl ASTTrait for Range {
    fn sem(&self) -> Result<bool, ASTError> {
        todo!()
    }

    fn print_on(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "<Range>")?;
        Ok(())
    }
}

impl ASTTrait for TupleIterator {
    fn sem(&self) -> Result<bool, ASTError> {
        todo!()
    }

    fn print_on(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "<TupleIterator>")?;
        let _ = &self.range.print_on(f)?;
        Ok(())
    }
}

impl ASTTrait for TupleComprehension {
    fn sem(&self) -> Result<bool, ASTError> {
        todo!()
    }

    fn print_on(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "<TupleComprehension>")?;
        writeln!(f, "\t")?;
        let _ = &self.tuple.print_on(f)?;
        for iter in &self.iter_pair {
            iter.print_on(f)?;
        }
        Ok(())
    }
}


impl ASTTrait for GenericTuple {
    fn sem(&self) -> Result<bool, ASTError> {
        todo!()
    }

    fn print_on(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "<GenericTuple>")?;
        let _ = &self.left.print_on(f)?;
        let _ = &self.right.print_on(f)?;
        Ok(())
    }
}

impl ASTTrait for Tuple {
    fn sem(&self) -> Result<bool, ASTError> {
        todo!()
    }

    fn print_on(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "<Tuple>")?;
        Ok(())
    }
}

impl ASTTrait for Shape {
    fn sem(&self) -> Result<bool, ASTError> {
        todo!()
    }

    fn print_on(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "<Shape>")?;
        let _ = &self.values.print_on(f)?;
        Ok(())
    }
}

impl ASTTrait for ObjectShape {
    fn sem(&self) -> Result<bool, ASTError> {
        todo!()
    }

    fn print_on(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "<ObjectShape>")?;
        for shape in &self.shape {
            shape.print_on(f)?;
        }
        Ok(())
    }
}

impl ASTTrait for ObjectColor {
    fn sem(&self) -> Result<bool, ASTError> {
        todo!()
    }

    fn print_on(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "<ObjectColor>")?;
        Ok(())
    }
}

impl ASTTrait for ObjectDesc {
    fn sem(&self) -> Result<bool, ASTError> {
        todo!()
    }

    fn print_on(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "<ObjectDesc>")?;
        let _ = &self.shape.print_on(f)?;
        let _ = &self.color.print_on(f)?;
        Ok(())
    }
}

impl ASTTrait for ObjectDecl {
    fn sem(&self) -> Result<bool, ASTError> {
        todo!()
    }

    fn print_on(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "<ObjectDecl>")?;
        let _ = &self.desc.print_on(f)?;
        Ok(())
    }
}

impl ASTTrait for VarDef {
    fn sem(&self) -> Result<bool, ASTError> {
        todo!()
    }

    fn print_on(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "<VarDef>")?;
        let _ = &self.expr.print_on(f)?;
        Ok(())
    }
}

impl ASTTrait for FuncCall { 
    fn sem(&self) -> Result<bool, ASTError> {
        todo!()
    }

    fn print_on(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "<FuncCall>")?;
        for expr in &self.exprs {
            expr.print_on(f)?;
        }
        Ok(())
    }
}

impl ASTTrait for ObjectCall { 
    fn sem(&self) -> Result<bool, ASTError> {
        todo!()
    }

    fn print_on(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "<ObjectCall>")?;
        if let Some(t) = &self.tuple {
            t.print_on(f)?;
        };
        Ok(())
    }
}

impl ASTTrait for RValue { 
    fn sem(&self) -> Result<bool, ASTError> {
        todo!()
    }

    fn print_on(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "<RValue>")?;
        Ok(())
    }
}
