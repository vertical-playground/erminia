use crate::ast::expr::*;
use crate::ast::stmt::*;
use crate::diagnostics::code::Code;
use crate::diagnostics::location::{Accumulator, Span};
use crate::diagnostics::messages::{MessageKind, Note};
use crate::lexer::lex::Lexer;

pub type BoxAST<'a> = Box<dyn AST<'a> + 'a>;
pub type ASTError = String;
type DB = crate::diagnostics::DiagnosticBuilder;
const AST_PASS: crate::config::CompilerPass = crate::config::CompilerPass::AST;

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
    fn get_ast_id(&self) -> u32;
    fn to_string(&self) -> String;
    fn check_poisoning(&self, tokens: &mut Lexer, diag: &mut Accumulator, span: Span);
}

impl<'a> std::fmt::Debug for dyn AST<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let depth: i32 = 0;
        self.print_on(f, depth)?;
        Ok(())
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

    fn to_string(&self) -> String {
        "GenericTupleOption".to_string()
    }

    fn get_ast_id(&self) -> u32 {
        0
    }

    fn check_poisoning(&self, tokens: &mut Lexer, diag: &mut Accumulator, span: Span) {
        if self.is_err() {
            diag.add_diag(
                DB::build(AST_PASS, Code::E0004)
                    .with_note(Note::ExpectedASTNode)
                    .with_args(
                        MessageKind::Note,
                        vec![self.to_string(), self.get_ast_id().to_string()],
                    )
                    .emmit(tokens, span),
            );
        }
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

    fn to_string(&self) -> String {
        "ProblemExample".to_string()
    }

    fn get_ast_id(&self) -> u32 {
        self.unique_ast_id
    }

    fn check_poisoning(&self, tokens: &mut Lexer, diag: &mut Accumulator, span: Span) {
        if self.is_err() {
            diag.add_diag(
                DB::build(AST_PASS, Code::E0004)
                    .with_note(Note::ExpectedASTNode)
                    .with_args(
                        MessageKind::Note,
                        vec![self.to_string(), self.get_ast_id().to_string()],
                    )
                    .emmit(tokens, span),
            );
        }

        for stmt in &self.stmts {
            stmt.check_poisoning(tokens, diag, span);
        }
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

    fn to_string(&self) -> String {
        "ProblemSolution".to_string()
    }

    fn get_ast_id(&self) -> u32 {
        self.unique_ast_id
    }

    fn check_poisoning(&self, tokens: &mut Lexer, diag: &mut Accumulator, span: Span) {
        if self.is_err() {
            diag.add_diag(
                DB::build(AST_PASS, Code::E0004)
                    .with_note(Note::ExpectedASTNode)
                    .with_args(
                        MessageKind::Note,
                        vec![self.to_string(), self.get_ast_id().to_string()],
                    )
                    .emmit(tokens, span),
            );
        }

        for stmt in &self.stmts {
            stmt.check_poisoning(tokens, diag, span);
        }
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
        let s = format!("<#{} ProblemInput id: {:?}>", self.unique_ast_id, self.id);
        writeln!(f, "{}", s)?;
        for stmt in &self.stmts {
            stmt.print_on(f, depth)?;
        }
        Ok(())
    }

    fn to_string(&self) -> String {
        "ProblemInput".to_string()
    }

    fn get_ast_id(&self) -> u32 {
        self.unique_ast_id
    }

    fn check_poisoning(&self, tokens: &mut Lexer, diag: &mut Accumulator, span: Span) {
        if self.is_err() {
            diag.add_diag(
                DB::build(AST_PASS, Code::E0004)
                    .with_note(Note::ExpectedASTNode)
                    .with_args(
                        MessageKind::Note,
                        vec![self.to_string(), self.get_ast_id().to_string()],
                    )
                    .emmit(tokens, span),
            );
        }

        for stmt in &self.stmts {
            stmt.check_poisoning(tokens, diag, span);
        }
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
        let s = format!("<#{} ProblemOutput id: {:?}>", self.unique_ast_id, self.id);
        writeln!(f, "{}", s)?;
        for stmt in &self.stmts {
            stmt.print_on(f, depth)?;
        }
        Ok(())
    }

    fn to_string(&self) -> String {
        "ProblemOutput".to_string()
    }

    fn get_ast_id(&self) -> u32 {
        self.unique_ast_id
    }

    fn check_poisoning(&self, tokens: &mut Lexer, diag: &mut Accumulator, span: Span) {
        if self.is_err() {
            diag.add_diag(
                DB::build(AST_PASS, Code::E0004)
                    .with_note(Note::ExpectedASTNode)
                    .with_args(
                        MessageKind::Note,
                        vec![self.to_string(), self.get_ast_id().to_string()],
                    )
                    .emmit(tokens, span),
            );
        }

        for stmt in &self.stmts {
            stmt.check_poisoning(tokens, diag, span);
        }
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

    fn to_string(&self) -> String {
        "Program".to_string()
    }

    fn get_ast_id(&self) -> u32 {
        self.unique_ast_id
    }

    fn check_poisoning(&self, tokens: &mut Lexer, diag: &mut Accumulator, span: Span) {
        if self.is_err() {
            diag.add_diag(
                DB::build(AST_PASS, Code::E0004)
                    .with_note(Note::ExpectedASTNode)
                    .with_args(
                        MessageKind::Note,
                        vec![self.to_string(), self.get_ast_id().to_string()],
                    )
                    .emmit(tokens, span),
            );
        }

        for stmt in &self.stmts {
            stmt.check_poisoning(tokens, diag, span);
        }
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
            "<#{} Range left_inclusive: {}, right_inclusive: {}, left: {}, right: {}>",
            self.unique_ast_id, self.left_inclusive, self.right_inclusive, self.left, self.right
        );
        writeln!(f, "{}", s)?;
        Ok(())
    }

    fn to_string(&self) -> String {
        "Range".to_string()
    }

    fn get_ast_id(&self) -> u32 {
        self.unique_ast_id
    }

    fn check_poisoning(&self, tokens: &mut Lexer, diag: &mut Accumulator, span: Span) {
        if self.is_err() {
            diag.add_diag(
                DB::build(AST_PASS, Code::E0004)
                    .with_note(Note::ExpectedASTNode)
                    .with_args(
                        MessageKind::Note,
                        vec![self.to_string(), self.get_ast_id().to_string()],
                    )
                    .emmit(tokens, span),
            );
        }
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
        let s = format!("<#{} TupleIterator id: {:?}>", self.unique_ast_id, self.id);
        writeln!(f, "{}", s)?;
        let _ = &self.range.print_on(f, depth)?;
        Ok(())
    }

    fn to_string(&self) -> String {
        "TupleIterator".to_string()
    }

    fn get_ast_id(&self) -> u32 {
        self.unique_ast_id
    }

    fn check_poisoning(&self, tokens: &mut Lexer, diag: &mut Accumulator, span: Span) {
        if self.is_err() {
            diag.add_diag(
                DB::build(AST_PASS, Code::E0004)
                    .with_note(Note::ExpectedASTNode)
                    .with_args(
                        MessageKind::Note,
                        vec![self.to_string(), self.get_ast_id().to_string()],
                    )
                    .emmit(tokens, span),
            );
        }

        let _ = &self.range.check_poisoning(tokens, diag, span);
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
        let s = format!("<#{} TupleComprehension>", self.unique_ast_id);
        writeln!(f, "{}", s)?;
        let _ = &self.tuple.print_on(f, depth)?;
        for iter in &self.iter_pair {
            iter.print_on(f, depth)?;
        }
        Ok(())
    }

    fn to_string(&self) -> String {
        "TupleComprehension".to_string()
    }

    fn get_ast_id(&self) -> u32 {
        self.unique_ast_id
    }

    fn check_poisoning(&self, tokens: &mut Lexer, diag: &mut Accumulator, span: Span) {
        if self.is_err() {
            diag.add_diag(
                DB::build(AST_PASS, Code::E0004)
                    .with_note(Note::ExpectedASTNode)
                    .with_args(
                        MessageKind::Note,
                        vec![self.to_string(), self.get_ast_id().to_string()],
                    )
                    .emmit(tokens, span),
            );
        }

        for iter in &self.iter_pair {
            iter.check_poisoning(tokens, diag, span);
        }
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
        let s = format!("<#{} GenericTuple>", self.unique_ast_id);
        writeln!(f, "{}", s)?;
        let _ = &self.left.print_on(f, depth)?;
        let _ = &self.right.print_on(f, depth)?;
        Ok(())
    }

    fn to_string(&self) -> String {
        "GenericTuple".to_string()
    }

    fn get_ast_id(&self) -> u32 {
        self.unique_ast_id
    }

    fn check_poisoning(&self, tokens: &mut Lexer, diag: &mut Accumulator, span: Span) {
        if self.is_err() {
            diag.add_diag(
                DB::build(AST_PASS, Code::E0004)
                    .with_note(Note::ExpectedASTNode)
                    .with_args(
                        MessageKind::Note,
                        vec![self.to_string(), self.get_ast_id().to_string()],
                    )
                    .emmit(tokens, span),
            );
        }

        let _ = &self.left.check_poisoning(tokens, diag, span);
        let _ = &self.right.check_poisoning(tokens, diag, span);
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
        let s = format!(
            "<#{} Tuple left: {}, right: {}>",
            self.unique_ast_id, self.left, self.right
        );
        writeln!(f, "{}", s)?;
        Ok(())
    }

    fn to_string(&self) -> String {
        "Tuple".to_string()
    }

    fn get_ast_id(&self) -> u32 {
        self.unique_ast_id
    }

    fn check_poisoning(&self, tokens: &mut Lexer, diag: &mut Accumulator, span: Span) {
        if self.is_err() {
            diag.add_diag(
                DB::build(AST_PASS, Code::E0004)
                    .with_note(Note::ExpectedASTNode)
                    .with_args(
                        MessageKind::Note,
                        vec![self.to_string(), self.get_ast_id().to_string()],
                    )
                    .emmit(tokens, span),
            );
        }
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
        let s = format!(
            "<#{} Shape type: {:?}>",
            self.unique_ast_id, self.shape_type
        );
        writeln!(f, "{}", s)?;
        let _ = &self.values.print_on(f, depth)?;
        Ok(())
    }

    fn to_string(&self) -> String {
        "Shape".to_string()
    }

    fn get_ast_id(&self) -> u32 {
        self.unique_ast_id
    }

    fn check_poisoning(&self, tokens: &mut Lexer, diag: &mut Accumulator, span: Span) {
        if self.is_err() {
            diag.add_diag(
                DB::build(AST_PASS, Code::E0004)
                    .with_note(Note::ExpectedASTNode)
                    .with_args(
                        MessageKind::Note,
                        vec![self.to_string(), self.get_ast_id().to_string()],
                    )
                    .emmit(tokens, span),
            );
        }

        let _ = &self.values.check_poisoning(tokens, diag, span);
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
        let s = format!("<#{} ObjectShape>", self.unique_ast_id);
        writeln!(f, "{}", s)?;
        for shape in &self.shape {
            shape.print_on(f, depth)?;
        }
        Ok(())
    }

    fn to_string(&self) -> String {
        "ObjectShape".to_string()
    }

    fn get_ast_id(&self) -> u32 {
        self.unique_ast_id
    }

    fn check_poisoning(&self, tokens: &mut Lexer, diag: &mut Accumulator, span: Span) {
        if self.is_err() {
            diag.add_diag(
                DB::build(AST_PASS, Code::E0004)
                    .with_note(Note::ExpectedASTNode)
                    .with_args(
                        MessageKind::Note,
                        vec![self.to_string(), self.get_ast_id().to_string()],
                    )
                    .emmit(tokens, span),
            );
        }

        for shape in &self.shape {
            shape.check_poisoning(tokens, diag, span);
        }
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
        let s = format!(
            "<#{} ObjectColor color: {}>",
            self.unique_ast_id, self.color
        );
        writeln!(f, "{}", s)?;
        Ok(())
    }

    fn to_string(&self) -> String {
        "ObjectColor".to_string()
    }

    fn get_ast_id(&self) -> u32 {
        self.unique_ast_id
    }

    fn check_poisoning(&self, tokens: &mut Lexer, diag: &mut Accumulator, span: Span) {
        if self.is_err() {
            diag.add_diag(
                DB::build(AST_PASS, Code::E0004)
                    .with_note(Note::ExpectedASTNode)
                    .with_args(
                        MessageKind::Note,
                        vec![self.to_string(), self.get_ast_id().to_string()],
                    )
                    .emmit(tokens, span),
            );
        }
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
        let s = format!("<#{} ObjectDesc>", self.unique_ast_id);
        writeln!(f, "{}", s)?;
        let _ = &self.shape.print_on(f, depth)?;
        let _ = &self.color.print_on(f, depth)?;
        Ok(())
    }

    fn to_string(&self) -> String {
        "ObjectDesc".to_string()
    }

    fn get_ast_id(&self) -> u32 {
        self.unique_ast_id
    }

    fn check_poisoning(&self, tokens: &mut Lexer, diag: &mut Accumulator, span: Span) {
        if self.is_err() {
            diag.add_diag(
                DB::build(AST_PASS, Code::E0004)
                    .with_note(Note::ExpectedASTNode)
                    .with_args(
                        MessageKind::Note,
                        vec![self.to_string(), self.get_ast_id().to_string()],
                    )
                    .emmit(tokens, span),
            );
        }

        let _ = &self.shape.check_poisoning(tokens, diag, span);
        let _ = &self.color.check_poisoning(tokens, diag, span);
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
        let s = format!("<#{} ObjectDecl id: {:?}>", self.unique_ast_id, self.id);
        writeln!(f, "{}", s)?;
        let _ = &self.desc.print_on(f, depth)?;
        Ok(())
    }

    fn to_string(&self) -> String {
        "ObjectDecl".to_string()
    }

    fn get_ast_id(&self) -> u32 {
        self.unique_ast_id
    }

    fn check_poisoning(&self, tokens: &mut Lexer, diag: &mut Accumulator, span: Span) {
        if self.is_err() {
            diag.add_diag(
                DB::build(AST_PASS, Code::E0004)
                    .with_note(Note::ExpectedASTNode)
                    .with_args(
                        MessageKind::Note,
                        vec![self.to_string(), self.get_ast_id().to_string()],
                    )
                    .emmit(tokens, span),
            );
        }

        let _ = &self.desc.check_poisoning(tokens, diag, span);
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
            "<#{} VarDef id: {:?}, data_type: {:?}>",
            self.unique_ast_id, self.id, self.data_type
        );
        writeln!(f, "{}", s)?;
        let _ = &self.expr.print_on(f, depth)?;
        Ok(())
    }

    fn to_string(&self) -> String {
        "VarDef".to_string()
    }

    fn get_ast_id(&self) -> u32 {
        self.unique_ast_id
    }

    fn check_poisoning(&self, tokens: &mut Lexer, diag: &mut Accumulator, span: Span) {
        if self.is_err() {
            diag.add_diag(
                DB::build(AST_PASS, Code::E0004)
                    .with_note(Note::ExpectedASTNode)
                    .with_args(
                        MessageKind::Note,
                        vec![self.to_string(), self.get_ast_id().to_string()],
                    )
                    .emmit(tokens, span),
            );
        }

        let _ = &self.expr.check_poisoning(tokens, diag, span);
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
        let s = format!("<#{} FuncCall id: {:?}>", self.unique_ast_id, self.id);
        writeln!(f, "{}", s)?;
        for expr in &self.exprs {
            expr.print_on(f, depth)?;
        }
        Ok(())
    }

    fn to_string(&self) -> String {
        "FuncCall".to_string()
    }

    fn get_ast_id(&self) -> u32 {
        self.unique_ast_id
    }

    fn check_poisoning(&self, tokens: &mut Lexer, diag: &mut Accumulator, span: Span) {
        if self.is_err() {
            diag.add_diag(
                DB::build(AST_PASS, Code::E0004)
                    .with_note(Note::ExpectedASTNode)
                    .with_args(
                        MessageKind::Note,
                        vec![self.to_string(), self.get_ast_id().to_string()],
                    )
                    .emmit(tokens, span),
            );
        }

        for expr in &self.exprs {
            expr.check_poisoning(tokens, diag, span);
        }
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
        let s = format!("<#{} ObjectCall id: {:?}>", self.unique_ast_id, self.id);
        writeln!(f, "{}", s)?;
        if let Some(t) = &self.tuple {
            t.print_on(f, depth)?;
        };
        Ok(())
    }

    fn to_string(&self) -> String {
        "ObjectCall".to_string()
    }

    fn get_ast_id(&self) -> u32 {
        self.unique_ast_id
    }

    fn check_poisoning(&self, tokens: &mut Lexer, diag: &mut Accumulator, span: Span) {
        if self.is_err() {
            diag.add_diag(
                DB::build(AST_PASS, Code::E0004)
                    .with_note(Note::ExpectedASTNode)
                    .with_args(
                        MessageKind::Note,
                        vec![self.to_string(), self.get_ast_id().to_string()],
                    )
                    .emmit(tokens, span),
            );
        }

        if let Some(t) = &self.tuple {
            t.check_poisoning(tokens, diag, span);
        };
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

    fn to_string(&self) -> String {
        "RValue".to_string()
    }

    fn get_ast_id(&self) -> u32 {
        0
    }

    fn check_poisoning(&self, tokens: &mut Lexer, diag: &mut Accumulator, span: Span) {
        if self.is_err() {
            diag.add_diag(
                DB::build(AST_PASS, Code::E0004)
                    .with_note(Note::ExpectedASTNode)
                    .with_args(
                        MessageKind::Note,
                        vec![self.to_string(), self.get_ast_id().to_string()],
                    )
                    .emmit(tokens, span),
            );
        }
    }
}

impl<'a> AST<'a> for PoisonedStmt {
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
        let s = format!("<#{} PoisonedStmt>", self.unique_ast_id);
        writeln!(f, "{}", s)?;
        Ok(())
    }

    fn to_string(&self) -> String {
        "PoisonedStmt".to_string()
    }

    fn get_ast_id(&self) -> u32 {
        self.unique_ast_id
    }

    fn check_poisoning(&self, tokens: &mut Lexer, diag: &mut Accumulator, span: Span) {
        diag.add_diag(
            DB::build(AST_PASS, Code::E0004)
                .with_note(Note::ExpectedASTNode)
                .with_args(
                    MessageKind::Note,
                    vec![self.to_string(), self.get_ast_id().to_string()],
                )
                .emmit(tokens, span),
        );
    }
}
