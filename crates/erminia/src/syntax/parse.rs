use crate::error::parser_error::{ParserError, ParserResult};
use crate::lexer::lex::Lexer;
use crate::lexer::token::TokenKind;
use crate::syntax::ast::{ObjectDecl, Program, Stmt};

// ==================================================================================== //
//  Consumers                                                                           //
// ==================================================================================== //

fn next_is_comma(tokens: &mut Lexer) -> ParserResult<bool> {
    match tokens.peek().get_kind() {
        TokenKind::Comma => Ok(true),
        _ => Ok(false),
    }
}

fn next_is_stmt(tokens: &mut Lexer) -> ParserResult<bool> {
    match tokens.peek().get_kind() {
        TokenKind::Object | TokenKind::LetKwd | TokenKind::ProblemExample => Ok(true),
        _ => Ok(false),
    }
}

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

// <object_call> ::= <id> ( <tuple> | ε ) 
fn parse_object_call(tokens: &mut Lexer) -> ParserResult<()> {
    let _id = consume_identifier(tokens)?;
    match tokens.peek().get_kind() {
        TokenKind::LeftPar => {
            let _tuple = parse_shape_tuple(tokens)?;
        }
        _ => ()
    }
    // Ok(Stmt::ObjectCall(ObjectCall::default(id)))
    // Ok(Stmt::ObjectCall(ObjectCall::new(id, tuple)))
    Ok(())
}

// <shape_tuple> ::= "(" <int_const> "," <int_const> ")"
fn parse_shape_tuple(tokens: &mut Lexer) -> ParserResult<()> {
    consume_keyword(tokens, TokenKind::LeftPar)?;
    let _left = consume_int_const(tokens)?;
    consume_keyword(tokens, TokenKind::Comma)?;
    let _right = consume_int_const(tokens)?;
    consume_keyword(tokens, TokenKind::RightPar)?;
    Ok(())
}

// <shape> ::= <tuple> | <object_call> | <id> 
fn parse_shape(tokens: &mut Lexer) -> ParserResult<Stmt> {
    match tokens.peek().get_kind() {
        TokenKind::LeftPar => {
            let _shape = parse_shape_tuple(tokens)?;
            Ok(Stmt::ObjectDecl(ObjectDecl::new()))
            // Ok(Expr::Shape(Shape::new()))
        },
        TokenKind::Ident => {
            let _shape = parse_object_call(tokens)?;
            Ok(Stmt::ObjectDecl(ObjectDecl::new()))
            // Ok(Expr::Shape(Shape::new()))
        }
        _ => Err(ParserError::ParserError)
    }
}

// <object_color> ::= "color" ":" <int_const>
fn parse_object_color<'a>(tokens: &mut Lexer<'a>) -> ParserResult<&'a str> {
    consume_keyword(tokens, TokenKind::ObjectColor)?;
    consume_keyword(tokens, TokenKind::Colon)?;
    let int_const = consume_int_const(tokens)?;
    Ok(int_const)
}

// <list_of_shapes> ::= "[" <shape> ("," <shape>)* "]"
fn parse_list_of_shapes(tokens: &mut Lexer) -> ParserResult<()> {
    let mut _shapes: Vec<Stmt> = vec![];
    consume_keyword(tokens, TokenKind::LeftBrace)?;
    let shape = parse_shape(tokens);
    match shape {
        Ok(sh) => _shapes.push(sh),
        _ => () 
    }
    while next_is_comma(tokens)? {
        let shape = parse_shape(tokens)?;
        _shapes.push(shape);
    }
    consume_keyword(tokens, TokenKind::RightBrace)?;
    Ok(())
}


// TODO
// <object_shape> ::= "shape" ":" <list_of_shapes>
fn parse_object_shape(tokens: &mut Lexer) -> ParserResult<()> {
    consume_keyword(tokens, TokenKind::ObjectShape)?;
    consume_keyword(tokens, TokenKind::Colon)?;
    consume_keyword(tokens, TokenKind::LeftPar)?;
    let _shapes = parse_list_of_shapes(tokens)?;
    consume_keyword(tokens, TokenKind::RightPar)?;
    Ok(())
}

// TODO
// <object_desc> ::= <object_shape> "," <object_color> | <object_color> "," <object_shape>
fn parse_object_desc(tokens: &mut Lexer) -> ParserResult<()> {
    match tokens.peek().get_kind() {
        TokenKind::ObjectShape => {
            let _shape = parse_object_shape(tokens);
            let _color = parse_object_color(tokens);
            // Ok(Stmt::ObjectDesc(ObjectDesc::new(shape, color)))
            Ok(())
        }
        TokenKind::ObjectColor => {
            let _color = parse_object_color(tokens);
            let _shape = parse_object_shape(tokens);
            // Ok(Stmt::ObjectDesc(ObjectDesc::new(shape, color)))
            Ok(())
        }
        _ => Err(ParserError::ParserError),
    }
}

// TODO
// <inner_compound_stmt> ::= <var_def>
fn parse_inner_compound_stmt(_tokens: &mut Lexer) -> ParserResult<()> {
    Ok(())
}

// TODO
// <stmt> ::= <object_def> | <example_def> | <var_def>
fn parse_stmt(tokens: &mut Lexer) -> ParserResult<Stmt> {
    match tokens.peek().get_kind() {
        TokenKind::Object => {
            consume_keyword(tokens, TokenKind::Object)?;
            let _id = consume_identifier(tokens)?;
            let _desc = parse_object_desc(tokens)?;
            consume_keyword(tokens, TokenKind::SemiColon)?;
            // Ok(Stmt::ObjectDecl(ObjectDecl::new(id, desc)))
            Ok(Stmt::ObjectDecl(ObjectDecl::new()))
        }
        TokenKind::ProblemExample => {
            consume_keyword(tokens, TokenKind::ProblemExample)?;
            let _id = consume_identifier(tokens)?;
            let _desc = parse_inner_compound_stmt(tokens)?;
            // Ok(Stmt::ExampleDecl(ExampleDecl::new(id, desc)))
            Ok(Stmt::ObjectDecl(ObjectDecl::new()))
        }
        TokenKind::LetKwd => {
            let _ = consume_data_type(tokens)?;
            let _id = consume_identifier(tokens)?;
            consume_keyword(tokens, TokenKind::Equals)?;
            let _expr = parse_object_call(tokens)?;
            // Ok(Stmt::VarDef(VarDef::new(type, id, expr)))
            Ok(Stmt::ObjectDecl(ObjectDecl::new()))
        }
        _ => Err(ParserError::ExpectedKeyWordError),
    }
}

// <stmts_list> ::= <stmt> (<stmt>)*
fn parse_list_of_stmts(tokens: &mut Lexer) -> ParserResult<Vec<Stmt>> {
    let mut stmts: Vec<Stmt> = vec![];
    while next_is_stmt(tokens)? {
        let stmt = parse_stmt(tokens)?;
        stmts.push(stmt);
    }
    Ok(stmts)
}

// <object_compound_desc> ::= "{" <object_desc> "}"
fn parse_object_compound_desc(tokens: &mut Lexer) -> ParserResult<()> {
    consume_keyword(tokens, TokenKind::LeftBrace)?;
    let object_desc = parse_object_desc(tokens)?;
    consume_keyword(tokens, TokenKind::RightBrace)?;
    Ok(object_desc)
}

// <object_def> ::= "object" <id> <object_desc> ";"
fn parse_object_decl(tokens: &mut Lexer) -> ParserResult<()> {
    consume_keyword(tokens, TokenKind::Object)?;
    let _id = tokens.peek().get_text();
    consume_identifier(tokens)?;
    let _object_desc = parse_object_compound_desc(tokens)?;
    consume_keyword(tokens, TokenKind::SemiColon)?;
    Ok(())
}

// <compound_stmt> ::= "{" [<stmt_list>] "}"
fn parse_compound_stmt(tokens: &mut Lexer) -> ParserResult<Vec<Stmt>> {
    consume_keyword(tokens, TokenKind::LeftBrace)?;
    let stmts: Vec<Stmt> = parse_list_of_stmts(tokens)?;
    consume_keyword(tokens, TokenKind::RightBrace)?;
    Ok(stmts)
}

// <problem_declaration> ::= "def" <id> "(" <int_const> ")" <compound_stmt>
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
        let text = "def hello (2) {

            object LShape { shape : [(0,0), (1,0), (1,1)], color : 1 };

        }";

        let mut parser = Parser::new(&text);

        let program = parser.parse();

        println!("{:?}", program);

        assert_eq!(
            Program::new("hello".to_string(), 20, vec![]),
            program.expect("")
        );
    }
}
