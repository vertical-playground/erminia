pub type ASTError = String;

pub enum GenericTupleOption {
    Int(i32),
    Id(&'static str),
    None
}

pub struct GenericTuple {
    left: GenericTupleOption,
    right: GenericTupleOption,
}

impl GenericTuple {
    pub fn new(left: GenericTupleOption, right: GenericTupleOption) -> Self {
        GenericTuple { left, right }
    }
}

pub struct Tuple {
    left: i32,
    right: i32,
}

impl Tuple {
    pub fn new(left: i32, right: i32) -> Self {
        Tuple { left, right }
    }
}

pub struct ObjectShape {
    shape: Vec<Tuple>
}

pub struct ObjectColor {
    color: &'static str,
}

pub struct ObjectDesc {
    shape: ObjectShape,
    color: ObjectColor,
}

impl ObjectDesc {
    pub fn new(shape: ObjectShape, color: ObjectColor) -> Self {
        ObjectDesc { shape: shape, color: color }
    }
}

pub struct ObjectDecl {
    id: &'static str,
    desc: ObjectDesc
}

impl ObjectDecl {
    pub fn new(id: &'static str, desc: ObjectDesc) -> Self {
        ObjectDecl { id, desc }
    }
}

pub struct ObjectCall {
    id: &'static str,
    tuple: Option<Tuple> 
}

impl ObjectCall {
    pub fn new(id: &'static str, tuple: Option<Tuple>) -> Self {
        ObjectCall { id, tuple }
    }
}

pub struct FuncCall {
    id: &'static str,
    expr: Vec<Expr>
}

impl FuncCall {
    pub fn new(id: &'static str, exprs: Vec<Expr>) -> Self {
        FuncCall { id, expr: exprs }
    }
}

pub struct Program {
    id: &'static str,
    int_const: i32,
    stmts: Vec<Stmt>
}

impl Program {
    pub fn new(id: &'static str, int_const: i32, stmts: Vec<Stmt>) -> Self {
        Program { id, int_const, stmts }
    }
}

// ==================================================================================== //
//  Stmts                                                                               //
// ==================================================================================== //

pub trait StmtTrait {
    fn sem(&self /*, Semantic Table */) -> Result<bool, ASTError>;
    fn run(&self) -> Result<u32, ASTError>;
    fn get_scope(&self);
    fn set_scope(&self);
}

pub enum Stmt {
    GenericTuple(GenericTuple),
    Tuple(Tuple),
    ObjectDecl(ObjectDecl),
    ObjectDesc(ObjectDesc),
    Program(Program)
}

impl Stmt {
    pub fn new_tuple(left: i32, right: i32) -> Self {
        Stmt::Tuple(Tuple::new(left, right))
    }

    pub fn new_generic_tuple(left: GenericTupleOption, right: GenericTupleOption) -> Self {
        Stmt::GenericTuple(GenericTuple::new(left, right))
    }

    pub fn new_object_decl(id: &'static str, desc: ObjectDesc) -> Self {
        Stmt::ObjectDecl(ObjectDecl::new(id, desc))
    }

    pub fn new_object_desc(shape: ObjectShape, color: ObjectColor) -> ObjectDesc {
        ObjectDesc::new(shape, color)
    }

    pub fn new_program(id: &'static str, int_const: i32, stmts: Vec<Stmt>) -> Self {
        Stmt::Program(Program::new(id, int_const, stmts))
    }
}

// ==================================================================================== //
//  Exprs                                                                               //
// ==================================================================================== //

pub trait ExprTrait {
    fn sem(&self /*, Semantic Table */) -> Result<bool, ASTError>;
    fn eval(&self) -> Result<u32, ASTError>;
    fn get_scope(&self);
    fn set_scope(&self);
}

pub enum Expr {
    FuncCall(FuncCall),
    ObjectCall(ObjectCall),
}

impl Expr {
    pub fn new_func_call(id: &'static str, tuple: Option<Tuple>) -> Self {
        Expr::FuncCall(FuncCall::new(id, tuple))
    }

    pub fn new_object_call(id: &'static str, tuple: Option<Tuple>) -> Self {
        Expr::ObjectCall(ObjectCall::new(id, tuple))
    }
}

// ==================================================================================== //
//  AST                                                                                 //
// ==================================================================================== //

pub enum AST {
    Stmt(Stmt),
    Expr(Expr)
}

impl AST {
    pub fn new_stmt(stmt: Stmt) -> Self {
        AST::Stmt(stmt)
    }

    pub fn new_expr(expr: Expr) -> Self {
        AST::Expr(expr)
    }
}
