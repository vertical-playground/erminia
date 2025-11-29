use crate::ast::ast::BoxAST;
use crate::ast::expr::*;
use crate::ast::stmt::*;
use crate::config::CompilerPass;
use crate::diagnostics::code::Code;
use crate::diagnostics::location::*;
use crate::lexer::lex::Lexer;
use crate::lexer::token::TokenKind;
use crate::types::ErminiaType;

// ==================================================================================== //
//  Utilities                                                                           //
// ==================================================================================== //

fn is_next_right_inclusive(tokens: &mut Lexer, diag: &mut Accumulator) -> ErminiaType {
    let kind = tokens.peek().get_kind();

    match kind {
        TokenKind::RightPar => ErminiaType::Bool(false),
        TokenKind::RightBracket => ErminiaType::Bool(true),
        _ => {
            let note = format!(
                "Expected ')' or ']' for range inclusivity, but found {:?}",
                kind
            );
            let mut diagnostic = create_diagnostic(CompilerPass::Parser, tokens, Code::E0002);
            diagnostic.add_note(note);
            diag.add_diag(diagnostic);
            ErminiaType::Poisoned
        }
    }
}

fn is_next_left_inclusive(tokens: &mut Lexer, diag: &mut Accumulator) -> ErminiaType {
    let kind = tokens.peek().get_kind();

    match kind {
        TokenKind::LeftPar => ErminiaType::Bool(false),
        TokenKind::LeftBracket => ErminiaType::Bool(true),
        _ => {
            let note = format!(
                "Expected '(' or '[' for range inclusivity, but found {:?}",
                kind
            );
            let mut diagnostic = create_diagnostic(CompilerPass::Parser, tokens, Code::E0002);
            diagnostic.add_note(note);
            diag.add_diag(diagnostic);
            ErminiaType::Poisoned
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
fn consume_data_type(tokens: &mut Lexer, diag: &mut Accumulator) -> ErminiaType {
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
            let note = format!("Expected data type but found {:?}. Did you mean to use 'int', 'string', or 'object'?", kind);
            let mut diagnostic = create_diagnostic(CompilerPass::Parser, tokens, Code::E0001);
            diagnostic.add_note(note);
            diag.add_diag(diagnostic);
            ErminiaType::Poisoned
        }
    }
}

fn consume_int_const(tokens: &mut Lexer, diag: &mut Accumulator) -> ErminiaType {
    let int_const = tokens.token;
    if int_const.get_kind() == TokenKind::Int {
        tokens.advance();
        ErminiaType::Integer(int_const.text.parse::<i32>().unwrap())
    } else {
        let note = format!("{:?} is not an integer.", int_const.get_kind());
        let mut diagnostic = create_diagnostic(CompilerPass::Parser, tokens, Code::E0003);
        diagnostic.add_note(note);
        diag.add_diag(diagnostic);
        ErminiaType::Poisoned
    }
}

fn consume_identifier(tokens: &mut Lexer, diag: &mut Accumulator) -> ErminiaType {
    let id = tokens.token;
    match id.get_kind() {
        TokenKind::Ident => {
            tokens.advance();
            ErminiaType::Ident(id.text.to_string())
        }
        _ => {
            let note = format!("{:?} is not an identifier.", id.get_kind());
            let mut diagnostic = create_diagnostic(CompilerPass::Parser, tokens, Code::E0001);
            diagnostic.add_note(note);
            diag.add_diag(diagnostic);
            ErminiaType::Poisoned
        }
    }
}

fn consume_keyword(tokens: &mut Lexer, expected: TokenKind, diag: &mut Accumulator) -> ErminiaType {
    let actual = tokens.peek().get_kind();
    if actual == expected {
        tokens.advance();
        ErminiaType::Void
    } else {
        let note = format!("Expected {:?} but found {:?}.", expected, actual);
        let mut diagnostic = create_diagnostic(CompilerPass::Parser, tokens, Code::E0001);
        diagnostic.add_note(note);
        diag.add_diag(diagnostic);
        ErminiaType::Poisoned
    }
}

// ==================================================================================== //
// Parsers                                                                              //
// ==================================================================================== //

// <expr> ::= <object_call> | <id> | <int_const>
fn parse_expr<'a>(tokens: &mut Lexer, diag: &mut Accumulator) -> BoxAST<'a> {
    let kind = tokens.peek().get_kind();
    let stmt: BoxAST;

    match kind {
        TokenKind::Ident => {
            let lookahead = tokens.lookahead();

            if matches!(lookahead.0, TokenKind::LeftPar) {
                stmt = parse_object_call(tokens, diag);

                if stmt.is_err() {
                    let note = format!(
                        "Expected 'ObjectCall' AST Node but failed to parse Node with id: {:?}.",
                        stmt.get_ast_id()
                    );
                    let mut diagnostic = create_diagnostic(CompilerPass::AST, tokens, Code::E0004);
                    diagnostic.add_note(note);
                    diag.add_diag(diagnostic);
                }
            } else {
                let id = consume_identifier(tokens, diag);

                if id.is_poisoned() {
                    let note = format!("{:?} is not an identifier.", id);
                    let mut diagnostic = create_diagnostic(CompilerPass::AST, tokens, Code::E0001);
                    diagnostic.add_note(note);
                    diag.add_diag(diagnostic);
                }
                stmt = RValue::boxed_id(id.to_string());

                if stmt.is_err() {
                    let note = format!(
                        "Expected 'RValue' AST Node but failed to parse Node with id: {:?}.",
                        stmt.get_ast_id()
                    );
                    let mut diagnostic = create_diagnostic(CompilerPass::AST, tokens, Code::E0004);
                    diagnostic.add_note(note);
                    diag.add_diag(diagnostic);
                }
            }

            stmt
        }
        TokenKind::Int => RValue::boxed_int(consume_int_const(tokens, diag).to_int()),
        _ => {
            let note = format!(
                "Could not parse expression. Expected id or integer but found {:?}",
                kind
            );
            let mut diagnostic = create_diagnostic(CompilerPass::Parser, tokens, Code::E0001);
            diagnostic.add_note(note);
            stmt = parse_object_call(tokens, diag);
            stmt
        }
    }
}

// <list_of_exprs> ::= <expr> ("," <expr>)*
fn parse_list_of_exprs<'a>(tokens: &mut Lexer, diag: &mut Accumulator) -> Vec<BoxAST<'a>> {
    let mut exprs: Vec<BoxAST> = vec![];

    while next_is_expr(tokens) {
        let expr = parse_expr(tokens, diag);

        exprs.push(expr);

        let next = tokens.peek().get_kind();

        if matches!(next, TokenKind::Comma) {
            consume_keyword(tokens, TokenKind::Comma, diag);
        }
    }

    exprs
}

// <func_call> ::= <id> "(" [<list_of_exprs>] ")" ";"
fn parse_func_call<'a>(tokens: &mut Lexer, diag: &mut Accumulator) -> BoxAST<'a> {
    let mut poisoned: bool = false;

    let start = tokens.get_position();
    let id = consume_identifier(tokens, diag);

    if id.is_poisoned() {
        poisoned = true;
    }

    consume_keyword(tokens, TokenKind::LeftPar, diag);

    let exprs = parse_list_of_exprs(tokens, diag);

    consume_keyword(tokens, TokenKind::RightPar, diag);

    consume_keyword(tokens, TokenKind::SemiColon, diag);
    let end = tokens.get_position();
    let span = Span::new(start, end);

    let func = FuncCall::boxed(id.to_string(), exprs, span, poisoned);

    func
}

// <inner_stmt> ::= <object_decl> | <var_def> | <func_call>
fn parse_inner_stmt<'a>(tokens: &mut Lexer, diag: &mut Accumulator) -> BoxAST<'a> {
    let kind = tokens.peek().get_kind();

    match kind {
        TokenKind::Object => {
            let object = parse_object_decl(tokens, diag);
            object
        }
        TokenKind::LetKwd => {
            let var = parse_var_def(tokens, diag);
            var
        }
        TokenKind::Ident => {
            let func = parse_func_call(tokens, diag);
            func
        }
        _ => {
            create_diagnostic(CompilerPass::Parser, tokens, Code::E000X);
            let object = parse_object_decl(tokens, diag);
            object
        }
    }
}

// <inner_stmt_list> ::= (<inner_stmt>)*
fn parse_inner_stmt_list<'a>(tokens: &mut Lexer, diag: &mut Accumulator) -> Vec<BoxAST<'a>> {
    let mut stmts: Vec<BoxAST> = vec![];
    while next_is_stmt(tokens) {
        let stmt = parse_inner_stmt(tokens, diag);
        stmts.push(stmt);
    }
    stmts
}

// <inner_compound_stmt> ::= "{" [<inner_stmt_list>] "}"
fn parse_inner_compound_stmt<'a>(tokens: &mut Lexer, diag: &mut Accumulator) -> Vec<BoxAST<'a>> {
    consume_keyword(tokens, TokenKind::LeftBrace, diag);
    let stmts = parse_inner_stmt_list(tokens, diag);
    consume_keyword(tokens, TokenKind::RightBrace, diag);
    stmts
}

// TODO: handle type inference
// <var_def> ::= "let" <id> ":" <data_type> "=" <expr> ";"
fn parse_var_def<'a>(tokens: &mut Lexer, diag: &mut Accumulator) -> BoxAST<'a> {
    let mut poisoned: bool = false;

    let start = tokens.get_position();
    let mut data_type: ErminiaType = ErminiaType::default();

    if consume_keyword(tokens, TokenKind::LetKwd, diag).is_poisoned() {
        poisoned = true;
    }

    let id = consume_identifier(tokens, diag);

    if id.is_poisoned() {
        poisoned = true;
    }

    if match_next(tokens, TokenKind::Colon) {
        if consume_keyword(tokens, TokenKind::Colon, diag).is_poisoned() {
            poisoned = true;
        }

        // change here if it's explicit about data type
        data_type = consume_data_type(tokens, diag);

        if data_type.is_poisoned() {
            poisoned = true;
        }
    }

    if consume_keyword(tokens, TokenKind::Equals, diag).is_poisoned() {
        poisoned = true;
    }

    let expr = parse_expr(tokens, diag);

    if expr.is_err() {
        poisoned = true;
    }

    if consume_keyword(tokens, TokenKind::SemiColon, diag).is_poisoned() {
        poisoned = true;
    }

    let end = tokens.get_position();
    let span = Span::new(start, end);

    // TODO
    let var_def = VarDef::boxed(id.to_string(), data_type, expr, span, poisoned);

    var_def
}

// <range> ::= ("[" | "(") <int_const> ".." <int_const> ("]" | ")")
fn parse_range<'a>(tokens: &mut Lexer, diag: &mut Accumulator) -> BoxAST<'a> {
    let mut poisoned: bool = false;

    let start = tokens.get_position();
    let is_left_inclusive = is_next_left_inclusive(tokens, diag);

    if is_left_inclusive.is_poisoned() {
        poisoned = true;
    }

    let left = consume_int_const(tokens, diag);

    if left.is_poisoned() {
        poisoned = true;
    }

    if consume_keyword(tokens, TokenKind::Range, diag).is_poisoned() {
        poisoned = true;
    }

    let right = consume_int_const(tokens, diag);

    if right.is_poisoned() {
        poisoned = true;
    }

    let is_right_inclusive = is_next_right_inclusive(tokens, diag);

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
fn parse_shape_tuple_iter<'a>(tokens: &mut Lexer, diag: &mut Accumulator) -> BoxAST<'a> {
    let mut poisoned: bool = false;

    let start = tokens.get_position();
    let coord = consume_identifier(tokens, diag);

    if coord.is_poisoned() {
        poisoned = true;
    }

    if consume_keyword(tokens, TokenKind::LeftArrow, diag).is_poisoned() {
        poisoned = true;
    }

    let range = parse_range(tokens, diag);

    if range.is_err() {
        poisoned = true;
    }

    let end = tokens.get_position();
    let span = Span::new(start, end);

    TupleIterator::boxed(coord.to_string(), range, span, poisoned)
}

// <shape_tuple_iter_pair> ::= <shape_tuple_iter> ("," <shape_tuple_iter>)
fn parse_shape_tuple_iter_pair<'a>(tokens: &mut Lexer, diag: &mut Accumulator) -> Vec<BoxAST<'a>> {
    let mut pairs: Vec<BoxAST> = vec![];

    let first_tuple_iter = parse_shape_tuple_iter(tokens, diag);

    if first_tuple_iter.is_err() {
        let note = format!(
            "Expected 'TupleIterator' AST Node but failed to parse Node with id: {:?}.",
            first_tuple_iter.get_ast_id()
        );
        let mut diagnostic = create_diagnostic(CompilerPass::AST, tokens, Code::E0004);
        diagnostic.add_note(note);
        diag.add_diag(diagnostic);
    }

    pairs.push(first_tuple_iter);

    if next_is_comma(tokens) {
        consume_keyword(tokens, TokenKind::Comma, diag);
        let second_tuple_iter = parse_shape_tuple_iter(tokens, diag);
        pairs.push(second_tuple_iter);
    }

    pairs
}

// <shape_tuple_compr> ::= <shape_tuple> "|" <shape_tuple_iter_pair>
fn parse_shape_tuple_compr<'a>(tokens: &mut Lexer, diag: &mut Accumulator) -> BoxAST<'a> {
    let mut poisoned: bool = false;

    let start = tokens.get_position();
    let tuple = parse_shape_tuple_generic(tokens, diag);

    if tuple.is_err() {
        poisoned = true;
    }

    if consume_keyword(tokens, TokenKind::Pipe, diag).is_poisoned() {
        poisoned = true;
    }

    let iter_pair = parse_shape_tuple_iter_pair(tokens, diag);

    if iter_pair.iter().any(|s| s.is_err()) {
        poisoned = true;
    }

    let end = tokens.get_position();
    let span = Span::new(start, end);

    TupleComprehension::boxed(tuple, iter_pair, span, poisoned)
}

// <object_call> ::= <id> <shape_tuple>
fn parse_object_call<'a>(tokens: &mut Lexer, diag: &mut Accumulator) -> BoxAST<'a> {
    let mut poisoned: bool = false;

    let start = tokens.get_position();
    let id = consume_identifier(tokens, diag);

    if id.is_poisoned() {
        poisoned = true;
    }

    match tokens.peek().get_kind() {
        TokenKind::LeftPar => {
            let tuple = parse_shape_tuple(tokens, diag);

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
fn parse_shape_tuple_generic<'a>(tokens: &mut Lexer, diag: &mut Accumulator) -> BoxAST<'a> {
    let mut poisoned: bool = false;

    let start = tokens.get_position();

    if consume_keyword(tokens, TokenKind::LeftPar, diag).is_poisoned() {
        poisoned = true;
    }

    let mut left: BoxAST = GenericTupleOption::boxed_none();
    let mut right: BoxAST = GenericTupleOption::boxed_none();

    if match_next(tokens, TokenKind::Int) {
        let int_const = consume_int_const(tokens, diag);

        if int_const.is_poisoned() {
            poisoned = true;
        }

        left = GenericTupleOption::boxed_int(int_const.to_int(), poisoned);
    } else if match_next(tokens, TokenKind::Ident) {
        let id = consume_identifier(tokens, diag);

        if id.is_poisoned() {
            poisoned = true;
        }

        left = GenericTupleOption::boxed_id(id.to_string(), poisoned);
    }

    // <inner_stmt> ::= <object_decl>
    consume_keyword(tokens, TokenKind::Comma, diag);

    if match_next(tokens, TokenKind::Int) {
        let int_const = consume_int_const(tokens, diag);

        if int_const.is_poisoned() {
            poisoned = true;
        }

        right = GenericTupleOption::boxed_int(int_const.to_int(), poisoned);
    } else if match_next(tokens, TokenKind::Ident) {
        let id = consume_identifier(tokens, diag);

        if id.is_poisoned() {
            poisoned = true;
        }

        right = GenericTupleOption::boxed_id(id.to_string(), poisoned);
    }

    consume_keyword(tokens, TokenKind::RightPar, diag);
    let end = tokens.get_position();
    let span = Span::new(start, end);

    GenericTuple::boxed(left, right, span, poisoned)
}

// <shape_tuple> ::= "(" <int_const> "," <int_const> ")"
fn parse_shape_tuple<'a>(tokens: &mut Lexer, diag: &mut Accumulator) -> BoxAST<'a> {
    let mut poisoned: bool = false;

    let span_start = tokens.get_position();

    if consume_keyword(tokens, TokenKind::LeftPar, diag).is_poisoned() {
        let diagnostic = create_diagnostic(CompilerPass::Parser, tokens, Code::E000X);
        diag.add_diag(diagnostic);
        poisoned = true;
    }

    let left = consume_int_const(tokens, diag);

    if left.is_poisoned() {
        poisoned = true;
    }

    if consume_keyword(tokens, TokenKind::Comma, diag).is_poisoned() {
        poisoned = true;
    }

    let right = consume_int_const(tokens, diag);

    if right.is_poisoned() {
        poisoned = true;
    }

    if consume_keyword(tokens, TokenKind::RightPar, diag).is_poisoned() {
        poisoned = true;
    }

    let span_end = tokens.get_position();
    let span = Span::new(span_start, span_end);

    Tuple::boxed(left.to_int(), right.to_int(), span, poisoned)
}

// <shape> ::= <shape_tuple> | <shape_tuple_compr> | <object_call> | <id>
fn parse_shape<'a>(tokens: &mut Lexer, diag: &mut Accumulator) -> BoxAST<'a> {
    let kind = tokens.peek().get_kind();

    match kind {
        TokenKind::LeftPar => {
            let lookahead = tokens.lookahead_by(4);

            if matches!(lookahead, TokenKind::Pipe) {
                let compr = parse_shape_tuple_compr(tokens, diag);

                if compr.is_err() {
                    let note = format!("Expected 'TupleComprehension' AST Node but failed to parse Node with id: {:?}.", compr.get_ast_id());
                    let mut diagnostic = create_diagnostic(CompilerPass::AST, tokens, Code::E0004);
                    diagnostic.add_note(note);
                    diag.add_diag(diagnostic);
                }
                return compr;
            }

            let tuple = parse_shape_tuple(tokens, diag);
            tuple
        }
        TokenKind::Ident => {
            let object = parse_object_call(tokens, diag);
            object
        }
        _ => {
            create_diagnostic(CompilerPass::Parser, tokens, Code::E000X);
            let lookahead = tokens.lookahead_by(4);

            if matches!(lookahead, TokenKind::Pipe) {
                let compr = parse_shape_tuple_compr(tokens, diag);
                return compr;
            }

            let tuple = parse_shape_tuple(tokens, diag);
            tuple
        }
    }
}

// <object_color> ::= "color" ":" <int_const>
fn parse_object_color<'a>(tokens: &mut Lexer, diag: &mut Accumulator) -> BoxAST<'a> {
    let mut poisoned: bool = false;

    let start = tokens.get_position();
    if consume_keyword(tokens, TokenKind::ObjectColor, diag).is_poisoned() {
        poisoned = true;
    }

    if consume_keyword(tokens, TokenKind::Colon, diag).is_poisoned() {
        poisoned = true;
    }

    let int_const = consume_int_const(tokens, diag);

    if int_const.is_poisoned() {
        poisoned = true;
    }

    let end = tokens.get_position();
    let span = Span::new(start, end);

    let color = ObjectColor::boxed(int_const.to_int(), span, poisoned);
    color
}

// <list_of_shapes> ::= "[" <shape> ("," <shape>)* "]"
fn parse_list_of_shapes<'a>(tokens: &mut Lexer, diag: &mut Accumulator) -> Vec<BoxAST<'a>> {
    let mut shapes: Vec<BoxAST> = vec![];
    consume_keyword(tokens, TokenKind::LeftBracket, diag);
    let shape = parse_shape(tokens, diag);
    shapes.push(shape);
    while next_is_comma(tokens) {
        consume_keyword(tokens, TokenKind::Comma, diag);
        let shape = parse_shape(tokens, diag);
        shapes.push(shape);
    }
    consume_keyword(tokens, TokenKind::RightBracket, diag);
    shapes
}

// <object_shape> ::= "shape" ":" <list_of_shapes>
fn parse_object_shape<'a>(tokens: &mut Lexer, diag: &mut Accumulator) -> BoxAST<'a> {
    let mut poisoned: bool = false;

    let start = tokens.get_position();
    if consume_keyword(tokens, TokenKind::ObjectShape, diag).is_poisoned() {
        poisoned = true;
    }

    if consume_keyword(tokens, TokenKind::Colon, diag).is_poisoned() {
        poisoned = true;
    }

    let shapes = parse_list_of_shapes(tokens, diag);

    if shapes.iter().any(|s| s.is_err()) {
        poisoned = true;
    }

    let end = tokens.get_position();
    let span = Span::new(start, end);

    let object_shape = ObjectShape::boxed(shapes, span, poisoned);
    object_shape
}

// <object_desc> ::= <object_shape> "," <object_color> | <object_color> "," <object_shape>
fn parse_object_desc<'a>(tokens: &mut Lexer, diag: &mut Accumulator) -> BoxAST<'a> {
    let mut poisoned: bool = false;

    let kind = tokens.peek().get_kind();

    let start = tokens.get_position();

    match kind {
        TokenKind::ObjectShape => {
            let shape = parse_object_shape(tokens, diag);
            consume_keyword(tokens, TokenKind::Comma, diag);
            let color = parse_object_color(tokens, diag);
            let end = tokens.get_position();
            let span = Span::new(start, end);

            let object_desc = ObjectDesc::boxed(shape, color, span, poisoned);
            object_desc
        }
        TokenKind::ObjectColor => {
            let color = parse_object_color(tokens, diag);
            consume_keyword(tokens, TokenKind::Comma, diag);
            let shape = parse_object_shape(tokens, diag);
            let end = tokens.get_position();
            let span = Span::new(start, end);

            let object_desc = ObjectDesc::boxed(shape, color, span, poisoned);
            object_desc
        }
        _ => {
            poisoned = true;
            create_diagnostic(CompilerPass::Parser, tokens, Code::E000X);
            let shape = parse_object_shape(tokens, diag);
            consume_keyword(tokens, TokenKind::Comma, diag);
            let color = parse_object_color(tokens, diag);
            let end = tokens.get_position();
            let span = Span::new(start, end);

            let object_desc = ObjectDesc::boxed(shape, color, span, poisoned);
            object_desc
        }
    }
}

// <example_decl> ::= "example" <id> <inner_compound_stmt>
fn parse_problem_example<'a>(tokens: &mut Lexer, diag: &mut Accumulator) -> BoxAST<'a> {
    let mut poisoned: bool = false;

    let start = tokens.get_position();

    if consume_keyword(tokens, TokenKind::ProblemExample, diag).is_poisoned() {
        poisoned = true;
    }

    let id = consume_identifier(tokens, diag);

    if id.is_poisoned() {
        poisoned = true;
    }

    let stmts = parse_inner_compound_stmt(tokens, diag);

    if stmts.iter().any(|s| s.is_err()) {
        poisoned = true;
    }

    let end = tokens.get_position();
    let span = Span::new(start, end);

    let example = ProblemExample::boxed(id.to_string(), stmts, span, poisoned);
    example
}

// <problem_solution> ::= "solution" <id> <inner_compound_stmt>
fn parse_problem_solution<'a>(tokens: &mut Lexer, diag: &mut Accumulator) -> BoxAST<'a> {
    let mut poisoned: bool = false;

    let start = tokens.get_position();

    if consume_keyword(tokens, TokenKind::ProblemSolution, diag).is_poisoned() {
        poisoned = true;
    }

    let id = consume_identifier(tokens, diag);

    if id.is_poisoned() {
        poisoned = true;
    }

    let stmts = parse_inner_compound_stmt(tokens, diag);

    if stmts.iter().any(|s| s.is_err()) {
        poisoned = true;
    }

    let end = tokens.get_position();
    let span = Span::new(start, end);

    let solution = ProblemSolution::boxed(id.to_string(), stmts, span, poisoned);
    solution
}

// <problem_input> ::= "input" <id> <tuple> <inner_compound_stmt>
fn parse_problem_input<'a>(tokens: &mut Lexer, diag: &mut Accumulator) -> BoxAST<'a> {
    let mut poisoned: bool = false;

    let start = tokens.get_position();

    if consume_keyword(tokens, TokenKind::ProblemInput, diag).is_poisoned() {
        poisoned = true;
    }

    let id = consume_identifier(tokens, diag);

    if id.is_poisoned() {
        poisoned = true;
    }

    let stmts = parse_inner_compound_stmt(tokens, diag);

    if stmts.iter().any(|s| s.is_err()) {
        poisoned = true;
    }

    let end = tokens.get_position();
    let span = Span::new(start, end);

    let input = ProblemInput::boxed(id.to_string(), stmts, span, poisoned);
    input
}

// <problem_output> ::= "output" <id> <tuple> <inner_compound_stmt>
fn parse_problem_output<'a>(tokens: &mut Lexer, diag: &mut Accumulator) -> BoxAST<'a> {
    let mut poisoned: bool = false;

    let start = tokens.get_position();

    if consume_keyword(tokens, TokenKind::ProblemOutput, diag).is_poisoned() {
        poisoned = true;
    }

    let id = consume_identifier(tokens, diag);

    if id.is_poisoned() {
        poisoned = true;
    }

    let stmts = parse_inner_compound_stmt(tokens, diag);

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
fn parse_stmt<'a>(tokens: &mut Lexer, diag: &mut Accumulator) -> BoxAST<'a> {
    let kind = tokens.peek().get_kind();

    match kind {
        TokenKind::Object => {
            let object = parse_object_decl(tokens, diag);
            object
        }
        TokenKind::ProblemExample => {
            let example = parse_problem_example(tokens, diag);
            example
        }
        TokenKind::ProblemSolution => {
            let solution = parse_problem_solution(tokens, diag);
            solution
        }
        TokenKind::ProblemInput => {
            let input = parse_problem_input(tokens, diag);
            input
        }
        TokenKind::ProblemOutput => {
            let output = parse_problem_output(tokens, diag);
            output
        }
        TokenKind::LetKwd => {
            let var = parse_var_def(tokens, diag);
            var
        }
        _ => {
            create_diagnostic(CompilerPass::Parser, tokens, Code::E000X);
            let object = parse_object_decl(tokens, diag);
            object
        }
    }
}

// <stmts_list> ::= (<stmt>)*
fn parse_stmt_list<'a>(tokens: &mut Lexer, diag: &mut Accumulator) -> Vec<BoxAST<'a>> {
    let mut stmts: Vec<BoxAST> = vec![];

    while next_is_stmt(tokens) {
        let stmt = parse_stmt(tokens, diag);
        stmts.push(stmt);
    }

    stmts
}

// <object_compound_desc> ::= "{" <object_desc> "}"
fn parse_object_compound_desc<'a>(tokens: &mut Lexer, diag: &mut Accumulator) -> BoxAST<'a> {
    consume_keyword(tokens, TokenKind::LeftBrace, diag);
    let object_desc = parse_object_desc(tokens, diag);
    consume_keyword(tokens, TokenKind::RightBrace, diag);
    object_desc
}

// <object_decl> ::= "object" <id> <object_compound_desc> ";"
fn parse_object_decl<'a>(tokens: &mut Lexer, diag: &mut Accumulator) -> BoxAST<'a> {
    let mut poisoned: bool = false;

    let start = tokens.get_position();

    if consume_keyword(tokens, TokenKind::Object, diag).is_poisoned() {
        poisoned = true;
    }

    let id = consume_identifier(tokens, diag);

    if id.is_poisoned() {
        poisoned = true;
    }

    let object_desc = parse_object_compound_desc(tokens, diag);

    if object_desc.is_err() {
        poisoned = true;
    }

    if consume_keyword(tokens, TokenKind::SemiColon, diag).is_poisoned() {
        poisoned = true;
    }

    let end = tokens.get_position();
    let span = Span::new(start, end);

    let object_decl = ObjectDecl::boxed(id.to_string(), object_desc, span, poisoned);
    object_decl
}

// <compound_stmt> ::= "{" [<stmt_list>] "}"
fn parse_compound_stmt<'a>(tokens: &mut Lexer, diag: &mut Accumulator) -> Vec<BoxAST<'a>> {
    consume_keyword(tokens, TokenKind::LeftBrace, diag);
    let stmts = parse_stmt_list(tokens, diag);
    consume_keyword(tokens, TokenKind::RightBrace, diag);
    stmts
}

// <problem_declaration> ::= "def" <id> "(" <int_const> ")" <compound_stmt>
fn parse_problem_decl<'a>(tokens: &mut Lexer, diag: &mut Accumulator) -> BoxAST<'a> {
    let mut poisoned: bool = false;

    let start = tokens.get_position();

    if consume_keyword(tokens, TokenKind::ProblemDef, diag).is_poisoned() {
        let diagnostic = create_diagnostic(CompilerPass::Parser, tokens, Code::E0001);
        diag.add_diag(diagnostic);
        poisoned = true;
    }

    let id = consume_identifier(tokens, diag);

    if id.is_poisoned() {
        poisoned = true;
    }

    if consume_keyword(tokens, TokenKind::LeftPar, diag).is_poisoned() {
        poisoned = true;
    }

    let int_const = consume_int_const(tokens, diag);

    if int_const.is_poisoned() {
        poisoned = true;
    }

    if consume_keyword(tokens, TokenKind::RightPar, diag).is_poisoned() {
        poisoned = true;
    }

    let stmts = parse_compound_stmt(tokens, diag);

    if stmts.iter().any(|s| s.is_err()) {
        poisoned = true;
    }

    let end = tokens.get_position();
    let span = Span::new(start, end);

    Program::boxed(id.to_string(), int_const.to_int(), stmts, span, poisoned)
}

// <program> ::= <problem_declaration>
pub fn parse_program<'a>(tokens: &mut Lexer, diag: &mut Accumulator) -> BoxAST<'a> {
    // [START] Token is first
    tokens.advance();
    let program = parse_problem_decl(tokens, diag);

    program
}

// ==================================================================================== //
// Parser Object                                                                        //
// ==================================================================================== //

pub struct Parser<'a> {
    lexer: Lexer<'a>,
    diagnostics: Accumulator,
}

impl<'a> Parser<'a> {
    pub fn new<'input>(input: &'input str) -> Parser<'input> {
        let lexer = Lexer::new(input);
        let diagnostics = Accumulator::new();
        Parser { lexer, diagnostics }
    }

    pub fn parse(&mut self) -> BoxAST<'a> {
        parse_program(&mut self.lexer, &mut self.diagnostics)
    }

    pub fn get_diagnostics(&self) -> &Accumulator {
        &self.diagnostics
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
        F: FnOnce(&mut Lexer, &mut Accumulator) -> BoxAST<'a>,
    {
        let mut tokens = Lexer::new(text);
        let mut diag = Accumulator::new();

        tokens.advance();

        let res = parser(&mut tokens, &mut diag);

        if res.is_err() {
            println!("Error in parsing for input {:?}: \n {:?}", text, res);
            for d in diag.get(CompilerPass::Parser) {
                println!("{}", d);
            }
        }

        assert!(res.is_ok())
    }

    fn check_no_err_multiple_ast<'a, F>(text: &'a str, parser: F)
    where
        F: FnOnce(&mut Lexer, &mut Accumulator) -> Vec<BoxAST<'a>>,
    {
        let mut tokens = Lexer::new(text);
        let mut diag = Accumulator::new();

        tokens.advance();

        let res = parser(&mut tokens, &mut diag);

        if res.iter().any(|ast| ast.is_err()) {
            println!("Error in parsing for input {:?}: \n {:?}", text, res);
            for d in diag.get(CompilerPass::Parser) {
                println!("{}", d);
            }
        }

        assert!(res.iter().all(|ast| ast.is_ok()))
    }

    fn check_type(text: &str, expected_type: ErminiaType) {
        let mut tokens = Lexer::new(text);
        let mut diag = Accumulator::new();

        tokens.advance();

        let _ = consume_keyword(&mut tokens, TokenKind::LetKwd, &mut diag);
        let _ = consume_identifier(&mut tokens, &mut diag);

        let actual_type = if match_next(&mut tokens, TokenKind::Colon) {
            let _ = consume_keyword(&mut tokens, TokenKind::Colon, &mut diag);
            consume_data_type(&mut tokens, &mut diag)
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

        check_no_err_single_ast(text, parse_shape)
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
