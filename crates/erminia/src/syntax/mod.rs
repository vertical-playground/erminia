use crate::ast::ast::BoxAST;
use crate::diagnostics::DiagnosticAccumulator;
use crate::lexer::lex::Lexer;
use crate::syntax::parse::parse_program;
mod consumers;
mod parse;

// ==================================================================================== //
// Parser Object                                                                        //
// ==================================================================================== //

pub struct Parser<'a> {
    lexer: Lexer<'a>,
    diagnostics: DiagnosticAccumulator,
}

impl<'a> Parser<'a> {
    pub fn new<'input>(input: &'input str) -> Parser<'input> {
        let lexer = Lexer::new(input);
        let diagnostics = DiagnosticAccumulator::new();
        Parser { lexer, diagnostics }
    }

    pub fn parse(&mut self) -> BoxAST<'a> {
        parse_program(&mut self.lexer, &mut self.diagnostics)
    }

    pub fn get_diagnostics(&self) -> &DiagnosticAccumulator {
        &self.diagnostics
    }
}
