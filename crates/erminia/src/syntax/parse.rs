use crate::error::parser_error::{ParserError, ParserResult};
use crate::diagnostics::diagnostics::Location;
use crate::lexer::lex::Lexer;
use crate::lexer::token::TokenKind;
use crate::syntax::ast::{ObjectDecl, Program, Stmt};

// ==================================================================================== //
//  Utilities                                                                           //
// ==================================================================================== //

fn is_next_right_inclusive(tokens: &mut Lexer) -> ParserResult<bool> {
    match tokens.peek().get_kind() {
        TokenKind::RightPar => Ok(false),
        TokenKind::RightBracket => Ok(true),
        _ => Err(ParserError::ExpectedRightInclusivity(
            Location::new(tokens.peek().get_start()),
            TokenKind::Object
        ))
    }
}

fn is_next_left_inclusive(tokens: &mut Lexer) -> ParserResult<bool> {
    match tokens.peek().get_kind() {
        TokenKind::LeftPar => Ok(false),
        TokenKind::LeftBracket => Ok(true),
        _ => Err(ParserError::ExpectedLeftInclusivity(
            Location::new(tokens.peek().get_start()),
            TokenKind::LeftBracket
        ))
    }
}

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
        _ => Err(ParserError::ParserError(
            Location::new(tokens.peek().get_start()),
            TokenKind::Object
        )),
    }
}

fn consume_int_const<'a>(tokens: &mut Lexer<'a>) -> ParserResult<&'a str> {
    let int_const = tokens.token;
    if int_const.get_kind() == TokenKind::Int {
        tokens.advance()?;
        Ok(int_const.text)
    } else {
        Err(ParserError::ExpectedIntegerConstError(Location::new(tokens.peek().get_start()), TokenKind::Int))
    }
}

fn consume_identifier<'a>(tokens: &mut Lexer<'a>) -> ParserResult<&'a str> {
    let id = tokens.token;
    match id.get_kind() {
        TokenKind::Ident => {
            tokens.advance()?;
            Ok(id.text)
        }
        _ => Err(ParserError::ExpectedIdentifierError(Location::new(tokens.peek().get_start()), TokenKind::Ident)),
    }
}

fn consume_keyword(tokens: &mut Lexer, kind: TokenKind) -> ParserResult<()> {
    if tokens.peek().get_kind() == kind {
        tokens.advance()?;
        Ok(())
    } else {
        Err(ParserError::ExpectedKeyWordError(Location::new(tokens.peek().get_start()), kind))
    }
}

// ==================================================================================== //
// Parsers                                                                              //
// ==================================================================================== //

// <range> ::= ("[" | "(") <int_const> ".." <int_const> ("]" | ")")
fn parse_range(tokens: &mut Lexer) -> ParserResult<()> {
    let _is_left_inclusive = is_next_left_inclusive(tokens)?;
    let _left = consume_int_const(tokens)?;
    consume_keyword(tokens, TokenKind::Range)?;
    let _right = consume_int_const(tokens)?;
    let _is_right_inclusive = is_next_right_inclusive(tokens)?;
    // return Range object
    Ok(())
}

// <shape_tuple_iter> ::= <coord> "<-" <range>
fn parse_shape_tuple_iter(tokens: &mut Lexer) -> ParserResult<()> {
    let _coord = consume_identifier(tokens)?;
    consume_keyword(tokens, TokenKind::LeftArrow)?;
    let _range = parse_range(tokens)?;
    Ok(())
}

// <shape_tuple_iter_pair> ::= <shape_tuple_iter> ("," <shape_tuple_iter>)
fn parse_shape_tuple_iter_pair(tokens: &mut Lexer) -> ParserResult<()> {
    let _first_tuple_iter = parse_shape_tuple_iter(tokens)?;
    if next_is_comma(tokens)? {
        let _second_tuple_iter = parse_shape_tuple_iter(tokens)?;
    }
    Ok(())
}

// <shape_tuple_compr> ::= <shape_tuple> "|" <shape_tuple_iter_pair>
fn parse_shape_tuple_compr(tokens: &mut Lexer) -> ParserResult<()> {
    let _tuple = parse_shape_tuple(tokens)?;
    consume_keyword(tokens, TokenKind::Pipe)?;
    // we may need to include _tuple here to assign coordinates correctly
    let _tuple_iter_pair = parse_shape_tuple_iter_pair(tokens)?;
    Ok(())
}

// <object_call> ::= <id> ( <shape_tuple> | ε ) 
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

// <shape> ::= <shape_tuple> | <shape_tuple_compr> | <object_call> | <id> 
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
        _ => Err(ParserError::ParserError(Location::new(tokens.peek().get_start()), TokenKind::LeftPar))
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
    consume_keyword(tokens, TokenKind::LeftBracket)?;
    let shape = parse_shape(tokens);
    match shape {
        Ok(sh) => _shapes.push(sh),
        _ => () 
    }
    while next_is_comma(tokens)? {
        consume_keyword(tokens, TokenKind::Comma)?;
        let shape = parse_shape(tokens)?;
        _shapes.push(shape);
    }
    consume_keyword(tokens, TokenKind::RightBracket)?;
    Ok(())
}


// TODO
// <object_shape> ::= "shape" ":" <list_of_shapes>
fn parse_object_shape(tokens: &mut Lexer) -> ParserResult<()> {
    consume_keyword(tokens, TokenKind::ObjectShape)?;
    consume_keyword(tokens, TokenKind::Colon)?;
    let _shapes = parse_list_of_shapes(tokens)?;
    Ok(())
}

// TODO
// <object_desc> ::= <object_shape> "," <object_color> | <object_color> "," <object_shape>
fn parse_object_desc(tokens: &mut Lexer) -> ParserResult<()> {
    match tokens.peek().get_kind() {
        TokenKind::ObjectShape => {
            let _shape = parse_object_shape(tokens)?;
            consume_keyword(tokens, TokenKind::Comma)?;
            let _color = parse_object_color(tokens)?;
            // Ok(Stmt::ObjectDesc(ObjectDesc::new(shape, color)))
            Ok(())
        }
        TokenKind::ObjectColor => {
            let _color = parse_object_color(tokens)?;
            consume_keyword(tokens, TokenKind::Comma)?;
            let _shape = parse_object_shape(tokens)?;
            // Ok(Stmt::ObjectDesc(ObjectDesc::new(shape, color)))
            Ok(())
        }
        _ => Err(ParserError::ParserError(Location::new(tokens.peek().get_start()), TokenKind::ObjectShape)),
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
        _ => Err(ParserError::ExpectedKeyWordError(Location::new(tokens.peek().get_start()), TokenKind::Object)),
    }
}

// <stmts_list> ::= (<stmt>)*
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
    let _object_desc = parse_object_desc(tokens)?;
    consume_keyword(tokens, TokenKind::RightBrace)?;
    Ok(())
}

// <object_def> ::= "object" <id> <object_compound_desc> ";"
fn parse_object_decl(tokens: &mut Lexer) -> ParserResult<()> {
    consume_keyword(tokens, TokenKind::Object)?;
    let _id = consume_identifier(tokens)?;
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
    fn test_range() {
        let text = "object Shape { shape : [(0,1), (1,1)], color: 1 };";

        let mut lexer = Lexer::new(&text);

        let _ = lexer.advance();

        let res = parse_object_decl(&mut lexer);

        println!("{:?}", res);

        assert!(!res.is_err())
    }

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
