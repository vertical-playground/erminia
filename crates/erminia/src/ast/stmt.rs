use crate::ast::ast::{BoxAST, ASTTrait, ASTError};
use crate::types::types::ErminiaType;

pub type BoxStmt = Box<dyn StmtTrait>;

// ==================================================================================== //
//  Traits                                                                              //
// ==================================================================================== //


pub trait StmtTrait: ASTTrait {
    fn run(&self) -> Result<u32, ASTError>;
}

// ==================================================================================== //
//  Enums                                                                               //
// ==================================================================================== //

#[derive(Debug)]
pub enum GenericTupleOption {
    Int(i32),
    Id(String),
    None
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
    id: String,
    data_type: ErminiaType,
    expr: BoxAST,
}

#[derive(Debug)]
pub struct GenericTuple {
    pub left: BoxAST,
    pub right: BoxAST,
}

#[derive(Debug)]
pub struct Tuple {
    left: i32,
    right: i32,
}

#[derive(Debug)]
pub struct Range {
    left_inclusive: bool,
    right_inclusive: bool,
    left: i32,
    right: i32,
}

#[derive(Debug)]
pub struct TupleIterator {
    id: String,
    pub range: BoxAST
}

#[derive(Debug)]
pub struct TupleComprehension { 
    pub tuple: BoxAST,
    pub iter_pair: Vec<BoxAST>
}

#[derive(Debug)]
pub struct Shape {
    shape_type: ShapeType,
    pub values: BoxAST,
}

#[derive(Debug)]
pub struct ObjectShape {
    pub shape: Vec<BoxAST>
}

#[derive(Debug)]
pub struct ObjectColor {
    color: String,
}

#[derive(Debug)]
pub struct ObjectDesc {
    pub shape: BoxAST,
    pub color: BoxAST,
}

#[derive(Debug)]
pub struct ObjectDecl {
    id: String,
    pub desc: BoxAST
}

#[derive(Debug)]
pub struct ProblemExample {
    id: String,
    pub stmts: Vec<BoxAST>,
}

#[derive(Debug)]
pub struct Program {
    id: String,
    int_const: i32,
    pub stmts: Vec<BoxAST>
}

// ==================================================================================== //
//  Implementations                                                                     //
// ==================================================================================== //

impl GenericTupleOption {
    pub fn new_int(value: i32) -> BoxAST {
        Box::new(GenericTupleOption::Int(value))
    }

    pub fn new_id(value: String) -> BoxAST {
        Box::new(GenericTupleOption::Id(value))
    }

    pub fn new_none() -> BoxAST {
        Box::new(GenericTupleOption::None)
    }
}

impl Range {
    pub fn new(left_inclusive: bool, right_inclusive: bool, left: i32, right: i32) -> BoxAST {
        Box::new(Range { left_inclusive, right_inclusive, left, right })
    }
}

impl TupleIterator {
    pub fn new(id: String, range: BoxAST) -> BoxAST {
        Box::new(TupleIterator { id, range })
    }
}

impl TupleComprehension {
    pub fn new(tuple: BoxAST, iter_pair: Vec<BoxAST>) -> BoxAST {
        Box::new(TupleComprehension { tuple, iter_pair })
    }
}


impl GenericTuple {
    pub fn new(left: BoxAST, right: BoxAST) -> BoxAST {
        Box::new(GenericTuple { left, right })
    }
}

impl Tuple {
    pub fn new(left: i32, right: i32) -> BoxAST {
        Box::new(Tuple { left, right })
    }
}

impl VarDef { 
    pub fn new(id: String, data_type: ErminiaType, expr: BoxAST) -> BoxAST {
        Box::new(VarDef { id, data_type, expr })
    }
}

impl Shape {
    pub fn new_none() -> BoxAST {
        Box::new(Shape { shape_type: ShapeType::ShapeTuple, values: GenericTupleOption::new_none() })
    }
}

impl ObjectShape {
    pub fn new(shape: Vec<BoxAST>) -> BoxAST {
        Box::new(ObjectShape { shape })
    }
}

impl ObjectColor {
    pub fn new(color: String) -> BoxAST {
        Box::new(ObjectColor { color })
    }
}

impl ObjectDesc {
    pub fn new(shape: BoxAST, color: BoxAST) -> BoxAST {
        Box::new(ObjectDesc { shape: shape, color: color })
    }
}

impl ObjectDecl {
    pub fn new(id: String, desc: BoxAST) -> BoxAST {
        Box::new(ObjectDecl { id, desc })
    }
}

impl ProblemExample {
    pub fn new(id: String, stmts: Vec<BoxAST>) -> BoxAST {
        Box::new(ProblemExample { id, stmts })
    }
}

impl Program {
    pub fn new(id: String, int_const: i32, stmts: Vec<BoxAST>) -> BoxAST {
        Box::new(Program { id, int_const, stmts })
    }
}
