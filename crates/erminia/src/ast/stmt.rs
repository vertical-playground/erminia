use crate::ast::ast::{ASTError, BoxAST, AST};
use crate::diagnostics::location::Span;
use crate::types::ErminiaType;

pub type BoxStmt = Box<dyn StmtTrait>;

// ==================================================================================== //
//  Traits                                                                              //
// ==================================================================================== //

pub trait StmtTrait: AST {
    fn run(&self) -> Result<u32, ASTError>;
}

// ==================================================================================== //
//  Enums                                                                               //
// ==================================================================================== //

#[derive(Debug)]
pub enum GenericTupleOption {
    Int(i32),
    Id(String),
    None,
}

#[derive(Debug)]
pub enum ShapeType {
    ShapeTuple,
    ShapeTupleIter,
}

// ==================================================================================== //
//  Structs                                                                             //
// ==================================================================================== //

#[derive(Debug)]
pub struct VarDef {
    pub id: String,
    pub data_type: ErminiaType,
    pub expr: BoxAST,
    pub span: Span,
}

#[derive(Debug)]
pub struct GenericTuple {
    pub left: BoxAST,
    pub right: BoxAST,
    pub span: Span,
}

#[derive(Debug)]
pub struct Tuple {
    pub left: i32,
    pub right: i32,
    pub span: Span,
}

#[derive(Debug)]
pub struct Range {
    pub left_inclusive: bool,
    pub right_inclusive: bool,
    pub left: i32,
    pub right: i32,
    pub span: Span,
}

#[derive(Debug)]
pub struct TupleIterator {
    pub id: String,
    pub range: BoxAST,
    pub span: Span,
}

#[derive(Debug)]
pub struct TupleComprehension {
    pub tuple: BoxAST,
    pub iter_pair: Vec<BoxAST>,
    pub span: Span,
}

#[derive(Debug)]
pub struct Shape {
    pub shape_type: ShapeType,
    pub values: BoxAST,
    pub span: Span,
}

#[derive(Debug)]
pub struct ObjectShape {
    pub shape: Vec<BoxAST>,
    pub span: Span,
}

#[derive(Debug)]
pub struct ObjectColor {
    pub color: i32,
    pub span: Span,
}

#[derive(Debug)]
pub struct ObjectDesc {
    pub shape: BoxAST,
    pub color: BoxAST,
    pub span: Span,
}

#[derive(Debug)]
pub struct ObjectDecl {
    pub id: String,
    pub desc: BoxAST,
    pub span: Span,
}

#[derive(Debug)]
pub struct ProblemExample {
    pub id: String,
    pub stmts: Vec<BoxAST>,
    pub span: Span,
}

#[derive(Debug)]
pub struct ProblemSolution {
    pub id: String,
    pub stmts: Vec<BoxAST>,
    pub span: Span,
}

#[derive(Debug)]
pub struct ProblemInput {
    pub id: String,
    pub stmts: Vec<BoxAST>,
    pub span: Span,
}

#[derive(Debug)]
pub struct ProblemOutput {
    pub id: String,
    pub stmts: Vec<BoxAST>,
    pub span: Span,
}

#[derive(Debug)]
pub struct Program {
    pub id: String,
    pub int_const: i32,
    pub stmts: Vec<BoxAST>,
    pub span: Span,
}

// ==================================================================================== //
//  Implementations                                                                     //
// ==================================================================================== //

impl GenericTupleOption {
    pub fn boxed_int(value: i32) -> BoxAST {
        Box::new(GenericTupleOption::Int(value))
    }

    pub fn boxed_id(value: String) -> BoxAST {
        Box::new(GenericTupleOption::Id(value))
    }

    pub fn boxed_none() -> BoxAST {
        Box::new(GenericTupleOption::None)
    }
}

impl Range {
    pub fn boxed(
        left_inclusive: bool,
        right_inclusive: bool,
        left: i32,
        right: i32,
        span: Span,
    ) -> BoxAST {
        Box::new(Range {
            left_inclusive,
            right_inclusive,
            left,
            right,
            span,
        })
    }
}

impl TupleIterator {
    pub fn boxed(id: String, range: BoxAST, span: Span) -> BoxAST {
        Box::new(TupleIterator { id, range, span })
    }
}

impl TupleComprehension {
    pub fn boxed(tuple: BoxAST, iter_pair: Vec<BoxAST>, span: Span) -> BoxAST {
        Box::new(TupleComprehension {
            tuple,
            iter_pair,
            span,
        })
    }
}

impl GenericTuple {
    pub fn boxed(left: BoxAST, right: BoxAST, span: Span) -> BoxAST {
        Box::new(GenericTuple { left, right, span })
    }
}

impl Tuple {
    pub fn boxed(left: i32, right: i32, span: Span) -> BoxAST {
        Box::new(Tuple { left, right, span })
    }
}

impl VarDef {
    pub fn boxed(id: String, data_type: ErminiaType, expr: BoxAST, span: Span) -> BoxAST {
        Box::new(VarDef {
            id,
            data_type,
            expr,
            span,
        })
    }
}

impl Shape {
    pub fn boxed_none(span: Span) -> BoxAST {
        Box::new(Shape {
            shape_type: ShapeType::ShapeTuple,
            values: GenericTupleOption::boxed_none(),
            span,
        })
    }
}

impl ObjectShape {
    pub fn boxed(shape: Vec<BoxAST>, span: Span) -> BoxAST {
        Box::new(ObjectShape { shape, span })
    }
}

impl ObjectColor {
    pub fn boxed(color: i32, span: Span) -> BoxAST {
        Box::new(ObjectColor { color, span })
    }
}

impl ObjectDesc {
    pub fn boxed(shape: BoxAST, color: BoxAST, span: Span) -> BoxAST {
        Box::new(ObjectDesc { shape, color, span })
    }
}

impl ObjectDecl {
    pub fn boxed(id: String, desc: BoxAST, span: Span) -> BoxAST {
        Box::new(ObjectDecl { id, desc, span })
    }
}

impl ProblemExample {
    pub fn boxed(id: String, stmts: Vec<BoxAST>, span: Span) -> BoxAST {
        Box::new(ProblemExample { id, stmts, span })
    }
}

impl ProblemSolution {
    pub fn boxed(id: String, stmts: Vec<BoxAST>, span: Span) -> BoxAST {
        Box::new(ProblemSolution { id, stmts, span })
    }
}

impl ProblemInput {
    pub fn boxed(id: String, stmts: Vec<BoxAST>, span: Span) -> BoxAST {
        Box::new(ProblemInput { id, stmts, span })
    }
}

impl ProblemOutput {
    pub fn boxed(id: String, stmts: Vec<BoxAST>, span: Span) -> BoxAST {
        Box::new(ProblemOutput { id, stmts, span })
    }
}

impl Program {
    pub fn boxed(id: String, int_const: i32, stmts: Vec<BoxAST>, span: Span) -> BoxAST {
        Box::new(Program {
            id,
            int_const,
            stmts,
            span,
        })
    }
}
