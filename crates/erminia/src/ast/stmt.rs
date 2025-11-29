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
pub struct VarDef<'a> {
    pub id: String,
    pub data_type: ErminiaType,
    pub expr: BoxAST<'a>,
    pub span: Span,
    pub is_poisoned: bool,
    pub unique_ast_id: u32,
}

#[derive(Debug)]
pub struct GenericTuple<'a> {
    pub left: BoxAST<'a>,
    pub right: BoxAST<'a>,
    pub span: Span,
    pub is_poisoned: bool,
    pub unique_ast_id: u32,
}

#[derive(Debug)]
pub struct Tuple {
    pub left: i32,
    pub right: i32,
    pub span: Span,
    pub is_poisoned: bool,
    pub unique_ast_id: u32,
}

#[derive(Debug)]
pub struct Range {
    pub left_inclusive: bool,
    pub right_inclusive: bool,
    pub left: i32,
    pub right: i32,
    pub span: Span,
    pub is_poisoned: bool,
    pub unique_ast_id: u32,
}

#[derive(Debug)]
pub struct TupleIterator<'a> {
    pub id: String,
    pub range: BoxAST<'a>,
    pub span: Span,
    pub is_poisoned: bool,
    pub unique_ast_id: u32,
}

#[derive(Debug)]
pub struct TupleComprehension<'a> {
    pub tuple: BoxAST<'a>,
    pub iter_pair: Vec<BoxAST<'a>>,
    pub span: Span,
    pub is_poisoned: bool,
    pub unique_ast_id: u32,
}

#[derive(Debug)]
pub struct Shape<'a> {
    pub shape_type: ShapeType,
    pub values: BoxAST<'a>,
    pub span: Span,
    pub is_poisoned: bool,
    pub unique_ast_id: u32,
}

#[derive(Debug)]
pub struct ObjectShape<'a> {
    pub shape: Vec<BoxAST<'a>>,
    pub span: Span,
    pub is_poisoned: bool,
    pub unique_ast_id: u32,
}

#[derive(Debug)]
pub struct ObjectColor {
    pub color: i32,
    pub span: Span,
    pub is_poisoned: bool,
    pub unique_ast_id: u32,
}

#[derive(Debug)]
pub struct ObjectDesc<'a> {
    pub shape: BoxAST<'a>,
    pub color: BoxAST<'a>,
    pub span: Span,
    pub is_poisoned: bool,
    pub unique_ast_id: u32,
}

#[derive(Debug)]
pub struct ObjectDecl<'a> {
    pub id: String,
    pub desc: BoxAST<'a>,
    pub span: Span,
    pub is_poisoned: bool,
    pub unique_ast_id: u32,
}

#[derive(Debug)]
pub struct ProblemExample<'a> {
    pub id: String,
    pub stmts: Vec<BoxAST<'a>>,
    pub span: Span,
    pub is_poisoned: bool,
    pub unique_ast_id: u32,
}

#[derive(Debug)]
pub struct ProblemSolution<'a> {
    pub id: String,
    pub stmts: Vec<BoxAST<'a>>,
    pub span: Span,
    pub is_poisoned: bool,
    pub unique_ast_id: u32,
}

#[derive(Debug)]
pub struct ProblemInput<'a> {
    pub id: String,
    pub stmts: Vec<BoxAST<'a>>,
    pub span: Span,
    pub is_poisoned: bool,
    pub unique_ast_id: u32,
}

#[derive(Debug)]
pub struct ProblemOutput<'a> {
    pub id: String,
    pub stmts: Vec<BoxAST<'a>>,
    pub span: Span,
    pub is_poisoned: bool,
    pub unique_ast_id: u32,
}

#[derive(Debug)]
pub struct Program<'a> {
    pub id: String,
    pub int_const: i32,
    pub stmts: Vec<BoxAST<'a>>,
    pub span: Span,
    pub is_poisoned: bool,
    pub unique_ast_id: u32,
}

// ==================================================================================== //
//  Implementations                                                                     //
// ==================================================================================== //

impl<'a> GenericTupleOption {
    pub fn boxed_int(value: i32, poisoned: bool) -> BoxAST<'a> {
        if poisoned {
            return Box::new(GenericTupleOption::Poisoned);
        }

        Box::new(GenericTupleOption::Int(value))
    }

    pub fn boxed_id(name: String, poisoned: bool) -> BoxAST<'a> {
        if poisoned {
            return Box::new(GenericTupleOption::Poisoned);
        }

        Box::new(GenericTupleOption::Id(name))
    }

    pub fn boxed_none() -> BoxAST<'a> {
        Box::new(GenericTupleOption::None)
    }
}

impl<'a> Range {
    pub fn boxed(
        left_inclusive: bool,
        right_inclusive: bool,
        left: i32,
        right: i32,
        span: Span,
        is_poisoned: bool,
    ) -> BoxAST<'a> {
        let unique_ast_id = 0;
        Box::new(Range {
            left_inclusive,
            right_inclusive,
            left,
            right,
            span,
            is_poisoned,
            unique_ast_id,
        })
    }
}

impl<'a> TupleIterator<'a> {
    pub fn boxed(id: String, range: BoxAST<'a>, span: Span, is_poisoned: bool) -> BoxAST<'a> {
        let unique_ast_id = 0;
        Box::new(TupleIterator {
            id,
            range,
            span,
            is_poisoned,
            unique_ast_id,
        }) as BoxAST<'a>
    }
}

impl<'a> TupleComprehension<'a> {
    pub fn boxed(
        tuple: BoxAST<'a>,
        iter_pair: Vec<BoxAST<'a>>,
        span: Span,
        is_poisoned: bool,
    ) -> BoxAST<'a> {
        let unique_ast_id = 0;
        Box::new(TupleComprehension {
            tuple,
            iter_pair,
            span,
            is_poisoned,
            unique_ast_id,
        }) as BoxAST<'a>
    }
}

impl<'a> GenericTuple<'a> {
    pub fn boxed(left: BoxAST<'a>, right: BoxAST<'a>, span: Span, is_poisoned: bool) -> BoxAST<'a> {
        let unique_ast_id = 0;
        Box::new(GenericTuple {
            left,
            right,
            span,
            is_poisoned,
            unique_ast_id,
        }) as BoxAST<'a>
    }
}

impl<'a> Tuple {
    pub fn boxed(left: i32, right: i32, span: Span, is_poisoned: bool) -> BoxAST<'a> {
        let unique_ast_id = 0;
        Box::new(Tuple {
            left,
            right,
            span,
            is_poisoned,
            unique_ast_id,
        })
    }
}

impl<'a> VarDef<'a> {
    pub fn boxed(
        id: String,
        data_type: ErminiaType,
        expr: BoxAST<'a>,
        span: Span,
        is_poisoned: bool,
    ) -> BoxAST<'a> {
        let unique_ast_id = 0;
        Box::new(VarDef {
            id,
            data_type,
            expr,
            span,
            is_poisoned,
            unique_ast_id,
        }) as BoxAST<'a>
    }
}

impl<'a> Shape<'a> {
    pub fn boxed_none(span: Span, is_poisoned: bool) -> BoxAST<'a> {
        let unique_ast_id = 0;
        Box::new(Shape {
            shape_type: ShapeType::ShapeTuple,
            values: GenericTupleOption::boxed_none(),
            span,
            is_poisoned,
            unique_ast_id,
        }) as BoxAST<'a>
    }
}

impl<'a> ObjectShape<'a> {
    pub fn boxed(shape: Vec<BoxAST<'a>>, span: Span, is_poisoned: bool) -> BoxAST<'a> {
        let unique_ast_id = 0;
        Box::new(ObjectShape {
            shape,
            span,
            is_poisoned,
            unique_ast_id,
        }) as BoxAST<'a>
    }
}

impl<'a> ObjectColor {
    pub fn boxed(color: i32, span: Span, is_poisoned: bool) -> BoxAST<'a> {
        let unique_ast_id = 0;
        Box::new(ObjectColor {
            color,
            span,
            is_poisoned,
            unique_ast_id,
        })
    }
}

impl<'a> ObjectDesc<'a> {
    pub fn boxed(
        shape: BoxAST<'a>,
        color: BoxAST<'a>,
        span: Span,
        is_poisoned: bool,
    ) -> BoxAST<'a> {
        let unique_ast_id = 0;
        Box::new(ObjectDesc {
            shape,
            color,
            span,
            is_poisoned,
            unique_ast_id,
        }) as BoxAST<'a>
    }
}

impl<'a> ObjectDecl<'a> {
    pub fn boxed(id: String, desc: BoxAST<'a>, span: Span, is_poisoned: bool) -> BoxAST<'a> {
        let unique_ast_id = 0;
        Box::new(ObjectDecl {
            id,
            desc,
            span,
            is_poisoned,
            unique_ast_id,
        }) as BoxAST<'a>
    }
}

impl<'a> ProblemExample<'a> {
    pub fn boxed(id: String, stmts: Vec<BoxAST<'a>>, span: Span, is_poisoned: bool) -> BoxAST<'a> {
        let unique_ast_id = 0;
        Box::new(ProblemExample {
            id,
            stmts,
            span,
            is_poisoned,
            unique_ast_id,
        }) as BoxAST<'a>
    }
}

impl<'a> ProblemSolution<'a> {
    pub fn boxed(id: String, stmts: Vec<BoxAST<'a>>, span: Span, is_poisoned: bool) -> BoxAST<'a> {
        let unique_ast_id = 0;
        Box::new(ProblemSolution {
            id,
            stmts,
            span,
            is_poisoned,
            unique_ast_id,
        }) as BoxAST<'a>
    }
}

impl<'a> ProblemInput<'a> {
    pub fn boxed(id: String, stmts: Vec<BoxAST<'a>>, span: Span, is_poisoned: bool) -> BoxAST<'a> {
        let unique_ast_id = 0;
        Box::new(ProblemInput {
            id,
            stmts,
            span,
            is_poisoned,
            unique_ast_id,
        }) as BoxAST<'a>
    }
}

impl<'a> ProblemOutput<'a> {
    pub fn boxed(id: String, stmts: Vec<BoxAST<'a>>, span: Span, is_poisoned: bool) -> BoxAST<'a> {
        let unique_ast_id = 0;
        Box::new(ProblemOutput {
            id,
            stmts,
            span,
            is_poisoned,
            unique_ast_id,
        }) as BoxAST<'a>
    }
}

impl<'a> Program<'a> {
    pub fn boxed(
        id: String,
        int_const: i32,
        stmts: Vec<BoxAST<'a>>,
        span: Span,
        is_poisoned: bool,
    ) -> BoxAST<'a> {
        let unique_ast_id = 0;
        Box::new(Program {
            id,
            int_const,
            stmts,
            span,
            is_poisoned,
            unique_ast_id,
        }) as BoxAST<'a>
    }
}
