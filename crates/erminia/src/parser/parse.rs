use crate::error::parser_error::{ParserError, ParserResult};
use crate::error::ast_error::{ASTError, ASTResult};
use crate::lexer::lex_s::Lexer;
use crate::lexer::token::TokenKind;

// ====================================================================================//
//                                      AST                                            //
// ====================================================================================//

#[derive(Debug, PartialEq)]
pub struct ObjectDecl {

}

#[derive(Debug, PartialEq)]
pub struct Program {
    id: String,
    int_const: i32,
    stmts: Vec<Stmt>
}

impl Program {
    fn new(id: String, int_const: i32, stmts: Vec<Stmt>) -> Program {
        Program {
            id: id,
            int_const: int_const,
            stmts: stmts
        }
    }
}

pub trait StmtTrait {
    fn sem(&self /*, Semantic Table */) -> ASTResult<()>;
    // fn run(&self) -> Result<u32, ASTError>;
    // fn get_scope(&self);
    // fn set_scope(&self);
}

#[derive(Debug, PartialEq)]
pub enum Stmt {
    ObjectDecl(ObjectDecl),
    Program(Program)
}


// ====================================================================================//
//                                Consumers                                            //
// ====================================================================================//

fn consume_int_const<'a>(tokens: &mut Lexer<'a>) -> ParserResult<&'a str> {
    let int_const = tokens.token;
    if int_const.get_kind() == TokenKind::Int {
        tokens.advance()?;
        Ok(int_const.text)
    } else {
        Err(ParserError::ExpectedIntegerConstError)
    }
}

fn consume_identifier<'a>(tokens: &mut Lexer<'a>) -> ParserResult<&'a str> {
    let id = tokens.token;
    match id.get_kind() {
        TokenKind::Ident => {
            tokens.advance()?;
            Ok(id.text)
        }
        _ => Err(ParserError::ExpectedIdentifierError),
    }
}

fn consume_keyword(tokens: &mut Lexer, kind: TokenKind) -> ParserResult<()> {
    if tokens.peek().get_kind() == kind {
        tokens.advance()?;
        Ok(())
    } else {
        Err(ParserError::ExpectedKeyWordError)
    }
}

// ====================================================================================//
//                                Parsers                                              //
// ====================================================================================//

fn parse_object_shape_desc(_tokens: &mut Lexer) -> ParserResult<()> {
    Ok(())
}

fn parse_object_prior_decl(_tokens: &mut Lexer) -> ParserResult<()> {
    Ok(())
}

// <shapes_list> ::= "[" <shape_desc> ("," <shape_desc>)* "]"
// fn parse_list_of_shapes<T>(tokens: &mut Lexer) -> ParserResult<Vec<T>> {
//     let mut shape_descs = vec![];
//     consume_keyword(tokens, TokenKind::LeftBracket)?;
//     loop {
//         let desc = parse_object_shape_desc(tokens)?;
//         shape_descs.push(desc);
//         let (kind, _offset) = tokens.lookahead()?;
//         if kind != TokenKind::Comma {
//             break
//         }
//         consume_keyword(tokens, TokenKind::Comma)?;
//     }
//     consume_keyword(tokens, TokenKind::RightBracket)?;
//     Ok(shape_descs)
// }

// <stmts_list> ::= <stmt> ("," <stmt>)*
fn parse_list_of_stmts(_tokens: &mut Lexer) -> ParserResult<Vec<Stmt>> {
    let stmts_list: Vec<Stmt> = vec![];
    Ok(stmts_list)
}

// <object_desc> ::= "{" <object_prior_decl> "}"
fn parse_object_desc(tokens: &mut Lexer) -> ParserResult<()> {
    consume_keyword(tokens, TokenKind::LeftBrace)?;
    let object_desc = parse_object_prior_decl(tokens)?;
    consume_keyword(tokens, TokenKind::RightBrace)?;
    Ok(object_desc)
}

// <object_def> ::= "object" <id> <object_desc>
fn parse_object_decl(tokens: &mut Lexer) -> ParserResult<()> {
    consume_keyword(tokens, TokenKind::Object)?;
    let _id = tokens.peek().get_text();
    consume_identifier(tokens)?;
    let _object_desc = parse_object_desc(tokens)?;
    Ok(())
}

// <compound_stmt> ::= "{" [<stmt_list>] "}"
fn parse_compound_stmt(tokens: &mut Lexer) -> ParserResult<Vec<Stmt>> {
    consume_keyword(tokens, TokenKind::LeftBrace)?;
    let stmts: Vec<Stmt> = parse_list_of_stmts(tokens)?;
    consume_keyword(tokens, TokenKind::RightBrace)?;
    Ok(stmts)
}

// <problem_declaration> ::= "def" <id> "(" [<int_const>] ")" <compound_stmt>
fn parse_problem_decl(tokens: &mut Lexer) -> ParserResult<Program> {
    consume_keyword(tokens, TokenKind::ProblemDef)?;
    let id = consume_identifier(tokens)?;
    consume_keyword(tokens, TokenKind::LeftPar)?;
    let int_const = consume_int_const(tokens)?;
    consume_keyword(tokens, TokenKind::RightPar)?;
    let stmts: Vec<Stmt> = parse_compound_stmt(tokens)?;
    let program = Program::new(id.to_string(), int_const.parse::<i32>().unwrap(), stmts); 
    Ok(program)
}

// <program> ::= <problem_declaration>
fn parse_program(tokens: &mut Lexer) -> ParserResult<Program> {
    // [START] Token is first
    tokens.advance()?;
    let program = parse_problem_decl(tokens)?;
    Ok(program)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_start() {
        let text = "def hello (20) {

            object LShape {
                shape : [(0,0), (1,0), (1,1)]
                color : \"blue\"
            }

        }";

        let mut lexer = Lexer::new(text);

        let program = parse_program(&mut lexer);

        assert_eq!(Program::new("hello".to_string(), 20, vec![]), program.expect(""));
    }
}
