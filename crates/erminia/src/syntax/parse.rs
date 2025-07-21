use crate::diagnostics::diagnostics::Location;
use crate::error::parser_error::{ParserError, ParserErrorInfo, ParserResult};
use crate::lexer::lex::Lexer;
use crate::lexer::token::TokenKind;
use crate::syntax::ast::{ObjectDecl, Program, Stmt};

// ==================================================================================== //
//  Utilities                                                                           //
// ==================================================================================== //

fn is_next_right_inclusive(tokens: &mut Lexer) -> ParserResult<bool> {
    let kind = tokens.peek().get_kind();

    match kind {
        TokenKind::RightPar => Ok(false),
        TokenKind::RightBracket => Ok(true),
        _ => {
            let position = tokens.peek().get_start();

            Err(ParserError::ExpectedRightInclusivity(ParserErrorInfo::new(
                Location::new(position),
                TokenKind::RightBracket,
                kind,
            )))
        }
    }
}

fn is_next_left_inclusive(tokens: &mut Lexer) -> ParserResult<bool> {
    let kind = tokens.peek().get_kind();

    match kind {
        TokenKind::LeftPar => Ok(false),
        TokenKind::LeftBracket => Ok(true),
        _ => {
            let position = tokens.peek().get_start();

            Err(ParserError::ExpectedLeftInclusivity(ParserErrorInfo::new(
                Location::new(position),
                TokenKind::LeftBracket,
                kind,
            )))
        }
    }
}

fn next_is_comma(tokens: &mut Lexer) -> bool {
    match tokens.peek().get_kind() {
        TokenKind::Comma => true,
        _ => false,
    }
}

fn next_is_expr(tokens: &mut Lexer) -> bool {
    match tokens.peek().get_kind() {
        TokenKind::Ident | TokenKind::Int => {

            true
        },
        _ => false
    }
}

fn next_is_stmt(tokens: &mut Lexer) -> bool {
    match tokens.peek().get_kind() {
        TokenKind::Ident | TokenKind::Object | TokenKind::LetKwd | TokenKind::ProblemExample => true,
        _ => false,
    }
}

fn match_next(tokens: &mut Lexer, matched: TokenKind) -> bool {
    tokens.peek().get_kind() == matched
}

// ==================================================================================== //
//  Consumers                                                                           //
// ==================================================================================== //

fn consume_data_type(tokens: &mut Lexer) -> ParserResult<()> {
    let kind = tokens.peek().get_kind();

    match kind {
        TokenKind::Object => {
            tokens.advance()?;
            Ok(())
        }
        _ => {
            let position = tokens.peek().get_start();

            Err(ParserError::ParserError(ParserErrorInfo::new(
                Location::new(position),
                TokenKind::Object,
                kind,
            )))
        }
    }
}

fn consume_int_const<'a>(tokens: &mut Lexer<'a>) -> ParserResult<&'a str> {
    let int_const = tokens.token;
    if int_const.get_kind() == TokenKind::Int {
        tokens.advance()?;
        Ok(int_const.text)
    } else {
        let position = tokens.peek().get_start();

        Err(ParserError::ExpectedIntegerConstError(
            ParserErrorInfo::new(
                Location::new(position),
                TokenKind::Int,
                int_const.get_kind(),
            ),
        ))
    }
}

fn consume_identifier<'a>(tokens: &mut Lexer<'a>) -> ParserResult<&'a str> {
    let id = tokens.token;
    match id.get_kind() {
        TokenKind::Ident => {
            tokens.advance()?;
            Ok(id.text)
        }
        _ => {
            let position = id.get_start();

            Err(ParserError::ExpectedIdentifierError(ParserErrorInfo::new(
                Location::new(position),
                TokenKind::Ident,
                id.get_kind(),
            )))
        }
    }
}

fn consume_keyword(tokens: &mut Lexer, expected: TokenKind) -> ParserResult<()> {
    let actual = tokens.peek().get_kind();
    if actual == expected {
        tokens.advance()?;
        Ok(())
    } else {
        let position = tokens.peek().get_start();

        Err(ParserError::ExpectedKeyWordError(ParserErrorInfo::new(
            Location::new(position),
            expected,
            actual,
        )))
    }
}

// ==================================================================================== //
// Parsers                                                                              //
// ==================================================================================== //

// <expr> ::= <object_call> | <id> | <int_const>
fn parse_expr(tokens: &mut Lexer) -> ParserResult<Stmt> {
    let kind = tokens.peek().get_kind();

    match kind {
        TokenKind::Ident => {
            let lookahead = tokens.lookahead()?;

            if matches!(lookahead.0, TokenKind::LeftPar) {
                let _object = parse_object_call(tokens)?;
            } else {
                let _var = consume_identifier(tokens)?;
            }

            Ok(Stmt::ObjectDecl(ObjectDecl::new()))
        },
        TokenKind::Int => {
            let _var = consume_int_const(tokens)?;
            Ok(Stmt::ObjectDecl(ObjectDecl::new()))
        },
        _ => {
            let position = tokens.peek().get_start();

            Err(ParserError::ParserError(ParserErrorInfo::new(
                Location::new(position),
                TokenKind::Ident,
                kind,
            )))
        }
    }
}

// <list_of_exprs> ::= <expr> ("," <expr>)*
fn parse_list_of_exprs(tokens: &mut Lexer) -> ParserResult<()> {

    while next_is_expr(tokens) {
        let _expr = parse_expr(tokens);
        
        let next = tokens.peek().get_kind();

        if matches!(next, TokenKind::Comma) {
            consume_keyword(tokens, TokenKind::Comma)?;
        }
    }
    Ok(())
}

// <func_call> ::= <id> "(" [<list_of_exprs>] ")" ";"
fn parse_func_call(tokens: &mut Lexer) -> ParserResult<()> {
    let _id = consume_identifier(tokens)?;

    consume_keyword(tokens, TokenKind::LeftPar)?;

    let _exprs = parse_list_of_exprs(tokens)?;

    consume_keyword(tokens, TokenKind::RightPar)?;

    consume_keyword(tokens, TokenKind::SemiColon)?;

    Ok(())
}

// <inner_stmt> ::= <object_def> | <var_def> | <func_call>
fn parse_inner_stmt(tokens: &mut Lexer) -> ParserResult<()> {
    let kind = tokens.peek().get_kind();

    match kind {
        TokenKind::Object => {
            let _object = parse_var_def(tokens)?;
            Ok(())
        }
        TokenKind::LetKwd => {
            let _var = parse_var_def(tokens)?;
            Ok(())
        }
        TokenKind::Ident => {
            let _func = parse_func_call(tokens)?;
            Ok(())
        }
        _ => {
            let position = tokens.peek().get_start();

            Err(ParserError::ParserError(ParserErrorInfo::new(
                Location::new(position),
                TokenKind::ObjectShape,
                kind,
            )))
        }
    }
}

// <inner_stmt_list> ::= (<inner_stmt>)*
fn parse_inner_stmt_list(tokens: &mut Lexer) -> ParserResult<()> {
    while next_is_stmt(tokens) {
        let _stmt = parse_inner_stmt(tokens)?;
    }
    Ok(())
}

// <inner_compound_stmt> ::= "{" [<inner_stmt_list>] "}"
fn parse_inner_compound_stmt(tokens: &mut Lexer) -> ParserResult<()> {
    consume_keyword(tokens, TokenKind::LeftBrace)?;
    let _stmts = parse_inner_stmt_list(tokens)?;
    consume_keyword(tokens, TokenKind::RightBrace)?;
    Ok(())
}

// <var_def> ::= "let" <id> ":" <data_type> "=" <object_call> ";"
fn parse_var_def(tokens: &mut Lexer) -> ParserResult<()> {
    let _data_type /* default of ToBeInfered */;
    consume_keyword(tokens, TokenKind::LetKwd)?;

    let _id = consume_identifier(tokens)?;

    if match_next(tokens, TokenKind::Colon) {
        consume_keyword(tokens, TokenKind::Colon)?;

        // change here if it's explicit about data type
        _data_type = consume_data_type(tokens)?;
    }

    consume_keyword(tokens, TokenKind::Equals)?;

    let _object = parse_object_call(tokens)?;

    consume_keyword(tokens, TokenKind::SemiColon)?;

    Ok(())
}

// <range> ::= ("[" | "(") <int_const> ".." <int_const> ("]" | ")")
fn parse_range(tokens: &mut Lexer) -> ParserResult<()> {
    let is_left_inclusive = is_next_left_inclusive(tokens)?;

    if is_left_inclusive {
        consume_keyword(tokens, TokenKind::LeftBracket)?;
    } else {
        consume_keyword(tokens, TokenKind::LeftPar)?;
    }

    let _left = consume_int_const(tokens)?;
    consume_keyword(tokens, TokenKind::Range)?;
    let _right = consume_int_const(tokens)?;
    let is_right_inclusive = is_next_right_inclusive(tokens)?;

    if is_right_inclusive {
        consume_keyword(tokens, TokenKind::RightBracket)?;
    } else {
        consume_keyword(tokens, TokenKind::RightPar)?;
    }
    // return Range object
    Ok(())
}

// <shape_tuple_iter> ::= <id> "<-" <range>
fn parse_shape_tuple_iter(tokens: &mut Lexer) -> ParserResult<()> {
    let _coord = consume_identifier(tokens)?;
    consume_keyword(tokens, TokenKind::LeftArrow)?;
    let _range = parse_range(tokens)?;
    Ok(())
}

// <shape_tuple_iter_pair> ::= <shape_tuple_iter> ("," <shape_tuple_iter>)
fn parse_shape_tuple_iter_pair(tokens: &mut Lexer) -> ParserResult<()> {
    let _first_tuple_iter = parse_shape_tuple_iter(tokens)?;
    if next_is_comma(tokens) {
        consume_keyword(tokens, TokenKind::Comma)?;
        let _second_tuple_iter = parse_shape_tuple_iter(tokens)?;
    }
    Ok(())
}

// <shape_tuple_compr> ::= <shape_tuple> "|" <shape_tuple_iter_pair>
fn parse_shape_tuple_compr(tokens: &mut Lexer) -> ParserResult<()> {
    let _tuple = parse_shape_tuple_generic(tokens)?;
    consume_keyword(tokens, TokenKind::Pipe)?;
    // we may need to include _tuple here to assign coordinates correctly
    let _tuple_iter_pair = parse_shape_tuple_iter_pair(tokens)?;
    Ok(())
}

// <object_call> ::= <id> <shape_tuple>
fn parse_object_call(tokens: &mut Lexer) -> ParserResult<()> {
    let _id = consume_identifier(tokens)?;
    match tokens.peek().get_kind() {
        TokenKind::LeftPar => {
            let _tuple = parse_shape_tuple(tokens)?;
        }
        _ => (),
    }
    // Ok(Stmt::ObjectCall(ObjectCall::default(id)))
    // Ok(Stmt::ObjectCall(ObjectCall::new(id, tuple)))
    Ok(())
}

// <shape_tuple_generic> ::= "(" (<int_const> | <id>) "," (<int_const> | <id>) ")"
fn parse_shape_tuple_generic(tokens: &mut Lexer) -> ParserResult<()> {
    consume_keyword(tokens, TokenKind::LeftPar)?;

    if match_next(tokens, TokenKind::Int) {
        let _left = consume_int_const(tokens)?;
    } else if match_next(tokens, TokenKind::Ident) {
        let _left = consume_identifier(tokens)?;
    }

    // <inner_stmt> ::= <object_def>
    consume_keyword(tokens, TokenKind::Comma)?;

    if match_next(tokens, TokenKind::Int) {
        let _right = consume_int_const(tokens)?;
    } else if match_next(tokens, TokenKind::Ident) {
        let _right = consume_identifier(tokens)?;
    }

    consume_keyword(tokens, TokenKind::RightPar)?;

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
    let kind = tokens.peek().get_kind();

    match kind {
        TokenKind::LeftPar => {
            let lookahead = tokens.lookahead_by(4)?;

            if matches!(lookahead, TokenKind::Pipe) {
                let _shape = parse_shape_tuple_compr(tokens)?;
                return Ok(Stmt::ObjectDecl(ObjectDecl::new()));
            }

            let _shape = parse_shape_tuple(tokens)?;
            return Ok(Stmt::ObjectDecl(ObjectDecl::new()));
            // Ok(Expr::Shape(Shape::new()))
        }
        TokenKind::Ident => {
            let _shape = parse_object_call(tokens)?;
            Ok(Stmt::ObjectDecl(ObjectDecl::new()))
            // Ok(Expr::Shape(Shape::new()))
        }
        _ => {
            let position = tokens.peek().get_start();

            Err(ParserError::ParserError(ParserErrorInfo::new(
                Location::new(position),
                TokenKind::LeftPar,
                kind,
            )))
        }
    }
}

// <object_color> ::= "color" ":" <int_const>
fn parse_object_color<'a>(tokens: &mut Lexer<'a>) -> ParserResult<()> {
    consume_keyword(tokens, TokenKind::ObjectColor)?;
    consume_keyword(tokens, TokenKind::Colon)?;
    let _int_const = consume_int_const(tokens)?;
    Ok(())
}

// <list_of_shapes> ::= "[" <shape> ("," <shape>)* "]"
fn parse_list_of_shapes(tokens: &mut Lexer) -> ParserResult<()> {
    let mut _shapes: Vec<Stmt> = vec![];
    consume_keyword(tokens, TokenKind::LeftBracket)?;
    let shape = parse_shape(tokens);
    match shape {
        Ok(sh) => _shapes.push(sh),
        _ => (),
    }
    while next_is_comma(tokens) {
        consume_keyword(tokens, TokenKind::Comma)?;
        let shape = parse_shape(tokens)?;
        _shapes.push(shape);
    }
    consume_keyword(tokens, TokenKind::RightBracket)?;
    Ok(())
}

// <object_shape> ::= "shape" ":" <list_of_shapes>
fn parse_object_shape(tokens: &mut Lexer) -> ParserResult<()> {
    consume_keyword(tokens, TokenKind::ObjectShape)?;
    consume_keyword(tokens, TokenKind::Colon)?;
    let _shapes = parse_list_of_shapes(tokens)?;
    Ok(())
}

// <object_desc> ::= <object_shape> "," <object_color> | <object_color> "," <object_shape>
fn parse_object_desc(tokens: &mut Lexer) -> ParserResult<()> {
    let kind = tokens.peek().get_kind();

    match kind {
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
        _ => {
            let position = tokens.peek().get_start();

            Err(ParserError::ParserError(ParserErrorInfo::new(
                Location::new(position),
                TokenKind::ObjectShape,
                kind,
            )))
        }
    }
}

// <example_def> ::= "example" <id> <inner_compound_stmt>
fn parse_problem_example(tokens: &mut Lexer) -> ParserResult<()> {
    consume_keyword(tokens, TokenKind::ProblemExample)?;
    let _id = consume_identifier(tokens)?;
    let _desc = parse_inner_compound_stmt(tokens)?;
    Ok(())
}

// <stmt> ::= <object_def> | <example_def> | <var_def>
fn parse_stmt(tokens: &mut Lexer) -> ParserResult<()> {
    let kind = tokens.peek().get_kind();

    match kind {
        TokenKind::Object => {
            let _object = parse_object_decl(tokens)?;
            // Ok(Stmt::ObjectDecl(ObjectDecl::new(id, desc)))
            Ok(())
        }
        TokenKind::ProblemExample => {
            let _example = parse_problem_example(tokens)?;
            // Ok(Stmt::ExampleDecl(ExampleDecl::new(id, desc)))
            Ok(())
        }
        TokenKind::LetKwd => {
            let _var = parse_var_def(tokens)?;
            // Ok(Stmt::VarDef(VarDef::new(type, id, expr)))
            Ok(())
        }
        _ => {
            let position = tokens.peek().get_start();

            Err(ParserError::ExpectedKeyWordError(ParserErrorInfo::new(
                Location::new(position),
                TokenKind::Object,
                kind,
            )))
        }
    }
}

// <stmts_list> ::= (<stmt>)*
fn parse_stmt_list(tokens: &mut Lexer) -> ParserResult<() /* Vec<Stmt> */> {
    // let mut stmts: Vec<Stmt> = vec![];
    while next_is_stmt(tokens) {
        let _stmt = parse_stmt(tokens)?;
        // stmts.push(stmt);
    }
    // Ok(stmts)
    Ok(())
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
fn parse_compound_stmt(tokens: &mut Lexer) -> ParserResult<()> {
    consume_keyword(tokens, TokenKind::LeftBrace)?;
    let _stmts = parse_stmt_list(tokens)?;
    consume_keyword(tokens, TokenKind::RightBrace)?;
    Ok(_stmts)
}

// <problem_declaration> ::= "def" <id> "(" <int_const> ")" <compound_stmt>
fn parse_problem_decl(tokens: &mut Lexer) -> ParserResult<Program> {
    consume_keyword(tokens, TokenKind::ProblemDef)?;
    let id = consume_identifier(tokens)?;
    consume_keyword(tokens, TokenKind::LeftPar)?;
    let int_const = consume_int_const(tokens)?;
    consume_keyword(tokens, TokenKind::RightPar)?;
    let _stmts = parse_compound_stmt(tokens)?;
    let program = Program::new(id.to_string(), int_const.parse::<i32>().unwrap(), _stmts);
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

    fn check_no_err<F>(text: &str, f: F)
    where
        F: FnOnce(&mut Lexer) -> ParserResult<()>,
    {
        let mut tokens = Lexer::new(&text);

        let _ = tokens.advance();

        let res = f(&mut tokens);

        if res.is_err() {
            let mut lexer = Lexer::new(&text);

            let tokens = lexer.lex_with_separate_pass();

            println!("{}", text);
            println!("{:#?}", tokens);
            println!("{:?}", res);
        }

        assert!(!res.is_err())
    }

    #[test]
    fn test_parse_object_decl() {
        let text = "object HA { shape: [(0,1), (0,2)], color: 1 };";

        check_no_err(text, parse_object_decl)
    }

    #[test]
    fn test_parse_list_of_shapes() {
        let text = "[(0,1), (0,2), obj, obj(1,1), (x,y) | x <- [0..1], y <- [0..2]]";

        check_no_err(text, parse_list_of_shapes)
    }

    #[test]
    fn test_parse_object_decl2() {
        let text =
            "object HA { shape: [(0,1), (0,2), (x,y) | x <- [0..1], y <- [0..2]], color: 1 };";

        check_no_err(text, parse_object_decl)
    }

    #[test]
    fn test_parse_object_compound_desc() {
        let text = "{ shape : [(0,1), (0,2)], color : 1 }";

        check_no_err(text, parse_object_compound_desc)
    }

    #[test]
    fn test_parse_object_desc() {
        let text = "shape : [(0,1), (0,2)], color : 1";

        check_no_err(text, parse_object_desc)
    }

    #[test]
    fn test_parse_shape() {
        let text = "shape : [(0,1), (0,2)]";

        check_no_err(text, parse_object_shape)
    }

    #[test]
    fn test_parse_color() {
        let text = "color : 1";

        check_no_err(text, parse_object_color)
    }

    #[test]
    fn test_parse_shape_tuple_compr() {
        let text = "(x,y) | x <- [0..1], y <- [0..1]";

        check_no_err(text, parse_shape_tuple_compr)
    }

    #[test]
    fn test_parse_var_def() {
        let text = "let x: object = HA(0,1);";

        check_no_err(text, parse_var_def)
    }

    #[test]
    fn test_parse_var_def_default_object() {
        let text = "let x: object = HA;";

        check_no_err(text, parse_var_def)
    }

    #[test]
    fn test_parse_var_def_default_object_no_type() {
        let text = "let x = HA;";

        check_no_err(text, parse_var_def)
    }

    #[test]
    fn test_parse_example_decl() {
        let text = "example hello {

            draw(1, foo(0,1), a);

        }";
            // let a = baz(0,0);

        check_no_err(text, parse_stmt)
    }

    // #[test]
    // fn test_range() {
    //     let text = "object Shape { shape : [(0,1), (1,1)], color: 1 };";
    //
    //     let mut lexer = Lexer::new(&text);
    //
    //     let _ = lexer.advance();
    //
    //     let res = parse_object_decl(&mut lexer);
    //
    //     println!("{:?}", res);
    //
    //     assert!(!res.is_err())
    // }
}
