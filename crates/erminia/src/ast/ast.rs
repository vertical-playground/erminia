use crate::ast::expr::*;
use crate::ast::printon::*;
use crate::ast::stmt::*;
use crate::diag;
use crate::diagnostics::{DiagnosticAccumulator, Span};
use crate::lexer::lex::Lexer;

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

pub trait AST<'a>: 'a + PrettyPrinting {
    fn sem(&self /*, Semantic Table */) -> Result<bool, ASTError>;
    fn is_err(&self) -> bool;
    fn is_ok(&self) -> bool;
    fn get_ast_id(&self) -> u32;
    fn to_string(&self) -> String;
    fn check_poisoning(&self, tokens: &mut Lexer, diag: &mut DiagnosticAccumulator);
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

    fn to_string(&self) -> String {
        "GenericTupleOption".to_string()
    }

    fn get_ast_id(&self) -> u32 {
        0
    }

    fn check_poisoning(&self, tokens: &mut Lexer, diag: &mut DiagnosticAccumulator) {
        if self.is_err() {
            diag!(
                AST,
                E0004,
                ExpectedASTNode(self.to_string(), self.get_ast_id().to_string()),
                tokens,
                diag,
                Span::default()
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

    fn to_string(&self) -> String {
        "ProblemExample".to_string()
    }

    fn get_ast_id(&self) -> u32 {
        self.unique_ast_id
    }

    fn check_poisoning(&self, tokens: &mut Lexer, diag: &mut DiagnosticAccumulator) {
        if self.is_err() {
            diag!(
                AST,
                E0004,
                ExpectedASTNode(self.to_string(), self.get_ast_id().to_string()),
                tokens,
                diag,
                Span::default()
            );
        }

        for stmt in &self.stmts {
            stmt.check_poisoning(tokens, diag);
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

    fn to_string(&self) -> String {
        "ProblemSolution".to_string()
    }

    fn get_ast_id(&self) -> u32 {
        self.unique_ast_id
    }

    fn check_poisoning(&self, tokens: &mut Lexer, diag: &mut DiagnosticAccumulator) {
        if self.is_err() {
            diag!(
                AST,
                E0004,
                ExpectedASTNode(self.to_string(), self.get_ast_id().to_string()),
                tokens,
                diag,
                Span::default()
            );
        }

        for stmt in &self.stmts {
            stmt.check_poisoning(tokens, diag);
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

    fn to_string(&self) -> String {
        "ProblemInput".to_string()
    }

    fn get_ast_id(&self) -> u32 {
        self.unique_ast_id
    }

    fn check_poisoning(&self, tokens: &mut Lexer, diag: &mut DiagnosticAccumulator) {
        if self.is_err() {
            diag!(
                AST,
                E0004,
                ExpectedASTNode(self.to_string(), self.get_ast_id().to_string()),
                tokens,
                diag,
                Span::default()
            );
        }

        for stmt in &self.stmts {
            stmt.check_poisoning(tokens, diag);
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

    fn to_string(&self) -> String {
        "ProblemOutput".to_string()
    }

    fn get_ast_id(&self) -> u32 {
        self.unique_ast_id
    }

    fn check_poisoning(&self, tokens: &mut Lexer, diag: &mut DiagnosticAccumulator) {
        if self.is_err() {
            diag!(
                AST,
                E0004,
                ExpectedASTNode(self.to_string(), self.get_ast_id().to_string()),
                tokens,
                diag,
                Span::default()
            );
        }

        for stmt in &self.stmts {
            stmt.check_poisoning(tokens, diag);
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

    fn to_string(&self) -> String {
        "Program".to_string()
    }

    fn get_ast_id(&self) -> u32 {
        self.unique_ast_id
    }

    fn check_poisoning(&self, tokens: &mut Lexer, diag: &mut DiagnosticAccumulator) {
        if self.is_err() {
            diag!(
                AST,
                E0004,
                ExpectedASTNode(self.to_string(), self.get_ast_id().to_string()),
                tokens,
                diag,
                Span::default()
            );
        }

        for stmt in &self.stmts {
            stmt.check_poisoning(tokens, diag);
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

    fn to_string(&self) -> String {
        "Range".to_string()
    }

    fn get_ast_id(&self) -> u32 {
        self.unique_ast_id
    }

    fn check_poisoning(&self, tokens: &mut Lexer, diag: &mut DiagnosticAccumulator) {
        if self.is_err() {
            diag!(
                AST,
                E0004,
                ExpectedASTNode(self.to_string(), self.get_ast_id().to_string()),
                tokens,
                diag,
                Span::default()
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

    fn to_string(&self) -> String {
        "TupleIterator".to_string()
    }

    fn get_ast_id(&self) -> u32 {
        self.unique_ast_id
    }

    fn check_poisoning(&self, tokens: &mut Lexer, diag: &mut DiagnosticAccumulator) {
        if self.is_err() {
            diag!(
                AST,
                E0004,
                ExpectedASTNode(self.to_string(), self.get_ast_id().to_string()),
                tokens,
                diag,
                Span::default()
            );
        }

        let _ = &self.range.check_poisoning(tokens, diag);
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

    fn to_string(&self) -> String {
        "TupleComprehension".to_string()
    }

    fn get_ast_id(&self) -> u32 {
        self.unique_ast_id
    }

    fn check_poisoning(&self, tokens: &mut Lexer, diag: &mut DiagnosticAccumulator) {
        if self.is_err() {
            diag!(
                AST,
                E0004,
                ExpectedASTNode(self.to_string(), self.get_ast_id().to_string()),
                tokens,
                diag,
                Span::default()
            );
        }

        for iter in &self.iter_pair {
            iter.check_poisoning(tokens, diag);
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

    fn to_string(&self) -> String {
        "GenericTuple".to_string()
    }

    fn get_ast_id(&self) -> u32 {
        self.unique_ast_id
    }

    fn check_poisoning(&self, tokens: &mut Lexer, diag: &mut DiagnosticAccumulator) {
        if self.is_err() {
            diag!(
                AST,
                E0004,
                ExpectedASTNode(self.to_string(), self.get_ast_id().to_string()),
                tokens,
                diag,
                Span::default()
            );
        }

        let _ = &self.left.check_poisoning(tokens, diag);
        let _ = &self.right.check_poisoning(tokens, diag);
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

    fn to_string(&self) -> String {
        "Tuple".to_string()
    }

    fn get_ast_id(&self) -> u32 {
        self.unique_ast_id
    }

    fn check_poisoning(&self, tokens: &mut Lexer, diag: &mut DiagnosticAccumulator) {
        if self.is_err() {
            diag!(
                AST,
                E0004,
                ExpectedASTNode(self.to_string(), self.get_ast_id().to_string()),
                tokens,
                diag,
                Span::default()
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

    fn to_string(&self) -> String {
        "Shape".to_string()
    }

    fn get_ast_id(&self) -> u32 {
        self.unique_ast_id
    }

    fn check_poisoning(&self, tokens: &mut Lexer, diag: &mut DiagnosticAccumulator) {
        if self.is_err() {
            diag!(
                AST,
                E0004,
                ExpectedASTNode(self.to_string(), self.get_ast_id().to_string()),
                tokens,
                diag,
                Span::default()
            );
        }

        let _ = &self.values.check_poisoning(tokens, diag);
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

    fn to_string(&self) -> String {
        "ObjectShape".to_string()
    }

    fn get_ast_id(&self) -> u32 {
        self.unique_ast_id
    }

    fn check_poisoning(&self, tokens: &mut Lexer, diag: &mut DiagnosticAccumulator) {
        if self.is_err() {
            diag!(
                AST,
                E0004,
                ExpectedASTNode(self.to_string(), self.get_ast_id().to_string()),
                tokens,
                diag,
                Span::default()
            );
        }

        for shape in &self.shape {
            shape.check_poisoning(tokens, diag);
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

    fn to_string(&self) -> String {
        "ObjectColor".to_string()
    }

    fn get_ast_id(&self) -> u32 {
        self.unique_ast_id
    }

    fn check_poisoning(&self, tokens: &mut Lexer, diag: &mut DiagnosticAccumulator) {
        if self.is_err() {
            diag!(
                AST,
                E0004,
                ExpectedASTNode(self.to_string(), self.get_ast_id().to_string()),
                tokens,
                diag,
                Span::default()
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

    fn to_string(&self) -> String {
        "ObjectDesc".to_string()
    }

    fn get_ast_id(&self) -> u32 {
        self.unique_ast_id
    }

    fn check_poisoning(&self, tokens: &mut Lexer, diag: &mut DiagnosticAccumulator) {
        if self.is_err() {
            diag!(
                AST,
                E0004,
                ExpectedASTNode(self.to_string(), self.get_ast_id().to_string()),
                tokens,
                diag,
                Span::default()
            );
        }

        let _ = &self.shape.check_poisoning(tokens, diag);
        let _ = &self.color.check_poisoning(tokens, diag);
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

    fn to_string(&self) -> String {
        "ObjectDecl".to_string()
    }

    fn get_ast_id(&self) -> u32 {
        self.unique_ast_id
    }

    fn check_poisoning(&self, tokens: &mut Lexer, diag: &mut DiagnosticAccumulator) {
        if self.is_err() {
            diag!(
                AST,
                E0004,
                ExpectedASTNode(self.to_string(), self.get_ast_id().to_string()),
                tokens,
                diag,
                Span::default()
            );
        }

        let _ = &self.desc.check_poisoning(tokens, diag);
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

    fn to_string(&self) -> String {
        "VarDef".to_string()
    }

    fn get_ast_id(&self) -> u32 {
        self.unique_ast_id
    }

    fn check_poisoning(&self, tokens: &mut Lexer, diag: &mut DiagnosticAccumulator) {
        if self.is_err() {
            diag!(
                AST,
                E0004,
                ExpectedASTNode(self.to_string(), self.get_ast_id().to_string()),
                tokens,
                diag,
                Span::default()
            );
        }

        let _ = &self.expr.check_poisoning(tokens, diag);
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

    fn to_string(&self) -> String {
        "FuncCall".to_string()
    }

    fn get_ast_id(&self) -> u32 {
        self.unique_ast_id
    }

    fn check_poisoning(&self, tokens: &mut Lexer, diag: &mut DiagnosticAccumulator) {
        if self.is_err() {
            diag!(
                AST,
                E0004,
                ExpectedASTNode(self.to_string(), self.get_ast_id().to_string()),
                tokens,
                diag,
                Span::default()
            );
        }

        for expr in &self.exprs {
            expr.check_poisoning(tokens, diag);
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

    fn to_string(&self) -> String {
        "ObjectCall".to_string()
    }

    fn get_ast_id(&self) -> u32 {
        self.unique_ast_id
    }

    fn check_poisoning(&self, tokens: &mut Lexer, diag: &mut DiagnosticAccumulator) {
        if self.is_err() {
            diag!(
                AST,
                E0004,
                ExpectedASTNode(self.to_string(), self.get_ast_id().to_string()),
                tokens,
                diag,
                Span::default()
            );
        }

        if let Some(t) = &self.tuple {
            t.check_poisoning(tokens, diag);
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

    fn to_string(&self) -> String {
        "RValue".to_string()
    }

    fn get_ast_id(&self) -> u32 {
        0
    }

    fn check_poisoning(&self, tokens: &mut Lexer, diag: &mut DiagnosticAccumulator) {
        if self.is_err() {
            diag!(
                AST,
                E0004,
                ExpectedASTNode(self.to_string(), self.get_ast_id().to_string()),
                tokens,
                diag,
                Span::default()
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

    fn to_string(&self) -> String {
        "PoisonedStmt".to_string()
    }

    fn get_ast_id(&self) -> u32 {
        self.unique_ast_id
    }

    fn check_poisoning(&self, tokens: &mut Lexer, diag: &mut DiagnosticAccumulator) {
        diag!(
            AST,
            E0004,
            ExpectedASTNode(self.to_string(), self.get_ast_id().to_string()),
            tokens,
            diag,
            Span::default()
        );
    }
}
