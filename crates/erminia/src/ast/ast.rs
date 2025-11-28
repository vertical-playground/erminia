use crate::ast::expr::*;
use crate::ast::stmt::*;

pub type BoxAST<'a> = Box<dyn AST<'a> + 'a>;
pub type ASTError = String;

#[derive(Debug)]
pub enum ASTResult<'a> {
    One(BoxAST<'a>),
    Many(Vec<BoxAST<'a>>),
}

impl<'a> ASTResult<'a> {
    pub fn is_one(&self) -> bool {
        matches!(self, ASTResult::One(_))
    }

    pub fn is_many(&self) -> bool {
        matches!(self, ASTResult::Many(_))
    }

    pub fn is_err(&self) -> bool {
        match self {
            ASTResult::One(ast) => ast.is_err(),
            ASTResult::Many(asts) => asts.iter().any(|ast| ast.is_err()),
        }
    }

    pub fn is_ok(&self) -> bool {
        match self {
            ASTResult::One(ast) => ast.is_ok(),
            ASTResult::Many(asts) => asts.iter().all(|ast| ast.is_ok()),
        }
    }

}

pub trait AST<'a>: 'a {
    fn sem(&self /*, Semantic Table */) -> Result<bool, ASTError>;
    fn is_err(&self) -> bool;
    fn is_ok(&self) -> bool; 
    fn print_on(&self, f: &mut std::fmt::Formatter<'_>, depth: i32) -> std::fmt::Result;
}

impl<'a> std::fmt::Debug for dyn AST<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let depth: i32 = 0;
        self.print_on(f, depth)?;
        Ok(())
    }
}

// pub struct ASTDefault<'a> {}
//
// impl<'a> ASTDefault<'a> {
//     pub fn boxed() -> BoxAST<'a> {
//         Box::new(ASTDefault {}) 
//     }
// }

// ==================================================================================== //
//  Implementations                                                                     //
// ==================================================================================== //

fn print_tabs(f: &mut std::fmt::Formatter<'_>, depth: i32) -> std::fmt::Result {
    for _ in 0..depth {
        write!(f, "\t")?;
    }
    Ok(())
}

// impl<'a> AST<'a> for ASTDefault<'a> {
//     fn sem(&self) -> Result<bool, ASTError> {
//         Ok(true)
//     }
//
//     fn is_err(&self) -> bool {
//         false
//     }
//
//     fn is_ok(&self) -> bool {
//         true
//     }
//
//     fn print_on(&self, f: &mut std::fmt::Formatter<'_>, mut depth: i32) -> std::fmt::Result {
//         depth += 1;
//         print_tabs(f, depth)?;
//         writeln!(f, "<ASTDefault>")?;
//         Ok(())
//     }
// }

impl<'a> AST<'a> for GenericTupleOption {
    fn sem(&self) -> Result<bool, ASTError> {
        todo!()
    }

    fn is_err(&self) -> bool {
        false
    }

    fn is_ok(&self) -> bool {
        true
    }

    fn print_on(&self, f: &mut std::fmt::Formatter<'_>, mut depth: i32) -> std::fmt::Result {
        depth += 1;
        print_tabs(f, depth)?;
        let s = format!("<GenericTupleOption {:?}>", self);
        writeln!(f, "{}", s)?;
        Ok(())
    }
}

impl<'a> AST<'a> for ProblemExample<'a> {
    fn sem(&self) -> Result<bool, ASTError> {
        todo!()
    }

    fn is_err(&self) -> bool {
        self.is_poisoned
    }

    fn is_ok(&self) -> bool {
        !self.is_poisoned
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

impl<'a> AST<'a> for ProblemSolution<'a> {
    fn sem(&self) -> Result<bool, ASTError> {
        todo!()
    }

    fn is_err(&self) -> bool {
        self.is_poisoned
    }

    fn is_ok(&self) -> bool {
        !self.is_poisoned
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

impl<'a> AST<'a> for ProblemInput<'a> {
    fn sem(&self) -> Result<bool, ASTError> {
        todo!()
    }

    fn is_err(&self) -> bool {
        self.is_poisoned
    }

    fn is_ok(&self) -> bool {
        !self.is_poisoned
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

impl<'a> AST<'a> for ProblemOutput<'a> {
    fn sem(&self) -> Result<bool, ASTError> {
        todo!()
    }

    fn is_err(&self) -> bool {
        self.is_poisoned
    }

    fn is_ok(&self) -> bool {
        !self.is_poisoned
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

impl<'a> AST<'a> for Program<'a> {
    fn sem(&self) -> Result<bool, ASTError> {
        todo!()
    }

    fn is_err(&self) -> bool {
        self.is_poisoned
    }

    fn is_ok(&self) -> bool {
        !self.is_poisoned
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

impl<'a> AST<'a> for Range {
    fn sem(&self) -> Result<bool, ASTError> {
        todo!()
    }

    fn is_err(&self) -> bool {
        self.is_poisoned
    }

    fn is_ok(&self) -> bool {
        !self.is_poisoned
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

impl<'a> AST<'a> for TupleIterator<'a> {
    fn sem(&self) -> Result<bool, ASTError> {
        todo!()
    }

    fn is_err(&self) -> bool {
        self.is_poisoned
    }

    fn is_ok(&self) -> bool {
        !self.is_poisoned
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

impl<'a> AST<'a> for TupleComprehension<'a> {
    fn sem(&self) -> Result<bool, ASTError> {
        todo!()
    }

    fn is_err(&self) -> bool {
        self.is_poisoned
    }

    fn is_ok(&self) -> bool {
        !self.is_poisoned
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

impl<'a> AST<'a> for GenericTuple<'a> {
    fn sem(&self) -> Result<bool, ASTError> {
        todo!()
    }

    fn is_err(&self) -> bool {
        self.is_poisoned
    }

    fn is_ok(&self) -> bool {
        !self.is_poisoned
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

impl<'a> AST<'a> for Tuple {
    fn sem(&self) -> Result<bool, ASTError> {
        todo!()
    }

    fn is_err(&self) -> bool {
        self.is_poisoned
    }

    fn is_ok(&self) -> bool {
        !self.is_poisoned
    }

    fn print_on(&self, f: &mut std::fmt::Formatter<'_>, mut depth: i32) -> std::fmt::Result {
        depth += 1;
        print_tabs(f, depth)?;
        let s = format!("<Tuple left: {}, right: {}>", self.left, self.right);
        writeln!(f, "{}", s)?;
        Ok(())
    }
}

impl<'a> AST<'a> for Shape<'a> {
    fn sem(&self) -> Result<bool, ASTError> {
        todo!()
    }

    fn is_err(&self) -> bool {
        self.is_poisoned
    }

    fn is_ok(&self) -> bool {
        !self.is_poisoned
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

impl<'a> AST<'a> for ObjectShape<'a> {
    fn sem(&self) -> Result<bool, ASTError> {
        todo!()
    }

    fn is_err(&self) -> bool {
        self.is_poisoned
    }

    fn is_ok(&self) -> bool {
        !self.is_poisoned
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

impl<'a> AST<'a> for ObjectColor {
    fn sem(&self) -> Result<bool, ASTError> {
        todo!()
    }

    fn is_err(&self) -> bool {
        self.is_poisoned
    }

    fn is_ok(&self) -> bool {
        !self.is_poisoned
    }

    fn print_on(&self, f: &mut std::fmt::Formatter<'_>, mut depth: i32) -> std::fmt::Result {
        depth += 1;
        print_tabs(f, depth)?;
        let s = format!("<ObjectColor color: {}>", self.color);
        writeln!(f, "{}", s)?;
        Ok(())
    }
}

impl<'a> AST<'a> for ObjectDesc<'a> {
    fn sem(&self) -> Result<bool, ASTError> {
        todo!()
    }

    fn is_err(&self) -> bool {
        self.is_poisoned
    }

    fn is_ok(&self) -> bool {
        !self.is_poisoned
    }

    fn print_on(&self, f: &mut std::fmt::Formatter<'_>, depth: i32) -> std::fmt::Result {
        print_tabs(f, depth)?;
        writeln!(f, "<ObjectDesc>")?;
        let _ = &self.shape.print_on(f, depth)?;
        let _ = &self.color.print_on(f, depth)?;
        Ok(())
    }
}

impl<'a> AST<'a> for ObjectDecl<'a> {
    fn sem(&self) -> Result<bool, ASTError> {
        todo!()
    }

    fn is_err(&self) -> bool {
        self.is_poisoned
    }

    fn is_ok(&self) -> bool {
        !self.is_poisoned
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

impl<'a> AST<'a> for VarDef<'a> {
    fn sem(&self) -> Result<bool, ASTError> {
        todo!()
    }

    fn is_err(&self) -> bool {
        self.is_poisoned
    }

    fn is_ok(&self) -> bool {
        !self.is_poisoned
    }

    fn print_on(&self, f: &mut std::fmt::Formatter<'_>, mut depth: i32) -> std::fmt::Result {
        depth += 1;
        print_tabs(f, depth)?;
        let s = format!(
            "<VarDef id: {:?}, data_type: {:?}>",
            self.id, self.data_type
        );
        writeln!(f, "{}", s)?;
        let _ = &self.expr.print_on(f, depth)?;
        Ok(())
    }
}

impl<'a> AST<'a> for FuncCall<'a> {
    fn sem(&self) -> Result<bool, ASTError> {
        todo!()
    }

    fn is_err(&self) -> bool {
        self.is_poisoned
    }

    fn is_ok(&self) -> bool {
        !self.is_poisoned
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

impl<'a> AST<'a> for ObjectCall<'a> {
    fn sem(&self) -> Result<bool, ASTError> {
        todo!()
    }

    fn is_err(&self) -> bool {
        self.is_poisoned
    }

    fn is_ok(&self) -> bool {
        !self.is_poisoned
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

impl<'a> AST<'a> for RValue {
    fn sem(&self) -> Result<bool, ASTError> {
        todo!()
    }

    fn is_err(&self) -> bool {
        false
    }

    fn is_ok(&self) -> bool {
        true
    }

    fn print_on(&self, f: &mut std::fmt::Formatter<'_>, mut depth: i32) -> std::fmt::Result {
        depth += 1;
        print_tabs(f, depth)?;
        let s = format!("<RValue {:?}>", self);
        writeln!(f, "{}", s)?;
        Ok(())
    }
}
