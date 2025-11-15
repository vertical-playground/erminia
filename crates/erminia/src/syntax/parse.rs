use crate::ast::ast::BoxAST;
use crate::ast::expr::*;
use crate::ast::stmt::*;
use crate::diagnostics::location::*;
use crate::error::parser_error::{ParserError, ParserErrorInfo, ParserResult};
use crate::lexer::lex::Lexer;
use crate::lexer::token::TokenKind;
use crate::types::ErminiaType;

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
    matches!(tokens.peek().get_kind(), TokenKind::Comma)
}

fn next_is_expr(tokens: &mut Lexer) -> bool {
    matches!(tokens.peek().get_kind(), TokenKind::Ident | TokenKind::Int)
}

fn next_is_stmt(tokens: &mut Lexer) -> bool {
    matches!(
        tokens.peek().get_kind(),
        TokenKind::Ident
            | TokenKind::Object
            | TokenKind::LetKwd
            | TokenKind::ProblemExample
            | TokenKind::ProblemSolution
    )
}

fn match_next(tokens: &mut Lexer, matched: TokenKind) -> bool {
    tokens.peek().get_kind() == matched
}

// ==================================================================================== //
//  Consumers                                                                           //
// ==================================================================================== //

// TODO: handle tuple & list types
fn consume_data_type(tokens: &mut Lexer) -> ParserResult<ErminiaType> {
    let kind = tokens.peek().get_kind();
    // TODO: Map TokenKind to ErminiaType
    match kind {
        TokenKind::Object => {
            tokens.advance()?;
            Ok(ErminiaType::Object)
        }
        TokenKind::Int => {
            tokens.advance()?;
            Ok(ErminiaType::Int)
        }
        TokenKind::String => {
            tokens.advance()?;
            Ok(ErminiaType::String)
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

fn consume_int_const<'a>(tokens: &mut Lexer<'a>) -> ParserResult<i32> {
    let int_const = tokens.token;
    if int_const.get_kind() == TokenKind::Int {
        tokens.advance()?;
        Ok(int_const.text.parse::<i32>().unwrap())
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
fn parse_expr(tokens: &mut Lexer) -> ParserResult<BoxAST> {
    let kind = tokens.peek().get_kind();
    let stmt: BoxAST;

    match kind {
        TokenKind::Ident => {
            let lookahead = tokens.lookahead()?;

            if matches!(lookahead.0, TokenKind::LeftPar) {
                stmt = parse_object_call(tokens)?;
            } else {
                let id = consume_identifier(tokens)?;
                stmt = RValue::boxed_id(id.to_string());
            }

            Ok(stmt)
        }
        TokenKind::Int => Ok(RValue::boxed_int(consume_int_const(tokens)?)),
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
fn parse_list_of_exprs(tokens: &mut Lexer) -> ParserResult<Vec<BoxAST>> {
    let mut exprs: Vec<BoxAST> = vec![];

    while next_is_expr(tokens) {
        let expr = parse_expr(tokens)?;

        exprs.push(expr);

        let next = tokens.peek().get_kind();

        if matches!(next, TokenKind::Comma) {
            consume_keyword(tokens, TokenKind::Comma)?;
        }
    }

    Ok(exprs)
}

// <func_call> ::= <id> "(" [<list_of_exprs>] ")" ";"
fn parse_func_call(tokens: &mut Lexer) -> ParserResult<BoxAST> {
    let start = tokens.get_position();
    let id = consume_identifier(tokens)?;

    consume_keyword(tokens, TokenKind::LeftPar)?;

    let exprs = parse_list_of_exprs(tokens)?;

    consume_keyword(tokens, TokenKind::RightPar)?;

    consume_keyword(tokens, TokenKind::SemiColon)?;
    let end = tokens.get_position();
    let span = Span::new(start, end);

    let func = FuncCall::boxed(id.to_string(), exprs, span);

    Ok(func)
}

// <inner_stmt> ::= <object_decl> | <var_def> | <func_call>
fn parse_inner_stmt(tokens: &mut Lexer) -> ParserResult<BoxAST> {
    let kind = tokens.peek().get_kind();

    match kind {
        TokenKind::Object => {
            let object = parse_object_decl(tokens)?;
            Ok(object)
        }
        TokenKind::LetKwd => {
            let var = parse_var_def(tokens)?;
            Ok(var)
        }
        TokenKind::Ident => {
            let func = parse_func_call(tokens)?;
            Ok(func)
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
fn parse_inner_stmt_list(tokens: &mut Lexer) -> ParserResult<Vec<BoxAST>> {
    let mut stmts: Vec<BoxAST> = vec![];
    while next_is_stmt(tokens) {
        let stmt = parse_inner_stmt(tokens)?;
        stmts.push(stmt);
    }
    Ok(stmts)
}

// <inner_compound_stmt> ::= "{" [<inner_stmt_list>] "}"
fn parse_inner_compound_stmt(tokens: &mut Lexer) -> ParserResult<Vec<BoxAST>> {
    consume_keyword(tokens, TokenKind::LeftBrace)?;
    let stmts = parse_inner_stmt_list(tokens)?;
    consume_keyword(tokens, TokenKind::RightBrace)?;
    Ok(stmts)
}

// TODO: handle type inference
// <var_def> ::= "let" <id> ":" <data_type> "=" <expr> ";"
fn parse_var_def(tokens: &mut Lexer) -> ParserResult<BoxAST> {
    let start = tokens.get_position();
    let mut data_type: ErminiaType = ErminiaType::default();
    consume_keyword(tokens, TokenKind::LetKwd)?;

    let id = consume_identifier(tokens)?;

    if match_next(tokens, TokenKind::Colon) {
        consume_keyword(tokens, TokenKind::Colon)?;

        // change here if it's explicit about data type
        data_type = consume_data_type(tokens)?;
    }

    consume_keyword(tokens, TokenKind::Equals)?;

    let expr = parse_expr(tokens)?;

    consume_keyword(tokens, TokenKind::SemiColon)?;
    let end = tokens.get_position();
    let span = Span::new(start, end);

    // TODO
    let var_def = VarDef::boxed(id.to_string(), data_type, expr, span);

    Ok(var_def)
}

// <range> ::= ("[" | "(") <int_const> ".." <int_const> ("]" | ")")
fn parse_range(tokens: &mut Lexer) -> ParserResult<BoxAST> {
    let start = tokens.get_position();
    let is_left_inclusive = is_next_left_inclusive(tokens)?;

    if is_left_inclusive {
        consume_keyword(tokens, TokenKind::LeftBracket)?;
    } else {
        consume_keyword(tokens, TokenKind::LeftPar)?;
    }

    let left = consume_int_const(tokens)?;
    consume_keyword(tokens, TokenKind::Range)?;
    let right = consume_int_const(tokens)?;
    let is_right_inclusive = is_next_right_inclusive(tokens)?;

    if is_right_inclusive {
        consume_keyword(tokens, TokenKind::RightBracket)?;
    } else {
        consume_keyword(tokens, TokenKind::RightPar)?;
    }
    let end = tokens.get_position();
    let span = Span::new(start, end);

    Ok(Range::boxed(
        is_left_inclusive,
        is_right_inclusive,
        left,
        right,
        span,
    ))
}

// <shape_tuple_iter> ::= <id> "<-" <range>
fn parse_shape_tuple_iter(tokens: &mut Lexer) -> ParserResult<BoxAST> {
    let start = tokens.get_position();
    let coord = consume_identifier(tokens)?;
    consume_keyword(tokens, TokenKind::LeftArrow)?;
    let range = parse_range(tokens)?;
    let end = tokens.get_position();
    let span = Span::new(start, end);

    Ok(TupleIterator::boxed(coord.to_string(), range, span))
}

// <shape_tuple_iter_pair> ::= <shape_tuple_iter> ("," <shape_tuple_iter>)
fn parse_shape_tuple_iter_pair(tokens: &mut Lexer) -> ParserResult<Vec<BoxAST>> {
    let mut pairs: Vec<BoxAST> = vec![];

    let first_tuple_iter = parse_shape_tuple_iter(tokens)?;
    pairs.push(first_tuple_iter);

    if next_is_comma(tokens) {
        consume_keyword(tokens, TokenKind::Comma)?;
        let second_tuple_iter = parse_shape_tuple_iter(tokens)?;
        pairs.push(second_tuple_iter);
    }

    Ok(pairs)
}

// <shape_tuple_compr> ::= <shape_tuple> "|" <shape_tuple_iter_pair>
fn parse_shape_tuple_compr(tokens: &mut Lexer) -> ParserResult<BoxAST> {
    let start = tokens.get_position();
    let tuple = parse_shape_tuple_generic(tokens)?;
    consume_keyword(tokens, TokenKind::Pipe)?;
    let iter_pair = parse_shape_tuple_iter_pair(tokens)?;
    let end = tokens.get_position();
    let span = Span::new(start, end);

    Ok(TupleComprehension::boxed(tuple, iter_pair, span))
}

// <object_call> ::= <id> <shape_tuple>
fn parse_object_call(tokens: &mut Lexer) -> ParserResult<BoxAST> {
    let start = tokens.get_position();
    let id = consume_identifier(tokens)?;
    match tokens.peek().get_kind() {
        TokenKind::LeftPar => {
            let tuple = parse_shape_tuple(tokens)?;
            let end = tokens.get_position();
            let span = Span::new(start, end);

            let object = ObjectCall::boxed(id.to_string(), Some(tuple), span);
            Ok(object)
        }
        _ => {
            let end = tokens.get_position();
            let span = Span::new(start, end);

            let object = ObjectCall::boxed(id.to_string(), None, span);
            Ok(object)
        }
    }
}

// <shape_tuple_generic> ::= "(" (<int_const> | <id>) "," (<int_const> | <id>) ")"
fn parse_shape_tuple_generic(tokens: &mut Lexer) -> ParserResult<BoxAST> {
    let start = tokens.get_position();
    consume_keyword(tokens, TokenKind::LeftPar)?;

    let mut left: BoxAST = GenericTupleOption::boxed_none();
    let mut right: BoxAST = GenericTupleOption::boxed_none();

    if match_next(tokens, TokenKind::Int) {
        let int_const = consume_int_const(tokens)?;
        left = GenericTupleOption::boxed_int(int_const);
    } else if match_next(tokens, TokenKind::Ident) {
        let id = consume_identifier(tokens)?;
        left = GenericTupleOption::boxed_id(id.to_string());
    }

    // <inner_stmt> ::= <object_decl>
    consume_keyword(tokens, TokenKind::Comma)?;

    if match_next(tokens, TokenKind::Int) {
        let int_const = consume_int_const(tokens)?;
        right = GenericTupleOption::boxed_int(int_const);
    } else if match_next(tokens, TokenKind::Ident) {
        let id = consume_identifier(tokens)?;
        right = GenericTupleOption::boxed_id(id.to_string());
    }

    consume_keyword(tokens, TokenKind::RightPar)?;
    let end = tokens.get_position();
    let span = Span::new(start, end);

    Ok(GenericTuple::boxed(left, right, span))
}

// <shape_tuple> ::= "(" <int_const> "," <int_const> ")"
fn parse_shape_tuple(tokens: &mut Lexer) -> ParserResult<BoxAST> {
    let span_start = tokens.get_position();
    consume_keyword(tokens, TokenKind::LeftPar)?;

    let left = consume_int_const(tokens)?;

    consume_keyword(tokens, TokenKind::Comma)?;

    let right = consume_int_const(tokens)?;

    consume_keyword(tokens, TokenKind::RightPar)?;
    let span_end = tokens.get_position();
    let span = Span::new(span_start, span_end);

    Ok(Tuple::boxed(left, right, span))
}

// <shape> ::= <shape_tuple> | <shape_tuple_compr> | <object_call> | <id>
fn parse_shape(tokens: &mut Lexer) -> ParserResult<BoxAST> {
    let kind = tokens.peek().get_kind();

    match kind {
        TokenKind::LeftPar => {
            let lookahead = tokens.lookahead_by(4)?;

            if matches!(lookahead, TokenKind::Pipe) {
                let compr = parse_shape_tuple_compr(tokens)?;
                return Ok(compr);
            }

            let tuple = parse_shape_tuple(tokens)?;
            Ok(tuple)
        }
        TokenKind::Ident => {
            let object = parse_object_call(tokens)?;
            Ok(object)
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
fn parse_object_color(tokens: &mut Lexer) -> ParserResult<BoxAST> {
    let start = tokens.get_position();
    consume_keyword(tokens, TokenKind::ObjectColor)?;
    consume_keyword(tokens, TokenKind::Colon)?;
    let int_const = consume_int_const(tokens)?;
    let end = tokens.get_position();
    let span = Span::new(start, end);

    let color = ObjectColor::boxed(int_const, span);
    Ok(color)
}

// <list_of_shapes> ::= "[" <shape> ("," <shape>)* "]"
fn parse_list_of_shapes(tokens: &mut Lexer) -> ParserResult<Vec<BoxAST>> {
    let mut shapes: Vec<BoxAST> = vec![];
    consume_keyword(tokens, TokenKind::LeftBracket)?;
    let shape = parse_shape(tokens);
    if let Ok(sh) = shape {
        shapes.push(sh);
    }
    while next_is_comma(tokens) {
        consume_keyword(tokens, TokenKind::Comma)?;
        let shape = parse_shape(tokens)?;
        shapes.push(shape);
    }
    consume_keyword(tokens, TokenKind::RightBracket)?;
    Ok(shapes)
}

// <object_shape> ::= "shape" ":" <list_of_shapes>
fn parse_object_shape(tokens: &mut Lexer) -> ParserResult<BoxAST> {
    let start = tokens.get_position();
    consume_keyword(tokens, TokenKind::ObjectShape)?;
    consume_keyword(tokens, TokenKind::Colon)?;
    let shapes = parse_list_of_shapes(tokens)?;
    let end = tokens.get_position();
    let span = Span::new(start, end);

    let object_shape = ObjectShape::boxed(shapes, span);
    Ok(object_shape)
}

// <object_desc> ::= <object_shape> "," <object_color> | <object_color> "," <object_shape>
fn parse_object_desc(tokens: &mut Lexer) -> ParserResult<BoxAST> {
    let kind = tokens.peek().get_kind();

    let start = tokens.get_position();

    match kind {
        TokenKind::ObjectShape => {
            let shape = parse_object_shape(tokens)?;
            consume_keyword(tokens, TokenKind::Comma)?;
            let color = parse_object_color(tokens)?;
            let end = tokens.get_position();
            let span = Span::new(start, end);

            let object_desc = ObjectDesc::boxed(shape, color, span);
            Ok(object_desc)
        }
        TokenKind::ObjectColor => {
            let color = parse_object_color(tokens)?;
            consume_keyword(tokens, TokenKind::Comma)?;
            let shape = parse_object_shape(tokens)?;
            let end = tokens.get_position();
            let span = Span::new(start, end);

            let object_desc = ObjectDesc::boxed(shape, color, span);
            Ok(object_desc)
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

// <example_decl> ::= "example" <id> <inner_compound_stmt>
fn parse_problem_example(tokens: &mut Lexer) -> ParserResult<BoxAST> {
    let start = tokens.get_position();
    consume_keyword(tokens, TokenKind::ProblemExample)?;
    let id = consume_identifier(tokens)?;
    let stmts = parse_inner_compound_stmt(tokens)?;
    let end = tokens.get_position();
    let span = Span::new(start, end);

    let example = ProblemExample::boxed(id.to_string(), stmts, span);
    Ok(example)
}

// <problem_solution> ::= "solution" <id> <inner_compound_stmt>
fn parse_problem_solution(tokens: &mut Lexer) -> ParserResult<BoxAST> {
    let start = tokens.get_position();

    consume_keyword(tokens, TokenKind::ProblemSolution)?;
    let id = consume_identifier(tokens)?;
    let stmts = parse_inner_compound_stmt(tokens)?;
    let end = tokens.get_position();
    let span = Span::new(start, end);

    let solution = ProblemSolution::boxed(id.to_string(), stmts, span);
    Ok(solution)
}

// <problem_input> ::= "input" <id> <tuple> <inner_compound_stmt>
fn parse_problem_input(tokens: &mut Lexer) -> ParserResult<BoxAST> {
    let start = tokens.get_position();

    consume_keyword(tokens, TokenKind::ProblemInput)?;
    let id = consume_identifier(tokens)?;
    let stmts = parse_inner_compound_stmt(tokens)?;

    let end = tokens.get_position();
    let span = Span::new(start, end);

    let input = ProblemInput::boxed(id.to_string(), stmts, span);
    Ok(input)
}

// <problem_output> ::= "output" <id> <tuple> <inner_compound_stmt>
fn parse_problem_output(tokens: &mut Lexer) -> ParserResult<BoxAST> {
    let start = tokens.get_position();
    consume_keyword(tokens, TokenKind::ProblemOutput)?;
    let id = consume_identifier(tokens)?;
    let stmts = parse_inner_compound_stmt(tokens)?;
    let end = tokens.get_position();
    let span = Span::new(start, end);

    let output = ProblemOutput::boxed(id.to_string(), stmts, span);
    Ok(output)
}

// <stmt> ::= <object_decl> | <example_decl> | <var_def>
fn parse_stmt(tokens: &mut Lexer) -> ParserResult<BoxAST> {
    let kind = tokens.peek().get_kind();

    match kind {
        TokenKind::Object => {
            let object = parse_object_decl(tokens)?;
            Ok(object)
        }
        TokenKind::ProblemExample => {
            let example = parse_problem_example(tokens)?;
            Ok(example)
        }
        TokenKind::ProblemSolution => {
            let solution = parse_problem_solution(tokens)?;
            Ok(solution)
        }
        TokenKind::ProblemInput => {
            let input = parse_problem_input(tokens)?;
            Ok(input)
        }
        TokenKind::ProblemOutput => {
            let output = parse_problem_output(tokens)?;
            Ok(output)
        }
        TokenKind::LetKwd => {
            let var = parse_var_def(tokens)?;
            Ok(var)
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
fn parse_stmt_list(tokens: &mut Lexer) -> ParserResult<Vec<BoxAST>> {
    let mut stmts: Vec<BoxAST> = vec![];

    while next_is_stmt(tokens) {
        let stmt = parse_stmt(tokens)?;
        stmts.push(stmt);
    }

    Ok(stmts)
}

// <object_compound_desc> ::= "{" <object_desc> "}"
fn parse_object_compound_desc(tokens: &mut Lexer) -> ParserResult<BoxAST> {
    consume_keyword(tokens, TokenKind::LeftBrace)?;
    let object_desc = parse_object_desc(tokens)?;
    consume_keyword(tokens, TokenKind::RightBrace)?;
    Ok(object_desc)
}

// <object_decl> ::= "object" <id> <object_compound_desc> ";"
fn parse_object_decl(tokens: &mut Lexer) -> ParserResult<BoxAST> {
    let start = tokens.get_position();
    consume_keyword(tokens, TokenKind::Object)?;
    let id = consume_identifier(tokens)?;
    let object_desc = parse_object_compound_desc(tokens)?;
    consume_keyword(tokens, TokenKind::SemiColon)?;
    let end = tokens.get_position();
    let span = Span::new(start, end);

    let object_decl = ObjectDecl::boxed(id.to_string(), object_desc, span);
    Ok(object_decl)
}

// <compound_stmt> ::= "{" [<stmt_list>] "}"
fn parse_compound_stmt(tokens: &mut Lexer) -> ParserResult<Vec<BoxAST>> {
    consume_keyword(tokens, TokenKind::LeftBrace)?;
    let stmts = parse_stmt_list(tokens)?;
    consume_keyword(tokens, TokenKind::RightBrace)?;
    Ok(stmts)
}

// <problem_declaration> ::= "def" <id> "(" <int_const> ")" <compound_stmt>
fn parse_problem_decl(tokens: &mut Lexer) -> ParserResult<BoxAST> {
    let start = tokens.get_position();
    consume_keyword(tokens, TokenKind::ProblemDef)?;
    let id = consume_identifier(tokens)?;
    consume_keyword(tokens, TokenKind::LeftPar)?;
    let int_const = consume_int_const(tokens)?;
    consume_keyword(tokens, TokenKind::RightPar)?;
    let stmts = parse_compound_stmt(tokens)?;
    let end = tokens.get_position();
    let span = Span::new(start, end);

    let program = Program::boxed(id.to_string(), int_const, stmts, span);

    Ok(program)
}

// <program> ::= <problem_declaration>
pub fn parse_program(tokens: &mut Lexer) -> ParserResult<BoxAST> {
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
        Parser { lexer }
    }

    pub fn parse(&mut self) -> ParserResult<BoxAST> {
        parse_program(&mut self.lexer)
    }
}

// ==================================================================================== //
// Parser Test Suite                                                                    //
// ==================================================================================== //

#[cfg(test)]
mod test {
    use super::*;

    fn check_no_err<F, T>(text: &str, parser: F)
    where
        F: FnOnce(&mut Lexer) -> ParserResult<T>,
        T: std::fmt::Debug,
    {
        let mut tokens = Lexer::new(text);

        let _ = tokens.advance();

        let res = parser(&mut tokens);

        if res.is_err() {
            let mut lexer = Lexer::new(text);

            let tokens = lexer.lex_with_separate_pass();

            println!("{}", text);
            println!("{:#?}", tokens);
            println!("{:?}", res);
        }

        assert!(res.is_ok())
    }

    fn check_type(text: &str, expected_type: ErminiaType) {
        let mut tokens = Lexer::new(text);

        let _ = tokens.advance();

        let _ = consume_keyword(&mut tokens, TokenKind::LetKwd);
        let _ = consume_identifier(&mut tokens);

        let actual_type = if match_next(&mut tokens, TokenKind::Colon) {
            let _ = consume_keyword(&mut tokens, TokenKind::Colon);
            consume_data_type(&mut tokens).unwrap()
        // TODO: Add logic for type inference
        } else {
            ErminiaType::default()
        };

        assert_eq!(actual_type, expected_type);
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
    fn test_parse_var_def_explicit_object_type() {
        let text = "let x: object = HA(0,1);";

        check_type(text, ErminiaType::Object);
    }

    #[test]
    fn test_parse_var_def_default_object() {
        let text = "let x: object = HA;";

        check_no_err(text, parse_var_def)
    }

    #[test]
    fn test_parse_var_def_default_explicit_object_type() {
        let text = "let x: object = HA;";

        check_type(text, ErminiaType::Object);
    }

    #[test]
    fn test_parse_var_def_default_object_no_type() {
        let text = "let x = HA;";

        check_no_err(text, parse_var_def)
    }

    // TODO: Include when inference logic is added
    // #[test]
    // fn test_parse_var_def_default_no_object_type() {
    //     let text = "let x = HA;";

    //     check_type(text, ErminiaType::Object);
    // }

    #[test]
    fn test_parse_example_decl() {
        let text = "example hello {

            draw(1, foo(0,1), a);

        }";

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
