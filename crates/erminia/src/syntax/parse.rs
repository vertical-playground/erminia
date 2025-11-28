use crate::ast::ast::BoxAST;
use crate::ast::expr::*;
use crate::ast::stmt::*;
use crate::diagnostics::location::*;
use crate::diagnostics::code::Code;
use crate::lexer::lex::Lexer;
use crate::lexer::token::TokenKind;
use crate::types::ErminiaType;
use crate::config::CompilerPass;

// ==================================================================================== //
//  Utilities                                                                           //
// ==================================================================================== //

fn is_next_right_inclusive<'a>(tokens: &mut Lexer) -> ErminiaType {
    let kind = tokens.peek().get_kind();

    match kind {
        TokenKind::RightPar => ErminiaType::Bool(false),
        TokenKind::RightBracket => ErminiaType::Bool(true),
        _ => {
            create_diagnostic(CompilerPass::Parser, tokens, Code::E000X);
            ErminiaType::Poisoned
        }
    }
}

fn is_next_left_inclusive<'a>(tokens: &mut Lexer) -> ErminiaType {
    let kind = tokens.peek().get_kind();

    match kind {
        TokenKind::LeftPar => ErminiaType::Bool(false),
        TokenKind::LeftBracket => ErminiaType::Bool(true),
        _ => {
            create_diagnostic(CompilerPass::Parser, tokens, Code::E000X);
            ErminiaType::Poisoned
        }
    }
}

fn next_is_comma<'a>(tokens: &mut Lexer) -> bool {
    matches!(tokens.peek().get_kind(), TokenKind::Comma)
}

fn next_is_expr<'a>(tokens: &mut Lexer) -> bool {
    matches!(tokens.peek().get_kind(), TokenKind::Ident | TokenKind::Int)
}

fn next_is_stmt<'a>(tokens: &mut Lexer) -> bool {
    matches!(
        tokens.peek().get_kind(),
        TokenKind::Ident
            | TokenKind::Object
            | TokenKind::LetKwd
            | TokenKind::ProblemExample
            | TokenKind::ProblemSolution
            | TokenKind::ProblemInput
            | TokenKind::ProblemOutput
    )
}

fn match_next(tokens: &mut Lexer, matched: TokenKind) -> bool {
    tokens.peek().get_kind() == matched
}

// ==================================================================================== //
//  Consumers                                                                           //
// ==================================================================================== //

// TODO: handle tuple & list types
fn consume_data_type<'a>(tokens: &mut Lexer) -> ErminiaType {
    let kind = tokens.peek().get_kind();
    // TODO: Map TokenKind to ErminiaType
    match kind {
        TokenKind::Object => {
            tokens.advance();
            ErminiaType::Object
        }
        TokenKind::Int => {
            tokens.advance();
            ErminiaType::Int
        }
        TokenKind::String => {
            tokens.advance();
            ErminiaType::String
        }
        _ => {
            create_diagnostic(CompilerPass::Parser, tokens, Code::E000X);
            ErminiaType::Poisoned
        }
    }
}

fn consume_int_const<'a>(tokens: &mut Lexer) -> ErminiaType {
    let int_const = tokens.token;
    if int_const.get_kind() == TokenKind::Int {
        tokens.advance();
        ErminiaType::Integer(int_const.text.parse::<i32>().unwrap())
    } else {
        create_diagnostic(CompilerPass::Parser, tokens, Code::E000X);
        ErminiaType::Poisoned
    }
}

fn consume_identifier<'a>(tokens: &mut Lexer) -> ErminiaType {
    let id = tokens.token;
    match id.get_kind() {
        TokenKind::Ident => {
            tokens.advance();
            ErminiaType::Ident(id.text.to_string())
        }
        _ => {
            create_diagnostic(CompilerPass::Parser, tokens, Code::E000X);
            ErminiaType::Poisoned
        }
    }
}

fn consume_keyword<'a>(tokens: &mut Lexer, expected: TokenKind) -> ErminiaType {
    let actual = tokens.peek().get_kind();
    if actual == expected {
        tokens.advance();
        ErminiaType::Void
    } else {
        create_diagnostic(CompilerPass::Parser, tokens, Code::E000X);
        ErminiaType::Poisoned
    }
}

// ==================================================================================== //
// Parsers                                                                              //
// ==================================================================================== //

// <expr> ::= <object_call> | <id> | <int_const>
fn parse_expr<'a>(tokens: &mut Lexer) -> BoxAST<'a> {
    let kind = tokens.peek().get_kind();
    let stmt: BoxAST;

    match kind {
        TokenKind::Ident => {
            let lookahead = tokens.lookahead();

            if matches!(lookahead.0, TokenKind::LeftPar) {
                stmt = parse_object_call(tokens);
            } else {
                let id = consume_identifier(tokens);
                stmt = RValue::boxed_id(id.to_string());
            }

            stmt
        }
        TokenKind::Int => RValue::boxed_int(
            consume_int_const(tokens)
                .to_int()
            ),
        _ => {
            create_diagnostic(CompilerPass::Parser, tokens, Code::E000X);
            stmt = parse_object_call(tokens);
            stmt
        }
    }
}

// <list_of_exprs> ::= <expr> ("," <expr>)*
fn parse_list_of_exprs<'a>(tokens: &mut Lexer) -> Vec<BoxAST<'a>> {
    let mut exprs: Vec<BoxAST> = vec![];

    while next_is_expr(tokens) {
        let expr = parse_expr(tokens);

        exprs.push(expr);

        let next = tokens.peek().get_kind();

        if matches!(next, TokenKind::Comma) {
            consume_keyword(tokens, TokenKind::Comma);
        }
    }

    exprs
}

// <func_call> ::= <id> "(" [<list_of_exprs>] ")" ";"
fn parse_func_call<'a>(tokens: &mut Lexer) -> BoxAST<'a> {
    let mut poisoned: bool = false;

    let start = tokens.get_position();
    let id = consume_identifier(tokens);

    if id.is_poisoned() {
        poisoned = true;
    }

    consume_keyword(tokens, TokenKind::LeftPar);

    let exprs = parse_list_of_exprs(tokens);

    consume_keyword(tokens, TokenKind::RightPar);

    consume_keyword(tokens, TokenKind::SemiColon);
    let end = tokens.get_position();
    let span = Span::new(start, end);

    let func = FuncCall::boxed(id.to_string(), exprs, span, poisoned);

    func
}

// <inner_stmt> ::= <object_decl> | <var_def> | <func_call>
fn parse_inner_stmt<'a>(tokens: &mut Lexer) -> BoxAST<'a> {
    let kind = tokens.peek().get_kind();

    match kind {
        TokenKind::Object => {
            let object = parse_object_decl(tokens);
            object
        }
        TokenKind::LetKwd => {
            let var = parse_var_def(tokens);
            var
        }
        TokenKind::Ident => {
            let func = parse_func_call(tokens);
            func
        }
        _ => {
            create_diagnostic(CompilerPass::Parser, tokens, Code::E000X);
            let object = parse_object_decl(tokens);
            object
        }
    }
}

// <inner_stmt_list> ::= (<inner_stmt>)*
fn parse_inner_stmt_list<'a>(tokens: &mut Lexer) -> Vec<BoxAST<'a>> {
    let mut stmts: Vec<BoxAST> = vec![];
    while next_is_stmt(tokens) {
        let stmt = parse_inner_stmt(tokens);
        stmts.push(stmt);
    }
    stmts
}

// <inner_compound_stmt> ::= "{" [<inner_stmt_list>] "}"
fn parse_inner_compound_stmt<'a>(tokens: &mut Lexer) -> Vec<BoxAST<'a>> {
    consume_keyword(tokens, TokenKind::LeftBrace);
    let stmts = parse_inner_stmt_list(tokens);
    consume_keyword(tokens, TokenKind::RightBrace);
    stmts
}

// TODO: handle type inference
// <var_def> ::= "let" <id> ":" <data_type> "=" <expr> ";"
fn parse_var_def<'a>(tokens: &mut Lexer) -> BoxAST<'a> {
    let mut poisoned: bool = false;

    let start = tokens.get_position();
    let mut data_type: ErminiaType = ErminiaType::default();

    if consume_keyword(tokens, TokenKind::LetKwd).is_poisoned() {
        poisoned = true;
    }

    let id = consume_identifier(tokens);

    if id.is_poisoned() {
        poisoned = true;
    }

    if match_next(tokens, TokenKind::Colon) {
        if consume_keyword(tokens, TokenKind::Colon).is_poisoned() {
            poisoned = true;
        }

        // change here if it's explicit about data type
        data_type = consume_data_type(tokens);

        if data_type.is_poisoned() {
            poisoned = true;
        }
    }

    if consume_keyword(tokens, TokenKind::Equals).is_poisoned() {
        poisoned = true;
    }

    let expr = parse_expr(tokens);

    if expr.is_err() {
        poisoned = true;
    }

    if consume_keyword(tokens, TokenKind::SemiColon).is_poisoned() {
        poisoned = true;
    }

    let end = tokens.get_position();
    let span = Span::new(start, end);

    // TODO
    let var_def = VarDef::boxed(id.to_string(), data_type, expr, span, poisoned);

    var_def
}

// <range> ::= ("[" | "(") <int_const> ".." <int_const> ("]" | ")")
fn parse_range<'a>(tokens: &mut Lexer) -> BoxAST<'a> {
    let mut poisoned: bool = false;

    let start = tokens.get_position();
    let is_left_inclusive = is_next_left_inclusive(tokens);

    if is_left_inclusive.is_poisoned() {
        poisoned = true;
    }

    let left = consume_int_const(tokens);

    if left.is_poisoned() {
        poisoned = true;
    }

    if consume_keyword(tokens, TokenKind::Range).is_poisoned() {
        poisoned = true;
    }

    let right = consume_int_const(tokens);

    if right.is_poisoned() {
        poisoned = true;
    }

    let is_right_inclusive = is_next_right_inclusive(tokens);

    if is_right_inclusive.is_poisoned() {
        poisoned = true;
    }

    let end = tokens.get_position();
    let span = Span::new(start, end);

    Range::boxed(
        is_left_inclusive.to_bool(),
        is_right_inclusive.to_bool(),
        left.to_int(),
        right.to_int(),
        span,
        poisoned,
    )
}

// <shape_tuple_iter> ::= <id> "<-" <range>
fn parse_shape_tuple_iter<'a>(tokens: &mut Lexer) -> BoxAST<'a> {
    let mut poisoned: bool = false;

    let start = tokens.get_position();
    let coord = consume_identifier(tokens);

    if coord.is_poisoned() {
        poisoned = true;
    }

    if consume_keyword(tokens, TokenKind::LeftArrow).is_poisoned() {
        poisoned = true;
    }

    let range = parse_range(tokens);

    if range.is_err() {
        poisoned = true;
    }

    let end = tokens.get_position();
    let span = Span::new(start, end);

    TupleIterator::boxed(coord.to_string(), range, span, poisoned)
}

// <shape_tuple_iter_pair> ::= <shape_tuple_iter> ("," <shape_tuple_iter>)
fn parse_shape_tuple_iter_pair<'a>(tokens: &mut Lexer) -> Vec<BoxAST<'a>> {
    let mut pairs: Vec<BoxAST> = vec![];

    let first_tuple_iter = parse_shape_tuple_iter(tokens);
    pairs.push(first_tuple_iter);

    if next_is_comma(tokens) {
        consume_keyword(tokens, TokenKind::Comma);
        let second_tuple_iter = parse_shape_tuple_iter(tokens);
        pairs.push(second_tuple_iter);
    }

    pairs
}

// <shape_tuple_compr> ::= <shape_tuple> "|" <shape_tuple_iter_pair>
fn parse_shape_tuple_compr<'a>(tokens: &mut Lexer) -> BoxAST<'a> {
    let mut poisoned: bool = false; 

    let start = tokens.get_position();
    let tuple = parse_shape_tuple_generic(tokens);

    if tuple.is_err() {
        poisoned = true;
    }

    if consume_keyword(tokens, TokenKind::Pipe).is_poisoned() {
        poisoned = true;
    }

    let iter_pair = parse_shape_tuple_iter_pair(tokens);

    if iter_pair.iter().any(|s| s.is_err()) {
        poisoned = true;
    }

    let end = tokens.get_position();
    let span = Span::new(start, end);

    TupleComprehension::boxed(tuple, iter_pair, span, poisoned)
}

// <object_call> ::= <id> <shape_tuple>
fn parse_object_call<'a>(tokens: &mut Lexer) -> BoxAST<'a> {
    let mut poisoned: bool = false;

    let start = tokens.get_position();
    let id = consume_identifier(tokens);

    if id.is_poisoned() {
        poisoned = true;
    }

    match tokens.peek().get_kind() {
        TokenKind::LeftPar => {
            let tuple = parse_shape_tuple(tokens);

            if tuple.is_err() {
                poisoned = true;
            }

            let end = tokens.get_position();
            let span = Span::new(start, end);

            let object = ObjectCall::boxed(id.to_string(), Some(tuple), span, poisoned);
            object
        }
        _ => {
            let end = tokens.get_position();
            let span = Span::new(start, end);

            let object = ObjectCall::boxed(id.to_string(), None, span, poisoned);
            object
        }
    }
}

// <shape_tuple_generic> ::= "(" (<int_const> | <id>) "," (<int_const> | <id>) ")"
fn parse_shape_tuple_generic<'a>(tokens: &mut Lexer) -> BoxAST<'a> {
    let mut poisoned: bool = false;

    let start = tokens.get_position();

    if consume_keyword(tokens, TokenKind::LeftPar).is_poisoned() {
        poisoned = true;
    }

    let mut left: BoxAST = GenericTupleOption::boxed_none();
    let mut right: BoxAST = GenericTupleOption::boxed_none();

    if match_next(tokens, TokenKind::Int) {
        let int_const = consume_int_const(tokens);

        if int_const.is_poisoned() {
            poisoned = true;
        }

        left = GenericTupleOption::boxed_int(int_const.to_int(), poisoned);
    } else if match_next(tokens, TokenKind::Ident) {
        let id = consume_identifier(tokens);

        if id.is_poisoned() {
            poisoned = true;
        }

        left = GenericTupleOption::boxed_id(id.to_string(), poisoned);
    }

    // <inner_stmt> ::= <object_decl>
    consume_keyword(tokens, TokenKind::Comma);

    if match_next(tokens, TokenKind::Int) {
        let int_const = consume_int_const(tokens);
        
        if int_const.is_poisoned() {
            poisoned = true;
        }

        right = GenericTupleOption::boxed_int(int_const.to_int(), poisoned);
    } else if match_next(tokens, TokenKind::Ident) {
        let id = consume_identifier(tokens);

        if id.is_poisoned() {
            poisoned = true;
        }

        right = GenericTupleOption::boxed_id(id.to_string(), poisoned);
    }

    consume_keyword(tokens, TokenKind::RightPar);
    let end = tokens.get_position();
    let span = Span::new(start, end);

    GenericTuple::boxed(left, right, span, poisoned)
}

// <shape_tuple> ::= "(" <int_const> "," <int_const> ")"
fn parse_shape_tuple<'a>(tokens: &mut Lexer) -> BoxAST<'a> {
    let mut poisoned: bool = false;

    let span_start = tokens.get_position();

    if consume_keyword(tokens, TokenKind::LeftPar).is_poisoned() {
        poisoned = true;
    }

    let left = consume_int_const(tokens);

    if left.is_poisoned() {
        poisoned = true;
    }

    if consume_keyword(tokens, TokenKind::Comma).is_poisoned() {
        poisoned = true;
    }

    let right = consume_int_const(tokens);

    if right.is_poisoned() {
        poisoned = true;
    }

    if consume_keyword(tokens, TokenKind::RightPar).is_poisoned() {
        poisoned = true;
    }

    let span_end = tokens.get_position();
    let span = Span::new(span_start, span_end);

    Tuple::boxed(left.to_int(), right.to_int(), span, poisoned)
}

// <shape> ::= <shape_tuple> | <shape_tuple_compr> | <object_call> | <id>
fn parse_shape<'a>(tokens: &mut Lexer) -> BoxAST<'a> {
    let kind = tokens.peek().get_kind();

    match kind {
        TokenKind::LeftPar => {
            let lookahead = tokens.lookahead_by(4);

            if matches!(lookahead, TokenKind::Pipe) {
                let compr = parse_shape_tuple_compr(tokens);
                return compr;
            }

            let tuple = parse_shape_tuple(tokens);
            tuple
        }
        TokenKind::Ident => {
            let object = parse_object_call(tokens);
            object
        }
        _ => {
            create_diagnostic(CompilerPass::Parser, tokens, Code::E000X);
            let lookahead = tokens.lookahead_by(4);

            if matches!(lookahead, TokenKind::Pipe) {
                let compr = parse_shape_tuple_compr(tokens);
                return compr;
            }

            let tuple = parse_shape_tuple(tokens);
            tuple
        }
    }
}

// <object_color> ::= "color" ":" <int_const>
fn parse_object_color<'a>(tokens: &mut Lexer) -> BoxAST<'a> {
    let mut poisoned: bool = false;

    let start = tokens.get_position();
    if consume_keyword(tokens, TokenKind::ObjectColor).is_poisoned() {
        poisoned = true;
    }

    if consume_keyword(tokens, TokenKind::Colon).is_poisoned() {
        poisoned = true;
    }

    let int_const = consume_int_const(tokens);

    if int_const.is_poisoned() {
        poisoned = true;
    }

    let end = tokens.get_position();
    let span = Span::new(start, end);

    let color = ObjectColor::boxed(int_const.to_int(), span, poisoned);
    color
}

// <list_of_shapes> ::= "[" <shape> ("," <shape>)* "]"
fn parse_list_of_shapes<'a>(tokens: &mut Lexer) -> Vec<BoxAST<'a>> {
    let mut shapes: Vec<BoxAST> = vec![];
    consume_keyword(tokens, TokenKind::LeftBracket);
    let shape = parse_shape(tokens);
    shapes.push(shape);
    while next_is_comma(tokens) {
        consume_keyword(tokens, TokenKind::Comma);
        let shape = parse_shape(tokens);
        shapes.push(shape);
    }
    consume_keyword(tokens, TokenKind::RightBracket);
    shapes
}

// <object_shape> ::= "shape" ":" <list_of_shapes>
fn parse_object_shape<'a>(tokens: &mut Lexer) -> BoxAST<'a> {
    let mut poisoned: bool = false;

    let start = tokens.get_position();
    if consume_keyword(tokens, TokenKind::ObjectShape).is_poisoned() {
        poisoned = true;
    }

    if consume_keyword(tokens, TokenKind::Colon).is_poisoned() {
        poisoned = true;
    }

    let shapes = parse_list_of_shapes(tokens);

    if shapes.iter().any(|s| s.is_err()) {
        poisoned = true;
    }

    let end = tokens.get_position();
    let span = Span::new(start, end);

    let object_shape = ObjectShape::boxed(shapes, span, poisoned);
    object_shape
}

// <object_desc> ::= <object_shape> "," <object_color> | <object_color> "," <object_shape>
fn parse_object_desc<'a>(tokens: &mut Lexer) -> BoxAST<'a> {
    let mut poisoned: bool = false;

    let kind = tokens.peek().get_kind();

    let start = tokens.get_position();

    match kind {
        TokenKind::ObjectShape => {
            let shape = parse_object_shape(tokens);
            consume_keyword(tokens, TokenKind::Comma);
            let color = parse_object_color(tokens);
            let end = tokens.get_position();
            let span = Span::new(start, end);

            let object_desc = ObjectDesc::boxed(shape, color, span, poisoned);
            object_desc
        }
        TokenKind::ObjectColor => {
            let color = parse_object_color(tokens);
            consume_keyword(tokens, TokenKind::Comma);
            let shape = parse_object_shape(tokens);
            let end = tokens.get_position();
            let span = Span::new(start, end);

            let object_desc = ObjectDesc::boxed(shape, color, span, poisoned);
            object_desc
        }
        _ => {
            poisoned = true;
            create_diagnostic(CompilerPass::Parser, tokens, Code::E000X);
            let shape = parse_object_shape(tokens);
            consume_keyword(tokens, TokenKind::Comma);
            let color = parse_object_color(tokens);
            let end = tokens.get_position();
            let span = Span::new(start, end);

            let object_desc = ObjectDesc::boxed(shape, color, span, poisoned);
            object_desc
        }
    }
}

// <example_decl> ::= "example" <id> <inner_compound_stmt>
fn parse_problem_example<'a>(tokens: &mut Lexer) -> BoxAST<'a> {
    let mut poisoned: bool = false;

    let start = tokens.get_position();

    if consume_keyword(tokens, TokenKind::ProblemExample).is_poisoned() {
        poisoned = true;
    }

    let id = consume_identifier(tokens);

    if id.is_poisoned() {
        poisoned = true;
    }

    let stmts = parse_inner_compound_stmt(tokens);

    if stmts.iter().any(|s| s.is_err()) {
        poisoned = true;
    }

    let end = tokens.get_position();
    let span = Span::new(start, end);

    let example = ProblemExample::boxed(id.to_string(), stmts, span, poisoned);
    example
}

// <problem_solution> ::= "solution" <id> <inner_compound_stmt>
fn parse_problem_solution<'a>(tokens: &mut Lexer) -> BoxAST<'a> {
    let mut poisoned: bool = false;

    let start = tokens.get_position();

    if consume_keyword(tokens, TokenKind::ProblemSolution).is_poisoned() {
        poisoned = true;
    }

    let id = consume_identifier(tokens);

    if id.is_poisoned() {
        poisoned = true;
    }

    let stmts = parse_inner_compound_stmt(tokens);

    if stmts.iter().any(|s| s.is_err()) {
        poisoned = true;
    }

    let end = tokens.get_position();
    let span = Span::new(start, end);

    let solution = ProblemSolution::boxed(id.to_string(), stmts, span, poisoned);
    solution
}

// <problem_input> ::= "input" <id> <tuple> <inner_compound_stmt>
fn parse_problem_input<'a>(tokens: &mut Lexer) -> BoxAST<'a> {
    let mut poisoned: bool = false;

    let start = tokens.get_position();

    if consume_keyword(tokens, TokenKind::ProblemInput).is_poisoned() {
        poisoned = true;
    }

    let id = consume_identifier(tokens);

    if id.is_poisoned() {
        poisoned = true;
    }

    let stmts = parse_inner_compound_stmt(tokens);

    if stmts.iter().any(|s| s.is_err()) {
        poisoned = true;
    }

    let end = tokens.get_position();
    let span = Span::new(start, end);

    let input = ProblemInput::boxed(id.to_string(), stmts, span, poisoned);
    input
}

// <problem_output> ::= "output" <id> <tuple> <inner_compound_stmt>
fn parse_problem_output<'a>(tokens: &mut Lexer) -> BoxAST<'a> {
    let mut poisoned: bool = false;

    let start = tokens.get_position();

    if consume_keyword(tokens, TokenKind::ProblemOutput).is_poisoned() {
        poisoned = true;
    }

    let id = consume_identifier(tokens);

    if id.is_poisoned() {
        poisoned = true;
    }

    let stmts = parse_inner_compound_stmt(tokens);

    if stmts.iter().any(|s| s.is_err()) {
        poisoned = true;
    }

    let end = tokens.get_position();
    let span = Span::new(start, end);

    let output = ProblemOutput::boxed(id.to_string(), stmts, span, poisoned);
    output
}

// <stmt> ::= <object_decl> | <example_decl> | <var_def> | <problem_solution> |
// <problem_input> | <problem_output>
fn parse_stmt<'a>(tokens: &mut Lexer) -> BoxAST<'a> {
    let kind = tokens.peek().get_kind();

    match kind {
        TokenKind::Object => {
            let object = parse_object_decl(tokens);
            object
        }
        TokenKind::ProblemExample => {
            let example = parse_problem_example(tokens);
            example
        }
        TokenKind::ProblemSolution => {
            let solution = parse_problem_solution(tokens);
            solution
        }
        TokenKind::ProblemInput => {
            let input = parse_problem_input(tokens);
            input
        }
        TokenKind::ProblemOutput => {
            let output = parse_problem_output(tokens);
            output
        }
        TokenKind::LetKwd => {
            let var = parse_var_def(tokens);
            var
        }
        _ => {
            create_diagnostic(CompilerPass::Parser, tokens, Code::E000X);
            let object = parse_object_decl(tokens);
            object
        }
    }
}

// <stmts_list> ::= (<stmt>)*
fn parse_stmt_list<'a>(tokens: &mut Lexer) -> Vec<BoxAST<'a>> {
    let mut stmts: Vec<BoxAST> = vec![];

    while next_is_stmt(tokens) {
        let stmt = parse_stmt(tokens);
        stmts.push(stmt);
    }

    stmts
}

// <object_compound_desc> ::= "{" <object_desc> "}"
fn parse_object_compound_desc<'a>(tokens: &mut Lexer) -> BoxAST<'a> {
    consume_keyword(tokens, TokenKind::LeftBrace);
    let object_desc = parse_object_desc(tokens);
    consume_keyword(tokens, TokenKind::RightBrace);
    object_desc
}

// <object_decl> ::= "object" <id> <object_compound_desc> ";"
fn parse_object_decl<'a>(tokens: &mut Lexer) -> BoxAST<'a> {
    let mut poisoned: bool = false;

    let start = tokens.get_position();

    if consume_keyword(tokens, TokenKind::Object).is_poisoned() {
        poisoned = true;
    }

    let id = consume_identifier(tokens);

    if id.is_poisoned() {
        poisoned = true;
    }

    let object_desc = parse_object_compound_desc(tokens);

    if object_desc.is_err() {
        poisoned = true;
    }

    if consume_keyword(tokens, TokenKind::SemiColon).is_poisoned() {
        poisoned = true;
    }

    let end = tokens.get_position();
    let span = Span::new(start, end);

    let object_decl = ObjectDecl::boxed(id.to_string(), object_desc, span, poisoned);
    object_decl
}

// <compound_stmt> ::= "{" [<stmt_list>] "}"
fn parse_compound_stmt<'a>(tokens: &mut Lexer) -> Vec<BoxAST<'a>> {
    consume_keyword(tokens, TokenKind::LeftBrace);
    let stmts = parse_stmt_list(tokens);
    consume_keyword(tokens, TokenKind::RightBrace);
    stmts
}

// <problem_declaration> ::= "def" <id> "(" <int_const> ")" <compound_stmt>
fn parse_problem_decl<'a>(tokens: &mut Lexer) -> BoxAST<'a> {
    let mut poisoned: bool = false;

    let start = tokens.get_position();

    if consume_keyword(tokens, TokenKind::ProblemDef).is_poisoned() {
        poisoned = true;
    }

    let id = consume_identifier(tokens);

    if id.is_poisoned() { poisoned = true; }

    if consume_keyword(tokens, TokenKind::LeftPar).is_poisoned() {
        poisoned = true;
    }

    let int_const = consume_int_const(tokens);

    if int_const.is_poisoned() { poisoned = true; }

    if consume_keyword(tokens, TokenKind::RightPar).is_poisoned() {
        poisoned = true;
    }

    let stmts = parse_compound_stmt(tokens);

    if stmts.iter().any(|s| s.is_err()) { poisoned = true; }

    let end = tokens.get_position();
    let span = Span::new(start, end);

    Program::boxed(
        id.to_string(),
        int_const.to_int(),
        stmts,
        span,
        poisoned
    )
}

// <program> ::= <problem_declaration>
pub fn parse_program<'a>(tokens: &mut Lexer) -> BoxAST<'a> {
    // [START] Token is first
    tokens.advance();
    let program = parse_problem_decl(tokens);
    program
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

    pub fn parse(&mut self) -> BoxAST<'a> {
        parse_program(&mut self.lexer)
    }
}

// ==================================================================================== //
// Parser Test Suite                                                                    //
// ==================================================================================== //

#[cfg(test)]
mod test {
    use super::*;

    fn check_no_err_single_ast<'a, F>(text: &'a str, parser: F)
    where
        F: FnOnce(&mut Lexer) -> BoxAST<'a>,
    {
        let mut tokens = Lexer::new(text);

        tokens.advance();

        let res = parser(&mut tokens);

        assert!(res.is_ok())
    }

    fn check_no_err_multiple_ast<'a, F>(text: &'a str, parser: F)
    where
        F: FnOnce(&mut Lexer) -> Vec<BoxAST<'a>>,
    {
        let mut tokens = Lexer::new(text);

        tokens.advance();

        let res = parser(&mut tokens);

        assert!(res.iter().all(|ast| ast.is_ok()))
    }

    fn check_type(text: &str, expected_type: ErminiaType) {
        let mut tokens = Lexer::new(text);

        tokens.advance();

        let _ = consume_keyword(&mut tokens, TokenKind::LetKwd);
        let _ = consume_identifier(&mut tokens);

        let actual_type = if match_next(&mut tokens, TokenKind::Colon) {
            let _ = consume_keyword(&mut tokens, TokenKind::Colon);
            consume_data_type(&mut tokens)
        // TODO: Add logic for type inference
        } else {
            ErminiaType::default()
        };

        assert_eq!(actual_type, expected_type);
    }

    #[test]
    fn test_parse_object_decl() {
        let text = "object HA { shape: [(0,1), (0,2)], color: 1 };";

        check_no_err_single_ast(text, parse_object_decl)
    }

    #[test]
    fn test_parse_list_of_shapes() {
        let text = "[(0,1), (0,2), obj, obj(1,1), (x,y) | x <- [0..1], y <- [0..2]]";

        check_no_err_multiple_ast(text, parse_list_of_shapes)
    }

    #[test]
    fn test_parse_object_decl2() {
        let text =
            "object HA { shape: [(0,1), (0,2), (x,y) | x <- [0..1], y <- [0..2]], color: 1 };";

        check_no_err_single_ast(text, parse_object_decl)
    }

    #[test]
    fn test_parse_object_compound_desc() {
        let text = "{ shape : [(0,1), (0,2)], color : 1 }";

        check_no_err_single_ast(text, parse_object_compound_desc)
    }

    #[test]
    fn test_parse_object_desc() {
        let text = "shape : [(0,1), (0,2)], color : 1";

        check_no_err_single_ast(text, parse_object_desc)
    }

    #[test]
    fn test_parse_shape() {
        let text = "shape : [(0,1), (0,2)]";

        check_no_err_single_ast(text, parse_object_shape)
    }

    #[test]
    fn test_parse_color() {
        let text = "color : 1";

        check_no_err_single_ast(text, parse_object_color)
    }

    #[test]
    fn test_parse_shape_tuple_compr() {
        let text = "(x,y) | x <- [0..1], y <- [0..1]";

        check_no_err_single_ast(text, parse_shape_tuple_compr)
    }

    #[test]
    fn test_parse_var_def() {
        let text = "let x: object = HA(0,1);";

        check_no_err_single_ast(text, parse_var_def)
    }

    #[test]
    fn test_parse_var_def_explicit_object_type() {
        let text = "let x: object = HA(0,1);";

        check_type(text, ErminiaType::Object);
    }

    #[test]
    fn test_parse_var_def_default_object() {
        let text = "let x: object = HA;";

        check_no_err_single_ast(text, parse_var_def)
    }

    #[test]
    fn test_parse_var_def_default_explicit_object_type() {
        let text = "let x: object = HA;";

        check_type(text, ErminiaType::Object);
    }

    #[test]
    fn test_parse_var_def_default_object_no_type() {
        let text = "let x = HA;";

        check_no_err_single_ast(text, parse_var_def)
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

        check_no_err_single_ast(text, parse_stmt)
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
