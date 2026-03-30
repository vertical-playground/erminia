use crate::ast::expr::*;
use crate::ast::stmt::*;

pub struct PrintOpt<'a, 'b> {
    f: &'a mut std::fmt::Formatter<'b>,
    depth: i32,
}

impl<'a, 'b> PrintOpt<'a, 'b> {
    pub fn new(f: &'a mut std::fmt::Formatter<'b>, depth: i32) -> PrintOpt<'a, 'b> {
        PrintOpt { f, depth }
    }
}

pub trait PrettyPrinting {
    fn print_on(&self, opts: &mut PrintOpt<'_, '_>) -> std::fmt::Result;
    fn to_json(&self, opts: &mut PrintOpt<'_, '_>) -> std::fmt::Result;
}

fn print_tabs(opts: &mut PrintOpt) -> std::fmt::Result {
    for _ in 0..opts.depth {
        write!(opts.f, "\t")?;
    }
    Ok(())
}

impl PrettyPrinting for GenericTupleOption {
    fn to_json(&self, _: &mut PrintOpt) -> std::fmt::Result {
        todo!("to_json not implemented yet");
    }

    fn print_on(&self, opts: &mut PrintOpt) -> std::fmt::Result {
        opts.depth += 1;
        print_tabs(opts)?;
        let s = format!("<GenericTupleOption {:?}>", self);
        writeln!(opts.f, "{}", s)?;
        Ok(())
    }
}

impl PrettyPrinting for ProblemExample<'_> {
    fn to_json(&self, _: &mut PrintOpt) -> std::fmt::Result {
        todo!("to_json not implemented yet");
    }

    fn print_on(&self, opts: &mut PrintOpt) -> std::fmt::Result {
        opts.depth += 1;
        print_tabs(opts)?;
        let s = format!(
            "<#{} ProblemExample id: {:?}, num: {:?}>",
            self.unique_ast_id, self.id, self.int_const
        );
        writeln!(opts.f, "{}", s)?;
        for stmt in &self.stmts {
            stmt.print_on(opts)?;
        }
        Ok(())
    }
}

impl PrettyPrinting for ProblemSolution<'_> {
    fn to_json(&self, _: &mut PrintOpt) -> std::fmt::Result {
        todo!("to_json not implemented yet");
    }

    fn print_on(&self, opts: &mut PrintOpt) -> std::fmt::Result {
        opts.depth += 1;
        print_tabs(opts)?;
        let s = format!(
            "<#{} ProblemSolution id: {:?}, num: {:?}>",
            self.unique_ast_id, self.id, self.int_const
        );
        writeln!(opts.f, "{}", s)?;
        for stmt in &self.stmts {
            stmt.print_on(opts)?;
        }
        Ok(())
    }
}

impl PrettyPrinting for ProblemInput<'_> {
    fn to_json(&self, _: &mut PrintOpt) -> std::fmt::Result {
        todo!("to_json not implemented yet");
    }

    fn print_on(&self, opts: &mut PrintOpt) -> std::fmt::Result {
        opts.depth += 1;
        print_tabs(opts)?;
        let s = format!("<#{} ProblemInput id: {:?}>", self.unique_ast_id, self.id);
        writeln!(opts.f, "{}", s)?;
        for stmt in &self.stmts {
            stmt.print_on(opts)?;
        }
        Ok(())
    }
}

impl PrettyPrinting for ProblemOutput<'_> {
    fn to_json(&self, _: &mut PrintOpt) -> std::fmt::Result {
        todo!("to_json not implemented yet");
    }

    fn print_on(&self, opts: &mut PrintOpt) -> std::fmt::Result {
        opts.depth += 1;
        print_tabs(opts)?;
        let s = format!("<#{} ProblemOutput id: {:?}>", self.unique_ast_id, self.id);
        writeln!(opts.f, "{}", s)?;
        for stmt in &self.stmts {
            stmt.print_on(opts)?;
        }
        Ok(())
    }
}

impl PrettyPrinting for Program<'_> {
    fn to_json(&self, _: &mut PrintOpt) -> std::fmt::Result {
        todo!("to_json not implemented yet");
    }

    fn print_on(&self, opts: &mut PrintOpt) -> std::fmt::Result {
        let s = format!(
            "<#{} Program id: {:?}, int_const: {}>",
            self.unique_ast_id, self.id, self.int_const
        );
        writeln!(opts.f, "{}", s)?;
        for stmt in &self.stmts {
            stmt.print_on(opts)?;
        }
        Ok(())
    }
}

impl PrettyPrinting for Range {
    fn to_json(&self, _: &mut PrintOpt) -> std::fmt::Result {
        todo!("to_json not implemented yet");
    }

    fn print_on(&self, opts: &mut PrintOpt) -> std::fmt::Result {
        opts.depth += 1;
        print_tabs(opts)?;
        let s = format!(
            "<#{} Range left_inclusive: {}, right_inclusive: {}, left: {}, right: {}>",
            self.unique_ast_id, self.left_inclusive, self.right_inclusive, self.left, self.right
        );
        writeln!(opts.f, "{}", s)?;
        Ok(())
    }
}

impl PrettyPrinting for TupleIterator<'_> {
    fn to_json(&self, _: &mut PrintOpt) -> std::fmt::Result {
        todo!("to_json not implemented yet");
    }

    fn print_on(&self, opts: &mut PrintOpt) -> std::fmt::Result {
        opts.depth += 1;
        print_tabs(opts)?;
        let s = format!("<#{} TupleIterator id: {:?}>", self.unique_ast_id, self.id);
        writeln!(opts.f, "{}", s)?;
        let _ = &self.range.print_on(opts)?;
        Ok(())
    }
}

impl PrettyPrinting for TupleComprehension<'_> {
    fn to_json(&self, _: &mut PrintOpt) -> std::fmt::Result {
        todo!("to_json not implemented yet");
    }

    fn print_on(&self, opts: &mut PrintOpt) -> std::fmt::Result {
        opts.depth += 1;
        print_tabs(opts)?;
        let s = format!("<#{} TupleComprehension>", self.unique_ast_id);
        writeln!(opts.f, "{}", s)?;
        let _ = &self.tuple.print_on(opts)?;
        for iter in &self.iter_pair {
            iter.print_on(opts)?;
        }
        Ok(())
    }
}

impl PrettyPrinting for GenericTuple<'_> {
    fn to_json(&self, _: &mut PrintOpt) -> std::fmt::Result {
        todo!("to_json not implemented yet");
    }

    fn print_on(&self, opts: &mut PrintOpt) -> std::fmt::Result {
        opts.depth += 1;
        print_tabs(opts)?;
        let s = format!("<#{} GenericTuple>", self.unique_ast_id);
        writeln!(opts.f, "{}", s)?;
        let _ = &self.left.print_on(opts)?;
        let _ = &self.right.print_on(opts)?;
        Ok(())
    }
}

impl PrettyPrinting for Tuple {
    fn to_json(&self, _: &mut PrintOpt) -> std::fmt::Result {
        todo!("to_json not implemented yet");
    }

    fn print_on(&self, opts: &mut PrintOpt) -> std::fmt::Result {
        opts.depth += 1;
        print_tabs(opts)?;
        let s = format!(
            "<#{} Tuple left: {}, right: {}>",
            self.unique_ast_id, self.left, self.right
        );
        writeln!(opts.f, "{}", s)?;
        Ok(())
    }
}

impl PrettyPrinting for Shape<'_> {
    fn to_json(&self, _: &mut PrintOpt) -> std::fmt::Result {
        todo!("to_json not implemented yet");
    }

    fn print_on(&self, opts: &mut PrintOpt) -> std::fmt::Result {
        opts.depth += 1;
        print_tabs(opts)?;
        let s = format!(
            "<#{} Shape type: {:?}>",
            self.unique_ast_id, self.shape_type
        );
        writeln!(opts.f, "{}", s)?;
        let _ = &self.values.print_on(opts)?;
        Ok(())
    }
}

impl PrettyPrinting for ObjectShape<'_> {
    fn to_json(&self, _: &mut PrintOpt) -> std::fmt::Result {
        todo!("to_json not implemented yet");
    }

    fn print_on(&self, opts: &mut PrintOpt) -> std::fmt::Result {
        opts.depth += 1;
        print_tabs(opts)?;
        let s = format!("<#{} ObjectShape>", self.unique_ast_id);
        writeln!(opts.f, "{}", s)?;
        for shape in &self.shape {
            shape.print_on(opts)?;
        }
        Ok(())
    }
}

impl PrettyPrinting for ObjectColor {
    fn to_json(&self, _: &mut PrintOpt) -> std::fmt::Result {
        todo!("to_json not implemented yet");
    }

    fn print_on(&self, opts: &mut PrintOpt) -> std::fmt::Result {
        opts.depth += 1;
        print_tabs(opts)?;
        let s = format!(
            "<#{} ObjectColor color: {}>",
            self.unique_ast_id, self.color
        );
        writeln!(opts.f, "{}", s)?;
        Ok(())
    }
}

impl PrettyPrinting for ObjectDesc<'_> {
    fn to_json(&self, _: &mut PrintOpt) -> std::fmt::Result {
        todo!("to_json not implemented yet");
    }

    fn print_on(&self, opts: &mut PrintOpt) -> std::fmt::Result {
        print_tabs(opts)?;
        let s = format!("<#{} ObjectDesc>", self.unique_ast_id);
        writeln!(opts.f, "{}", s)?;
        let _ = &self.shape.print_on(opts)?;
        let _ = &self.color.print_on(opts)?;
        Ok(())
    }
}

impl PrettyPrinting for ObjectDecl<'_> {
    fn to_json(&self, _: &mut PrintOpt) -> std::fmt::Result {
        todo!("to_json not implemented yet");
    }

    fn print_on(&self, opts: &mut PrintOpt) -> std::fmt::Result {
        opts.depth += 1;
        print_tabs(opts)?;
        let s = format!("<#{} ObjectDecl id: {:?}>", self.unique_ast_id, self.id);
        writeln!(opts.f, "{}", s)?;
        let _ = &self.desc.print_on(opts)?;
        Ok(())
    }
}

impl PrettyPrinting for VarDef<'_> {
    fn to_json(&self, _: &mut PrintOpt) -> std::fmt::Result {
        todo!("to_json not implemented yet");
    }

    fn print_on(&self, opts: &mut PrintOpt) -> std::fmt::Result {
        opts.depth += 1;
        print_tabs(opts)?;
        let s = format!(
            "<#{} VarDef id: {:?}, data_type: {:?}>",
            self.unique_ast_id, self.id, self.data_type
        );
        writeln!(opts.f, "{}", s)?;
        let _ = &self.expr.print_on(opts)?;
        Ok(())
    }
}

impl PrettyPrinting for FuncCall<'_> {
    fn to_json(&self, _: &mut PrintOpt) -> std::fmt::Result {
        todo!("to_json not implemented yet");
    }

    fn print_on(&self, opts: &mut PrintOpt) -> std::fmt::Result {
        opts.depth += 1;
        print_tabs(opts)?;
        let s = format!("<#{} FuncCall id: {:?}>", self.unique_ast_id, self.id);
        writeln!(opts.f, "{}", s)?;
        for expr in &self.exprs {
            expr.print_on(opts)?;
        }
        Ok(())
    }
}

impl PrettyPrinting for ObjectCall<'_> {
    fn to_json(&self, _: &mut PrintOpt) -> std::fmt::Result {
        todo!("to_json not implemented yet");
    }

    fn print_on(&self, opts: &mut PrintOpt) -> std::fmt::Result {
        opts.depth += 1;
        print_tabs(opts)?;
        let s = format!("<#{} ObjectCall id: {:?}>", self.unique_ast_id, self.id);
        writeln!(opts.f, "{}", s)?;
        if let Some(t) = &self.tuple {
            t.print_on(opts)?;
        };
        Ok(())
    }
}

impl PrettyPrinting for RValue {
    fn to_json(&self, _: &mut PrintOpt) -> std::fmt::Result {
        todo!("to_json not implemented yet");
    }

    fn print_on(&self, opts: &mut PrintOpt) -> std::fmt::Result {
        opts.depth += 1;
        print_tabs(opts)?;
        let s = format!("<RValue {:?}>", self);
        writeln!(opts.f, "{}", s)?;
        Ok(())
    }
}

impl PrettyPrinting for PoisonedStmt {
    fn to_json(&self, _: &mut PrintOpt) -> std::fmt::Result {
        todo!("to_json not implemented yet");
    }

    fn print_on(&self, opts: &mut PrintOpt) -> std::fmt::Result {
        opts.depth += 1;
        print_tabs(opts)?;
        let s = format!("<#{} PoisonedStmt>", self.unique_ast_id);
        writeln!(opts.f, "{}", s)?;
        Ok(())
    }
}
