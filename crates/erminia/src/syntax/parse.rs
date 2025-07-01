use crate::error::parser_error::{ParserError, ParserResult};
use crate::lexer::lex::Lexer;
use crate::lexer::token::TokenKind;
use crate::syntax::ast::{ObjectDecl, Program, Stmt};

// ==================================================================================== //
//  Consumers                                                                           //
// ==================================================================================== //

fn consume_data_type(tokens: &mut Lexer) -> ParserResult<()> {
    match tokens.peek().get_kind() {
        TokenKind::Object => {
            tokens.advance()?;
            Ok(())
        }
        _ => Err(ParserError::ParserError),
    }
}

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

// ==================================================================================== //
// Parsers                                                                              //
// ==================================================================================== //

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

// TODO
// <inner_compound_stmt> ::= <var_def>
fn parse_inner_compound_stmt(_tokens: &mut Lexer) -> ParserResult<Stmt> {
    Ok(Stmt::ObjectDecl(ObjectDecl::new()))
}

// TODO:
fn parse_object_call(_tokens: &mut Lexer) -> ParserResult<Stmt> {
    Ok(Stmt::ObjectDecl(ObjectDecl::new()))
}

// TODO
// <stmt> ::= <object_def> | <example_def> | <var_def>
fn parse_stmt(tokens: &mut Lexer) -> ParserResult<Stmt> {
    match tokens.peek().get_kind() {
        TokenKind::Object => {
            consume_keyword(tokens, TokenKind::Object)?;
            let id = consume_identifier(tokens)?;
            let desc = parse_object_desc(tokens)?;
            Ok(Stmt::ObjectDecl(ObjectDecl::new()))
        }
        TokenKind::ProblemExample => {
            consume_keyword(tokens, TokenKind::ProblemExample)?;
            let id = consume_identifier(tokens)?;
            let desc = parse_inner_compound_stmt(tokens)?;
            // Ok(Stmt::ProblemExample(ProblemExample::new(id, desc)))
            Ok(Stmt::ObjectDecl(ObjectDecl::new()))
        }
        TokenKind::LetKwd => {
            let _ = consume_data_type(tokens)?;
            let id = consume_identifier(tokens)?;
            consume_keyword(tokens, TokenKind::Equals)?;
            let expr = parse_object_call(tokens)?;
            // Ok(Stmt::VarDef(VarDef::new(type, id, expr)))
            Ok(Stmt::ObjectDecl(ObjectDecl::new()))
        }
        _ => Err(ParserError::ExpectedKeyWordError),
    }
}

// <stmts_list> ::= <stmt> ("," <stmt>)*
fn parse_list_of_stmts(tokens: &mut Lexer) -> ParserResult<Vec<Stmt>> {
    let mut stmts: Vec<Stmt> = vec![];
    loop {
        let stmt = parse_stmt(tokens)?;
        stmts.push(stmt);
        let (kind, _offset) = tokens.lookahead()?;
        if kind != TokenKind::Comma {
            break;
        }
    }
    Ok(stmts)
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
pub fn parse_program(tokens: &mut Lexer) -> ParserResult<Program> {
    // [START] Token is first
    tokens.advance()?;
    let program = parse_problem_decl(tokens)?;
    Ok(program)
}

// ==================================================================================== //
// Parser Object                                                                        //
// ==================================================================================== //

pub struct Parser<'a> {
    lexer: Lexer<'a>,
}

impl<'a> Parser<'a> {
    pub fn new<'input>(input: &'input str) -> Parser<'input> {
        let lexer = Lexer::new(input);
        Parser { lexer: lexer }
    }

    pub fn parse(&mut self) -> ParserResult<Program> {
        parse_program(&mut self.lexer)
    }
}

// ==================================================================================== //
// Parser Test Suite                                                                    //
// ==================================================================================== //

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

        let mut parser = Parser::new(&text);

        let program = parser.parse();

        assert_eq!(
            Program::new("hello".to_string(), 20, vec![]),
            program.expect("")
        );
    }
}
