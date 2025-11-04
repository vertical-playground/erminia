use crate::ast::stmt::*;
use crate::ast::expr::*;

pub type BoxAST = Box<dyn ASTTrait>;
pub type ASTError = String;

pub trait ASTTrait {
    fn sem(&self /*, Semantic Table */) -> Result<bool, ASTError>;
    fn print_on(&self, f: &mut std::fmt::Formatter<'_>, depth: i32) -> std::fmt::Result;
}

impl std::fmt::Debug for dyn ASTTrait {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let depth: i32 = 0;
        self.print_on(f, depth)?;
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

fn print_tabs(f: &mut std::fmt::Formatter<'_>, depth: i32) -> std::fmt::Result {
    for _ in 0..depth {
        write!(f, "\t")?;
    }
    Ok(())
}

impl ASTTrait for ASTDefault {
    fn sem(&self) -> Result<bool, ASTError> {
        Ok(true)
    }

    fn print_on(&self, f: &mut std::fmt::Formatter<'_>, mut depth: i32) -> std::fmt::Result {
        depth += 1;
        print_tabs(f, depth)?;
        writeln!(f, "<ASTDefault>")?;
        Ok(())
    }
}

impl ASTTrait for GenericTupleOption {
    fn sem(&self) -> Result<bool, ASTError> {
        todo!()
    }

    fn print_on(&self, f: &mut std::fmt::Formatter<'_>, mut depth: i32) -> std::fmt::Result {
        depth += 1;
        print_tabs(f, depth)?;
        let s = format!("<GenericTupleOption {:?}>", self);
        writeln!(f, "{}", s)?;
        Ok(())
    }
}

impl ASTTrait for ProblemExample {
    fn sem(&self) -> Result<bool, ASTError> {
        todo!()
    }

    fn print_on(&self, f: &mut std::fmt::Formatter<'_>, mut depth: i32) -> std::fmt::Result {
        depth += 1;
        print_tabs(f, depth)?;
        let s = format!("<ProblemExample id: {:?}>", self.id);
        writeln!(f, "{}", s)?;
        for stmt in &self.stmts { 
            stmt.print_on(f, depth)?;
        }
        Ok(())
    }
}

impl ASTTrait for ProblemSolution {
    fn sem(&self) -> Result<bool, ASTError> {
        todo!()
    }

    fn print_on(&self, f: &mut std::fmt::Formatter<'_>, mut depth: i32) -> std::fmt::Result {
        depth += 1;
        print_tabs(f, depth)?;
        let s = format!("<ProblemSolution id: {:?}>", self.id);
        writeln!(f, "{}", s)?;
        for stmt in &self.stmts { 
            stmt.print_on(f, depth)?;
        }
        Ok(())
    }
}

impl ASTTrait for ProblemInput {
    fn sem(&self) -> Result<bool, ASTError> {
        todo!()
    }

    fn print_on(&self, f: &mut std::fmt::Formatter<'_>, mut depth: i32) -> std::fmt::Result {
        depth += 1;
        print_tabs(f, depth)?;
        let s = format!("<ProblemInput id: {:?}>", self.id);
        writeln!(f, "{}", s)?;
        for stmt in &self.stmts { 
            stmt.print_on(f, depth)?;
        }
        Ok(())
    }
}

impl ASTTrait for ProblemOutput {
    fn sem(&self) -> Result<bool, ASTError> {
        todo!()
    }

    fn print_on(&self, f: &mut std::fmt::Formatter<'_>, mut depth: i32) -> std::fmt::Result {
        depth += 1;
        print_tabs(f, depth)?;
        let s = format!("<ProblemOutput id: {:?}>", self.id);
        writeln!(f, "{}", s)?;
        for stmt in &self.stmts { 
            stmt.print_on(f, depth)?;
        }
        Ok(())
    }
}

impl ASTTrait for Program { 
    fn sem(&self) -> Result<bool, ASTError> {
        todo!()
    }

    fn print_on(&self, f: &mut std::fmt::Formatter<'_>, depth: i32) -> std::fmt::Result {
        let s = format!("<Program id: {:?}, int_const: {}>", self.id, self.int_const);
        writeln!(f, "{}", s)?;
        for stmt in &self.stmts { 
            stmt.print_on(f, depth)?;
        }
        Ok(())
    }
}

impl ASTTrait for Range {
    fn sem(&self) -> Result<bool, ASTError> {
        todo!()
    }

    fn print_on(&self, f: &mut std::fmt::Formatter<'_>, mut depth: i32) -> std::fmt::Result {
        depth += 1;
        print_tabs(f, depth)?;
        let s = format!(
            "<Range left_inclusive: {}, right_inclusive: {}, left: {}, right: {}>",
            self.left_inclusive, self.right_inclusive, self.left, self.right
        );
        writeln!(f, "{}", s)?;
        Ok(())
    }
}

impl ASTTrait for TupleIterator {
    fn sem(&self) -> Result<bool, ASTError> {
        todo!()
    }

    fn print_on(&self, f: &mut std::fmt::Formatter<'_>, mut depth: i32) -> std::fmt::Result {
        depth += 1;
        print_tabs(f, depth)?;
        let s = format!("<TupleIterator id: {:?}>", self.id);
        writeln!(f, "{}", s)?;
        let _ = &self.range.print_on(f, depth)?;
        Ok(())
    }
}

impl ASTTrait for TupleComprehension {
    fn sem(&self) -> Result<bool, ASTError> {
        todo!()
    }

    fn print_on(&self, f: &mut std::fmt::Formatter<'_>, mut depth: i32) -> std::fmt::Result {
        depth += 1;
        print_tabs(f, depth)?;
        writeln!(f, "<TupleComprehension>")?;
        let _ = &self.tuple.print_on(f, depth)?;
        for iter in &self.iter_pair {
            iter.print_on(f, depth)?;
        }
        Ok(())
    }
}


impl ASTTrait for GenericTuple {
    fn sem(&self) -> Result<bool, ASTError> {
        todo!()
    }

    fn print_on(&self, f: &mut std::fmt::Formatter<'_>, mut depth: i32) -> std::fmt::Result {
        depth += 1;
        print_tabs(f, depth)?;
        writeln!(f, "<GenericTuple>")?;
        let _ = &self.left.print_on(f, depth)?;
        let _ = &self.right.print_on(f, depth)?;
        Ok(())
    }
}

impl ASTTrait for Tuple {
    fn sem(&self) -> Result<bool, ASTError> {
        todo!()
    }

    fn print_on(&self, f: &mut std::fmt::Formatter<'_>, mut depth: i32) -> std::fmt::Result {
        depth += 1;
        print_tabs(f, depth)?;
        let s = format!("<Tuple left: {}, right: {}>", self.left, self.right);
        writeln!(f, "{}", s)?;
        Ok(())
    }
}

impl ASTTrait for Shape {
    fn sem(&self) -> Result<bool, ASTError> {
        todo!()
    }

    fn print_on(&self, f: &mut std::fmt::Formatter<'_>, mut depth: i32) -> std::fmt::Result {
        depth += 1;
        print_tabs(f, depth)?;
        let s = format!("<Shape type: {:?}>", self.shape_type);
        writeln!(f, "{}", s)?;
        let _ = &self.values.print_on(f, depth)?;
        Ok(())
    }
}


impl ASTTrait for ObjectShape {
    fn sem(&self) -> Result<bool, ASTError> {
        todo!()
    }

    fn print_on(&self, f: &mut std::fmt::Formatter<'_>, mut depth: i32) -> std::fmt::Result {
        depth += 1;
        print_tabs(f, depth)?;
        writeln!(f, "<ObjectShape>")?;
        for shape in &self.shape {
            shape.print_on(f, depth)?;
        }
        Ok(())
    }
}

impl ASTTrait for ObjectColor {
    fn sem(&self) -> Result<bool, ASTError> {
        todo!()
    }

    fn print_on(&self, f: &mut std::fmt::Formatter<'_>, mut depth: i32) -> std::fmt::Result {
        depth += 1;
        print_tabs(f, depth)?;
        let s = format!("<ObjectColor color: {}>", self.color);
        writeln!(f, "{}", s)?;
        Ok(())
    }
}

impl ASTTrait for ObjectDesc {
    fn sem(&self) -> Result<bool, ASTError> {
        todo!()
    }

    fn print_on(&self, f: &mut std::fmt::Formatter<'_>, depth: i32) -> std::fmt::Result {
        print_tabs(f, depth)?;
        writeln!(f, "<ObjectDesc>")?;
        let _ = &self.shape.print_on(f, depth)?;
        let _ = &self.color.print_on(f, depth)?;
        Ok(())
    }
}

impl ASTTrait for ObjectDecl {
    fn sem(&self) -> Result<bool, ASTError> {
        todo!()
    }

    fn print_on(&self, f: &mut std::fmt::Formatter<'_>, mut depth: i32) -> std::fmt::Result {
        depth += 1;
        print_tabs(f, depth)?;
        let s = format!("<ObjectDecl id: {:?}>", self.id);
        writeln!(f, "{}", s)?;
        let _ = &self.desc.print_on(f, depth)?;
        Ok(())
    }
}

impl ASTTrait for VarDef {
    fn sem(&self) -> Result<bool, ASTError> {
        todo!()
    }

    fn print_on(&self, f: &mut std::fmt::Formatter<'_>, mut depth: i32) -> std::fmt::Result {
        depth += 1;
        print_tabs(f, depth)?;
        let s = format!("<VarDef id: {:?}, data_type: {:?}>", self.id, self.data_type);
        writeln!(f, "{}", s)?;
        let _ = &self.expr.print_on(f, depth)?;
        Ok(())
    }
}

impl ASTTrait for FuncCall { 
    fn sem(&self) -> Result<bool, ASTError> {
        todo!()
    }

    fn print_on(&self, f: &mut std::fmt::Formatter<'_>, mut depth: i32) -> std::fmt::Result {
        depth += 1;
        print_tabs(f, depth)?;
        let s = format!("<FuncCall id: {:?}>", self.id);
        writeln!(f, "{}", s)?;
        for expr in &self.exprs {
            expr.print_on(f, depth)?;
        }
        Ok(())
    }
}

impl ASTTrait for ObjectCall { 
    fn sem(&self) -> Result<bool, ASTError> {
        todo!()
    }

    fn print_on(&self, f: &mut std::fmt::Formatter<'_>, mut depth: i32) -> std::fmt::Result {
        depth += 1;
        print_tabs(f, depth)?;
        let s = format!("<ObjectCall id: {:?}>", self.id);
        writeln!(f, "{}", s)?;
        if let Some(t) = &self.tuple {
            t.print_on(f, depth)?;
        };
        Ok(())
    }
}

impl ASTTrait for RValue { 
    fn sem(&self) -> Result<bool, ASTError> {
        todo!()
    }

    fn print_on(&self, f: &mut std::fmt::Formatter<'_>, mut depth: i32) -> std::fmt::Result {
        depth += 1;
        print_tabs(f, depth)?;
        let s = format!("<RValue {:?}>", self);
        writeln!(f, "{}", s)?;
        Ok(())
    }
}
