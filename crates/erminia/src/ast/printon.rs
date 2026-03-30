use crate::ast::expr::*;
use crate::ast::stmt::*;

pub trait PrettyPrinting {
    fn print_on(&self, f: &mut std::fmt::Formatter<'_>, depth: i32) -> std::fmt::Result;
}

fn print_tabs(f: &mut std::fmt::Formatter<'_>, depth: i32) -> std::fmt::Result {
    for _ in 0..depth {
        write!(f, "\t")?;
    }
    Ok(())
}

impl PrettyPrinting for GenericTupleOption {
    fn print_on(&self, f: &mut std::fmt::Formatter<'_>, mut depth: i32) -> std::fmt::Result {
        depth += 1;
        print_tabs(f, depth)?;
        let s = format!("<GenericTupleOption {:?}>", self);
        writeln!(f, "{}", s)?;
        Ok(())
    }
}

impl PrettyPrinting for ProblemExample<'_> {
    fn print_on(&self, f: &mut std::fmt::Formatter<'_>, mut depth: i32) -> std::fmt::Result {
        depth += 1;
        print_tabs(f, depth)?;
        let s = format!(
            "<#{} ProblemExample id: {:?}, num: {:?}>",
            self.unique_ast_id, self.id, self.int_const
        );
        writeln!(f, "{}", s)?;
        for stmt in &self.stmts {
            stmt.print_on(f, depth)?;
        }
        Ok(())
    }
}

impl PrettyPrinting for ProblemSolution<'_> {
    fn print_on(&self, f: &mut std::fmt::Formatter<'_>, mut depth: i32) -> std::fmt::Result {
        depth += 1;
        print_tabs(f, depth)?;
        let s = format!(
            "<#{} ProblemSolution id: {:?}, num: {:?}>",
            self.unique_ast_id, self.id, self.int_const
        );
        writeln!(f, "{}", s)?;
        for stmt in &self.stmts {
            stmt.print_on(f, depth)?;
        }
        Ok(())
    }
}

impl PrettyPrinting for ProblemInput<'_> {
    fn print_on(&self, f: &mut std::fmt::Formatter<'_>, mut depth: i32) -> std::fmt::Result {
        depth += 1;
        print_tabs(f, depth)?;
        let s = format!("<#{} ProblemInput id: {:?}>", self.unique_ast_id, self.id);
        writeln!(f, "{}", s)?;
        for stmt in &self.stmts {
            stmt.print_on(f, depth)?;
        }
        Ok(())
    }
}

impl PrettyPrinting for ProblemOutput<'_> {
    fn print_on(&self, f: &mut std::fmt::Formatter<'_>, mut depth: i32) -> std::fmt::Result {
        depth += 1;
        print_tabs(f, depth)?;
        let s = format!("<#{} ProblemOutput id: {:?}>", self.unique_ast_id, self.id);
        writeln!(f, "{}", s)?;
        for stmt in &self.stmts {
            stmt.print_on(f, depth)?;
        }
        Ok(())
    }
}

impl PrettyPrinting for Program<'_> {
    fn print_on(&self, f: &mut std::fmt::Formatter<'_>, depth: i32) -> std::fmt::Result {
        let s = format!(
            "<#{} Program id: {:?}, int_const: {}>",
            self.unique_ast_id, self.id, self.int_const
        );
        writeln!(f, "{}", s)?;
        for stmt in &self.stmts {
            stmt.print_on(f, depth)?;
        }
        Ok(())
    }
}

impl PrettyPrinting for Range {
    fn print_on(&self, f: &mut std::fmt::Formatter<'_>, mut depth: i32) -> std::fmt::Result {
        depth += 1;
        print_tabs(f, depth)?;
        let s = format!(
            "<#{} Range left_inclusive: {}, right_inclusive: {}, left: {}, right: {}>",
            self.unique_ast_id, self.left_inclusive, self.right_inclusive, self.left, self.right
        );
        writeln!(f, "{}", s)?;
        Ok(())
    }
}

impl PrettyPrinting for TupleIterator<'_> {
    fn print_on(&self, f: &mut std::fmt::Formatter<'_>, mut depth: i32) -> std::fmt::Result {
        depth += 1;
        print_tabs(f, depth)?;
        let s = format!("<#{} TupleIterator id: {:?}>", self.unique_ast_id, self.id);
        writeln!(f, "{}", s)?;
        let _ = &self.range.print_on(f, depth)?;
        Ok(())
    }
}

impl PrettyPrinting for TupleComprehension<'_> {
    fn print_on(&self, f: &mut std::fmt::Formatter<'_>, mut depth: i32) -> std::fmt::Result {
        depth += 1;
        print_tabs(f, depth)?;
        let s = format!("<#{} TupleComprehension>", self.unique_ast_id);
        writeln!(f, "{}", s)?;
        let _ = &self.tuple.print_on(f, depth)?;
        for iter in &self.iter_pair {
            iter.print_on(f, depth)?;
        }
        Ok(())
    }
}

impl PrettyPrinting for GenericTuple<'_> {
    fn print_on(&self, f: &mut std::fmt::Formatter<'_>, mut depth: i32) -> std::fmt::Result {
        depth += 1;
        print_tabs(f, depth)?;
        let s = format!("<#{} GenericTuple>", self.unique_ast_id);
        writeln!(f, "{}", s)?;
        let _ = &self.left.print_on(f, depth)?;
        let _ = &self.right.print_on(f, depth)?;
        Ok(())
    }
}

impl PrettyPrinting for Tuple {
    fn print_on(&self, f: &mut std::fmt::Formatter<'_>, mut depth: i32) -> std::fmt::Result {
        depth += 1;
        print_tabs(f, depth)?;
        let s = format!(
            "<#{} Tuple left: {}, right: {}>",
            self.unique_ast_id, self.left, self.right
        );
        writeln!(f, "{}", s)?;
        Ok(())
    }
}

impl PrettyPrinting for Shape<'_> {
    fn print_on(&self, f: &mut std::fmt::Formatter<'_>, mut depth: i32) -> std::fmt::Result {
        depth += 1;
        print_tabs(f, depth)?;
        let s = format!(
            "<#{} Shape type: {:?}>",
            self.unique_ast_id, self.shape_type
        );
        writeln!(f, "{}", s)?;
        let _ = &self.values.print_on(f, depth)?;
        Ok(())
    }
}

impl PrettyPrinting for ObjectShape<'_> {
    fn print_on(&self, f: &mut std::fmt::Formatter<'_>, mut depth: i32) -> std::fmt::Result {
        depth += 1;
        print_tabs(f, depth)?;
        let s = format!("<#{} ObjectShape>", self.unique_ast_id);
        writeln!(f, "{}", s)?;
        for shape in &self.shape {
            shape.print_on(f, depth)?;
        }
        Ok(())
    }
}

impl PrettyPrinting for ObjectColor {
    fn print_on(&self, f: &mut std::fmt::Formatter<'_>, mut depth: i32) -> std::fmt::Result {
        depth += 1;
        print_tabs(f, depth)?;
        let s = format!(
            "<#{} ObjectColor color: {}>",
            self.unique_ast_id, self.color
        );
        writeln!(f, "{}", s)?;
        Ok(())
    }
}

impl PrettyPrinting for ObjectDesc<'_> {
    fn print_on(&self, f: &mut std::fmt::Formatter<'_>, depth: i32) -> std::fmt::Result {
        print_tabs(f, depth)?;
        let s = format!("<#{} ObjectDesc>", self.unique_ast_id);
        writeln!(f, "{}", s)?;
        let _ = &self.shape.print_on(f, depth)?;
        let _ = &self.color.print_on(f, depth)?;
        Ok(())
    }
}

impl PrettyPrinting for ObjectDecl<'_> {
    fn print_on(&self, f: &mut std::fmt::Formatter<'_>, mut depth: i32) -> std::fmt::Result {
        depth += 1;
        print_tabs(f, depth)?;
        let s = format!("<#{} ObjectDecl id: {:?}>", self.unique_ast_id, self.id);
        writeln!(f, "{}", s)?;
        let _ = &self.desc.print_on(f, depth)?;
        Ok(())
    }
}

impl PrettyPrinting for VarDef<'_> {
    fn print_on(&self, f: &mut std::fmt::Formatter<'_>, mut depth: i32) -> std::fmt::Result {
        depth += 1;
        print_tabs(f, depth)?;
        let s = format!(
            "<#{} VarDef id: {:?}, data_type: {:?}>",
            self.unique_ast_id, self.id, self.data_type
        );
        writeln!(f, "{}", s)?;
        let _ = &self.expr.print_on(f, depth)?;
        Ok(())
    }
}

impl PrettyPrinting for FuncCall<'_> {
    fn print_on(&self, f: &mut std::fmt::Formatter<'_>, mut depth: i32) -> std::fmt::Result {
        depth += 1;
        print_tabs(f, depth)?;
        let s = format!("<#{} FuncCall id: {:?}>", self.unique_ast_id, self.id);
        writeln!(f, "{}", s)?;
        for expr in &self.exprs {
            expr.print_on(f, depth)?;
        }
        Ok(())
    }
}

impl PrettyPrinting for ObjectCall<'_> {
    fn print_on(&self, f: &mut std::fmt::Formatter<'_>, mut depth: i32) -> std::fmt::Result {
        depth += 1;
        print_tabs(f, depth)?;
        let s = format!("<#{} ObjectCall id: {:?}>", self.unique_ast_id, self.id);
        writeln!(f, "{}", s)?;
        if let Some(t) = &self.tuple {
            t.print_on(f, depth)?;
        };
        Ok(())
    }
}

impl PrettyPrinting for RValue {
    fn print_on(&self, f: &mut std::fmt::Formatter<'_>, mut depth: i32) -> std::fmt::Result {
        depth += 1;
        print_tabs(f, depth)?;
        let s = format!("<RValue {:?}>", self);
        writeln!(f, "{}", s)?;
        Ok(())
    }
}

impl PrettyPrinting for PoisonedStmt {
    fn print_on(&self, f: &mut std::fmt::Formatter<'_>, mut depth: i32) -> std::fmt::Result {
        depth += 1;
        print_tabs(f, depth)?;
        let s = format!("<#{} PoisonedStmt>", self.unique_ast_id);
        writeln!(f, "{}", s)?;
        Ok(())
    }
}
