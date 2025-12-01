use crate::ast::ast::{ASTError, BoxAST, AST};
use crate::diagnostics::location::Span;
use crate::types::ErminiaType;

pub type BoxStmt<'a> = Box<dyn StmtTrait<'a> + 'a>;

// ==================================================================================== //
//  Traits                                                                              //
// ==================================================================================== //

pub trait StmtTrait<'a>: AST<'a> {
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
    Poisoned,
}

#[derive(Debug)]
pub enum ShapeType {
    ShapeTuple,
    ShapeTupleIter,
    Poisoned,
}

// ==================================================================================== //
//  Structs                                                                             //
// ==================================================================================== //

#[derive(Debug)]
pub struct PoisonedStmt {
    pub span: Span,
    pub is_poisoned: bool,
    pub unique_ast_id: u32,
}

#[derive(Debug)]
pub struct VarDef<'a> {
    pub id: ErminiaType,
    pub data_type: ErminiaType,
    pub expr: BoxAST<'a>,
    pub span: Span,
    pub is_poisoned: bool,
    pub unique_ast_id: u32,
    pub syntax: Vec<ErminiaType>,
}

#[derive(Debug)]
pub struct GenericTuple<'a> {
    pub left: BoxAST<'a>,
    pub right: BoxAST<'a>,
    pub span: Span,
    pub is_poisoned: bool,
    pub unique_ast_id: u32,
    pub syntax: Vec<ErminiaType>,
}

#[derive(Debug)]
pub struct Tuple {
    pub left: ErminiaType,
    pub right: ErminiaType,
    pub span: Span,
    pub is_poisoned: bool,
    pub unique_ast_id: u32,
    pub syntax: Vec<ErminiaType>,
}

#[derive(Debug)]
pub struct Range {
    pub left_inclusive: ErminiaType,
    pub right_inclusive: ErminiaType,
    pub left: ErminiaType,
    pub right: ErminiaType,
    pub span: Span,
    pub is_poisoned: bool,
    pub unique_ast_id: u32,
    pub syntax: Vec<ErminiaType>,
}

#[derive(Debug)]
pub struct TupleIterator<'a> {
    pub id: ErminiaType,
    pub range: BoxAST<'a>,
    pub span: Span,
    pub is_poisoned: bool,
    pub unique_ast_id: u32,
    pub syntax: Vec<ErminiaType>,
}

#[derive(Debug)]
pub struct TupleComprehension<'a> {
    pub tuple: BoxAST<'a>,
    pub iter_pair: Vec<BoxAST<'a>>,
    pub span: Span,
    pub is_poisoned: bool,
    pub unique_ast_id: u32,
    pub syntax: Vec<ErminiaType>,
}

#[derive(Debug)]
pub struct Shape<'a> {
    pub shape_type: ShapeType,
    pub values: BoxAST<'a>,
    pub span: Span,
    pub is_poisoned: bool,
    pub unique_ast_id: u32,
    pub syntax: Vec<ErminiaType>,
}

#[derive(Debug)]
pub struct ObjectShape<'a> {
    pub shape: Vec<BoxAST<'a>>,
    pub span: Span,
    pub is_poisoned: bool,
    pub unique_ast_id: u32,
    pub syntax: Vec<ErminiaType>,
}

#[derive(Debug)]
pub struct ObjectColor {
    pub color: ErminiaType,
    pub span: Span,
    pub is_poisoned: bool,
    pub unique_ast_id: u32,
    pub syntax: Vec<ErminiaType>,
}

#[derive(Debug)]
pub struct ObjectDesc<'a> {
    pub shape: BoxAST<'a>,
    pub color: BoxAST<'a>,
    pub span: Span,
    pub is_poisoned: bool,
    pub unique_ast_id: u32,
    pub syntax: Vec<ErminiaType>,
}

#[derive(Debug)]
pub struct ObjectDecl<'a> {
    pub id: ErminiaType,
    pub desc: BoxAST<'a>,
    pub span: Span,
    pub is_poisoned: bool,
    pub unique_ast_id: u32,
    pub syntax: Vec<ErminiaType>,
}

#[derive(Debug)]
pub struct ProblemExample<'a> {
    pub id: ErminiaType,
    pub stmts: Vec<BoxAST<'a>>,
    pub span: Span,
    pub is_poisoned: bool,
    pub unique_ast_id: u32,
    pub syntax: Vec<ErminiaType>,
}

#[derive(Debug)]
pub struct ProblemSolution<'a> {
    pub id: ErminiaType,
    pub stmts: Vec<BoxAST<'a>>,
    pub span: Span,
    pub is_poisoned: bool,
    pub unique_ast_id: u32,
    pub syntax: Vec<ErminiaType>,
}

#[derive(Debug)]
pub struct ProblemInput<'a> {
    pub id: ErminiaType,
    pub stmts: Vec<BoxAST<'a>>,
    pub span: Span,
    pub is_poisoned: bool,
    pub unique_ast_id: u32,
    pub syntax: Vec<ErminiaType>,
}

#[derive(Debug)]
pub struct ProblemOutput<'a> {
    pub id: ErminiaType,
    pub stmts: Vec<BoxAST<'a>>,
    pub span: Span,
    pub is_poisoned: bool,
    pub unique_ast_id: u32,
    pub syntax: Vec<ErminiaType>,
}

#[derive(Debug)]
pub struct Program<'a> {
    pub id: ErminiaType,
    pub int_const: ErminiaType,
    pub stmts: Vec<BoxAST<'a>>,
    pub span: Span,
    pub is_poisoned: bool,
    pub unique_ast_id: u32,
    pub syntax: Vec<ErminiaType>,
}

// ==================================================================================== //
//  Implementations                                                                     //
// ==================================================================================== //

impl<'a> GenericTupleOption {
    pub fn boxed_int(value: ErminiaType) -> BoxAST<'a> {
        let val = match value {
            ErminiaType::Integer(i) => i,
            _ => return Box::new(GenericTupleOption::Poisoned),
        };

        Box::new(GenericTupleOption::Int(val))
    }

    pub fn boxed_id(name: ErminiaType) -> BoxAST<'a> {
        let val = match name {
            ErminiaType::Ident(s) => s,
            _ => return Box::new(GenericTupleOption::Poisoned),
        };

        Box::new(GenericTupleOption::Id(val))
    }

    pub fn boxed_none() -> BoxAST<'a> {
        Box::new(GenericTupleOption::None)
    }
}

impl<'a> Range {
    pub fn boxed(
        left_inclusive: ErminiaType,
        right_inclusive: ErminiaType,
        left: ErminiaType,
        right: ErminiaType,
        span: Span,
        syntax: Vec<ErminiaType>,
    ) -> BoxAST<'a> {
        let unique_ast_id = 0;
        let mut is_poisoned = false;

        if syntax.iter().any(|s| s.is_poisoned()) {
            is_poisoned = true;
        }

        if left_inclusive.is_poisoned()
            || right_inclusive.is_poisoned()
            || left.is_poisoned()
            || right.is_poisoned()
        {
            is_poisoned = true;
        }

        Box::new(Range {
            left_inclusive,
            right_inclusive,
            left,
            right,
            span,
            is_poisoned,
            unique_ast_id,
            syntax,
        })
    }
}

impl<'a> TupleIterator<'a> {
    pub fn boxed(
        id: ErminiaType,
        range: BoxAST<'a>,
        span: Span,
        syntax: Vec<ErminiaType>,
    ) -> BoxAST<'a> {
        let unique_ast_id = 0;
        let mut is_poisoned = false;

        if syntax.iter().any(|s| s.is_poisoned()) {
            is_poisoned = true;
        }

        if id.is_poisoned() || range.is_err() {
            is_poisoned = true;
        }

        Box::new(TupleIterator {
            id,
            range,
            span,
            is_poisoned,
            unique_ast_id,
            syntax,
        }) as BoxAST<'a>
    }
}

impl<'a> TupleComprehension<'a> {
    pub fn boxed(
        tuple: BoxAST<'a>,
        iter_pair: Vec<BoxAST<'a>>,
        span: Span,
        syntax: Vec<ErminiaType>,
    ) -> BoxAST<'a> {
        let unique_ast_id = 0;
        let mut is_poisoned = false;

        if iter_pair.iter().any(|s| s.is_err()) {
            is_poisoned = true;
        }

        if tuple.is_err() {
            is_poisoned = true;
        }

        Box::new(TupleComprehension {
            tuple,
            iter_pair,
            span,
            is_poisoned,
            unique_ast_id,
            syntax,
        }) as BoxAST<'a>
    }
}

impl<'a> GenericTuple<'a> {
    pub fn boxed(
        left: BoxAST<'a>,
        right: BoxAST<'a>,
        span: Span,
        syntax: Vec<ErminiaType>,
    ) -> BoxAST<'a> {
        let unique_ast_id = 0;
        let mut is_poisoned = false;

        if syntax.iter().any(|s| s.is_poisoned()) {
            is_poisoned = true;
        }

        if left.is_err() || right.is_err() {
            is_poisoned = true;
        }

        Box::new(GenericTuple {
            left,
            right,
            span,
            is_poisoned,
            unique_ast_id,
            syntax,
        }) as BoxAST<'a>
    }
}

impl<'a> Tuple {
    pub fn boxed(
        left: ErminiaType,
        right: ErminiaType,
        span: Span,
        syntax: Vec<ErminiaType>,
    ) -> BoxAST<'a> {
        let unique_ast_id = 0;
        let mut is_poisoned = false;

        if syntax.iter().any(|s| s.is_poisoned()) {
            is_poisoned = true;
        }

        if left.is_poisoned() || right.is_poisoned() {
            is_poisoned = true;
        }

        Box::new(Tuple {
            left,
            right,
            span,
            is_poisoned,
            unique_ast_id,
            syntax,
        }) as BoxAST<'a>
    }
}

impl<'a> VarDef<'a> {
    pub fn boxed(
        id: ErminiaType,
        data_type: ErminiaType,
        expr: BoxAST<'a>,
        span: Span,
        syntax: Vec<ErminiaType>,
    ) -> BoxAST<'a> {
        let unique_ast_id = 0;
        let mut is_poisoned = false;

        if syntax.iter().any(|s| s.is_poisoned()) {
            is_poisoned = true;
        }

        if id.is_poisoned() || data_type.is_poisoned() || expr.is_err() {
            is_poisoned = true;
        }

        Box::new(VarDef {
            id,
            data_type,
            expr,
            span,
            is_poisoned,
            unique_ast_id,
            syntax,
        }) as BoxAST<'a>
    }
}

impl<'a> Shape<'a> {
    pub fn boxed_none(span: Span, syntax: Vec<ErminiaType>) -> BoxAST<'a> {
        let unique_ast_id = 0;

        Box::new(Shape {
            shape_type: ShapeType::ShapeTuple,
            values: GenericTupleOption::boxed_none(),
            span,
            is_poisoned: false,
            unique_ast_id,
            syntax,
        }) as BoxAST<'a>
    }
}

impl<'a> ObjectShape<'a> {
    pub fn boxed(shape: Vec<BoxAST<'a>>, span: Span, syntax: Vec<ErminiaType>) -> BoxAST<'a> {
        let unique_ast_id = 0;
        let mut is_poisoned = false;

        if syntax.iter().any(|s| s.is_poisoned()) {
            is_poisoned = true;
        }

        if shape.iter().any(|s| s.is_err()) {
            is_poisoned = true;
        }

        Box::new(ObjectShape {
            shape,
            span,
            is_poisoned,
            unique_ast_id,
            syntax,
        }) as BoxAST<'a>
    }
}

impl<'a> ObjectColor {
    pub fn boxed(color: ErminiaType, span: Span, syntax: Vec<ErminiaType>) -> BoxAST<'a> {
        let unique_ast_id = 0;
        let mut is_poisoned = false;

        if syntax.iter().any(|s| s.is_poisoned()) {
            is_poisoned = true;
        }

        if color.is_poisoned() {
            is_poisoned = true;
        }

        Box::new(ObjectColor {
            color,
            span,
            is_poisoned,
            unique_ast_id,
            syntax,
        })
    }
}

impl<'a> ObjectDesc<'a> {
    pub fn boxed(
        shape: BoxAST<'a>,
        color: BoxAST<'a>,
        span: Span,
        syntax: Vec<ErminiaType>,
    ) -> BoxAST<'a> {
        let unique_ast_id = 0;
        let mut is_poisoned = false;

        if syntax.iter().any(|s| s.is_poisoned()) {
            is_poisoned = true;
        }

        if shape.is_err() || color.is_err() {
            is_poisoned = true;
        }

        Box::new(ObjectDesc {
            shape,
            color,
            span,
            is_poisoned,
            unique_ast_id,
            syntax,
        }) as BoxAST<'a>
    }
}

impl<'a> ObjectDecl<'a> {
    pub fn boxed(
        id: ErminiaType,
        desc: BoxAST<'a>,
        span: Span,
        syntax: Vec<ErminiaType>,
    ) -> BoxAST<'a> {
        let unique_ast_id = 0;
        let mut is_poisoned = false;

        if syntax.iter().any(|s| s.is_poisoned()) {
            is_poisoned = true;
        }

        if id.is_poisoned() || desc.is_err() {
            is_poisoned = true;
        }

        Box::new(ObjectDecl {
            id,
            desc,
            span,
            is_poisoned,
            unique_ast_id,
            syntax,
        }) as BoxAST<'a>
    }
}

impl<'a> ProblemExample<'a> {
    pub fn boxed(
        id: ErminiaType,
        stmts: Vec<BoxAST<'a>>,
        span: Span,
        syntax: Vec<ErminiaType>,
    ) -> BoxAST<'a> {
        let unique_ast_id = 0;
        let mut is_poisoned = false;

        if syntax.iter().any(|s| s.is_poisoned()) {
            is_poisoned = true;
        }

        if stmts.iter().any(|s| s.is_err()) || id.is_poisoned() {
            is_poisoned = true;
        }

        Box::new(ProblemExample {
            id,
            stmts,
            span,
            is_poisoned,
            unique_ast_id,
            syntax,
        }) as BoxAST<'a>
    }
}

impl<'a> ProblemSolution<'a> {
    pub fn boxed(
        id: ErminiaType,
        stmts: Vec<BoxAST<'a>>,
        span: Span,
        syntax: Vec<ErminiaType>,
    ) -> BoxAST<'a> {
        let unique_ast_id = 0;
        let mut is_poisoned = false;

        if syntax.iter().any(|s| s.is_poisoned()) {
            is_poisoned = true;
        }

        if stmts.iter().any(|s| s.is_err()) || id.is_poisoned() {
            is_poisoned = true;
        }

        Box::new(ProblemSolution {
            id,
            stmts,
            span,
            is_poisoned,
            unique_ast_id,
            syntax,
        }) as BoxAST<'a>
    }
}

impl<'a> ProblemInput<'a> {
    pub fn boxed(
        id: ErminiaType,
        stmts: Vec<BoxAST<'a>>,
        span: Span,
        syntax: Vec<ErminiaType>,
    ) -> BoxAST<'a> {
        let unique_ast_id = 0;
        let mut is_poisoned = false;

        if syntax.iter().any(|s| s.is_poisoned()) {
            is_poisoned = true;
        }

        if stmts.iter().any(|s| s.is_err()) || id.is_poisoned() {
            is_poisoned = true;
        }

        Box::new(ProblemInput {
            id,
            stmts,
            span,
            is_poisoned,
            unique_ast_id,
            syntax,
        }) as BoxAST<'a>
    }
}

impl<'a> ProblemOutput<'a> {
    pub fn boxed(
        id: ErminiaType,
        stmts: Vec<BoxAST<'a>>,
        span: Span,
        syntax: Vec<ErminiaType>,
    ) -> BoxAST<'a> {
        let unique_ast_id = 0;
        let mut is_poisoned = false;

        if syntax.iter().any(|s| s.is_poisoned()) {
            is_poisoned = true;
        }

        if stmts.iter().any(|s| s.is_err()) || id.is_poisoned() {
            is_poisoned = true;
        }

        Box::new(ProblemOutput {
            id,
            stmts,
            span,
            is_poisoned,
            unique_ast_id,
            syntax,
        }) as BoxAST<'a>
    }
}

impl<'a> Program<'a> {
    pub fn boxed(
        id: ErminiaType,
        int_const: ErminiaType,
        stmts: Vec<BoxAST<'a>>,
        span: Span,
        syntax: Vec<ErminiaType>,
    ) -> BoxAST<'a> {
        let unique_ast_id = 0;
        let mut is_poisoned = false;

        if syntax.iter().any(|s| s.is_poisoned()) {
            is_poisoned = true;
        }

        if stmts.iter().any(|s| s.is_err()) || id.is_poisoned() || int_const.is_poisoned() {
            is_poisoned = true;
        }

        Box::new(Program {
            id,
            int_const,
            stmts,
            span,
            is_poisoned,
            unique_ast_id,
            syntax,
        }) as BoxAST<'a>
    }
}

impl<'a> PoisonedStmt {
    pub fn boxed(span: Span) -> BoxAST<'a> {
        let unique_ast_id = 0;

        Box::new(PoisonedStmt {
            span,
            is_poisoned: true,
            unique_ast_id,
        }) as BoxAST<'a>
    }
}
