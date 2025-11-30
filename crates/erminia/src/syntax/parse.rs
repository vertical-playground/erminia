use crate::ast::ast::BoxAST;
use crate::ast::expr::*;
use crate::ast::stmt::*;
use crate::config::CompilerPass;
use crate::diagnostics::code::Code;
use crate::diagnostics::location::*;
use crate::diagnostics::messages::*;
use crate::diagnostics::DiagnosticBuilder;
use crate::lexer::lex::Lexer;
use crate::lexer::token::TokenKind;
use crate::types::ErminiaType;

type DB = DiagnosticBuilder;
const PARSER_PASS: CompilerPass = CompilerPass::Parser;

// ==================================================================================== //
//  Utilities                                                                           //
// ==================================================================================== //

fn is_next_right_inclusive(tokens: &mut Lexer, diag: &mut Accumulator) -> ErminiaType {
    let kind = tokens.peek().get_kind();

    let res = match kind {
        TokenKind::RightPar => ErminiaType::Bool(false),
        TokenKind::RightBracket => ErminiaType::Bool(true),
        _ => {
            diag.add_diag(
                DB::build(PARSER_PASS, Code::E0002)
                    .with_note(Note::ExpectedRightInclusive)
                    .with_args(MessageKind::Note, vec![kind.to_string()])
                    .with_help(Help::ConsiderChangingToInclusive)
                    .emmit(tokens, Span::default()),
            );

            ErminiaType::Poisoned
        }
    };

    tokens.advance();

    res
}

fn is_next_left_inclusive(tokens: &mut Lexer, diag: &mut Accumulator) -> ErminiaType {
    let kind = tokens.peek().get_kind();

    let res = match kind {
        TokenKind::LeftPar => ErminiaType::Bool(false),
        TokenKind::LeftBracket => ErminiaType::Bool(true),
        _ => {
            diag.add_diag(
                DB::build(PARSER_PASS, Code::E0002)
                    .with_note(Note::ExpectedLeftInclusive)
                    .with_args(MessageKind::Note, vec![kind.to_string()])
                    .emmit(tokens, Span::default()),
            );

            ErminiaType::Poisoned
        }
    };

    tokens.advance();

    res
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
    let res = match kind {
        TokenKind::Object => ErminiaType::Object,
        TokenKind::Int => ErminiaType::Int,
        TokenKind::String => ErminiaType::String,
        _ => {
            diag.add_diag(
                DB::build(PARSER_PASS, Code::E0002)
                    .with_note(Note::ExpectedDataType)
                    .with_args(MessageKind::Note, vec![kind.to_string()])
                    .with_help(Help::ConsiderChangingToInclusive)
                    .emmit(tokens, Span::default()),
            );

            ErminiaType::Poisoned
        }
    };

    tokens.advance();

    res
}

fn consume_int_const(tokens: &mut Lexer, diag: &mut Accumulator) -> ErminiaType {
    let int_const = tokens.token;

    let res = if int_const.get_kind() == TokenKind::Int {
        ErminiaType::Integer(int_const.text.parse::<i32>().unwrap())
    } else {
        diag.add_diag(
            DB::build(PARSER_PASS, Code::E0003)
                .with_note(Note::ExpectedInteger)
                .with_args(MessageKind::Note, vec![int_const.get_kind().to_string()])
                .emmit(tokens, Span::default()),
        );

        ErminiaType::Poisoned
    };

    tokens.advance();

    res
}

fn consume_identifier(tokens: &mut Lexer, diag: &mut Accumulator) -> ErminiaType {
    let id = tokens.token;

    let res = match id.get_kind() {
        TokenKind::Ident => ErminiaType::Ident(id.text.to_string()),
        _ => {
            diag.add_diag(
                DB::build(PARSER_PASS, Code::E0001)
                    .with_note(Note::ExpectedIdentifier)
                    .with_args(MessageKind::Note, vec![id.get_kind().to_string()])
                    .emmit(tokens, Span::default()),
            );

            ErminiaType::Poisoned
        }
    };

    tokens.advance();

    res
}

fn consume_keyword(tokens: &mut Lexer, expected: TokenKind, diag: &mut Accumulator) -> ErminiaType {
    let actual = tokens.peek().get_kind();

    let res = if actual == expected {
        ErminiaType::Void
    } else {
        diag.add_diag(
            DB::build(PARSER_PASS, Code::E0001)
                .with_note(Note::ExpectedSomethingElse)
                .with_args(
                    MessageKind::Note,
                    vec![expected.to_string(), actual.to_string()],
                )
                .emmit(tokens, Span::default()),
        );

        ErminiaType::Poisoned
    };

    tokens.advance();

    res
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
            } else {
                let id = consume_identifier(tokens, diag);
                stmt = RValue::boxed_id(id.to_string());
                stmt.check(tokens, diag, Span::default());
            }

            stmt
        }
        TokenKind::Int => {
            let stmt = RValue::boxed_int(consume_int_const(tokens, diag).to_int());
            stmt.check(tokens, diag, Span::default());

            stmt
        }
        _ => {
            diag.add_diag(
                DB::build(PARSER_PASS, Code::E0001)
                    .with_note(Note::ExpectedIDorInteger)
                    .with_args(MessageKind::Note, vec![kind.to_string()])
                    .emmit(tokens, Span::default()),
            );

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
    let start = tokens.get_position();
    let id = consume_identifier(tokens, diag);
    consume_keyword(tokens, TokenKind::LeftPar, diag);

    let exprs = parse_list_of_exprs(tokens, diag);

    consume_keyword(tokens, TokenKind::RightPar, diag);

    consume_keyword(tokens, TokenKind::SemiColon, diag);
    let end = tokens.get_position();
    let span = Span::new(start, end);

    let func = FuncCall::boxed(id, exprs, span);
    func.check(tokens, diag, span);

    func
}

// <inner_stmt> ::= <object_decl> | <var_def> | <func_call>
fn parse_inner_stmt<'a>(tokens: &mut Lexer, diag: &mut Accumulator) -> BoxAST<'a> {
    let kind = tokens.peek().get_kind();

    match kind {
        TokenKind::Object => parse_object_decl(tokens, diag),
        TokenKind::LetKwd => parse_var_def(tokens, diag),
        TokenKind::Ident => parse_func_call(tokens, diag),
        _ => parse_object_decl(tokens, diag),
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
    let start = tokens.get_position();
    let mut data_type: ErminiaType = ErminiaType::default();

    consume_keyword(tokens, TokenKind::LetKwd, diag);

    let id = consume_identifier(tokens, diag);

    if match_next(tokens, TokenKind::Colon) {
        consume_keyword(tokens, TokenKind::Colon, diag);

        // change here if it's explicit about data type
        data_type = consume_data_type(tokens, diag);
    }

    consume_keyword(tokens, TokenKind::Equals, diag);

    let expr = parse_expr(tokens, diag);

    consume_keyword(tokens, TokenKind::SemiColon, diag);

    let end = tokens.get_position();
    let span = Span::new(start, end);

    let var_def = VarDef::boxed(id, data_type, expr, span);
    var_def.check(tokens, diag, span);

    var_def
}

// <range> ::= ("[" | "(") <int_const> ".." <int_const> ("]" | ")")
fn parse_range<'a>(tokens: &mut Lexer, diag: &mut Accumulator) -> BoxAST<'a> {
    let start = tokens.get_position();
    let is_left_inclusive = is_next_left_inclusive(tokens, diag);

    let left = consume_int_const(tokens, diag);

    consume_keyword(tokens, TokenKind::Range, diag);

    let right = consume_int_const(tokens, diag);

    let is_right_inclusive = is_next_right_inclusive(tokens, diag);

    let end = tokens.get_position();
    let span = Span::new(start, end);

    let range = Range::boxed(is_left_inclusive, is_right_inclusive, left, right, span);
    range.check(tokens, diag, span);

    range
}

// <shape_tuple_iter> ::= <id> "<-" <range>
fn parse_shape_tuple_iter<'a>(tokens: &mut Lexer, diag: &mut Accumulator) -> BoxAST<'a> {
    let start = tokens.get_position();
    let coord = consume_identifier(tokens, diag);

    consume_keyword(tokens, TokenKind::LeftArrow, diag);

    let range = parse_range(tokens, diag);

    let end = tokens.get_position();
    let span = Span::new(start, end);

    let tuple_iter = TupleIterator::boxed(coord, range, span);
    tuple_iter.check(tokens, diag, span);

    tuple_iter
}

// <shape_tuple_iter_pair> ::= <shape_tuple_iter> ("," <shape_tuple_iter>)
fn parse_shape_tuple_iter_pair<'a>(tokens: &mut Lexer, diag: &mut Accumulator) -> Vec<BoxAST<'a>> {
    let mut pairs: Vec<BoxAST> = vec![];

    let first_tuple_iter = parse_shape_tuple_iter(tokens, diag);

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
    let start = tokens.get_position();
    let tuple = parse_shape_tuple_generic(tokens, diag);

    consume_keyword(tokens, TokenKind::Pipe, diag);

    let iter_pair = parse_shape_tuple_iter_pair(tokens, diag);

    let end = tokens.get_position();
    let span = Span::new(start, end);

    let tuple_compr = TupleComprehension::boxed(tuple, iter_pair, span);
    tuple_compr.check(tokens, diag, span);

    tuple_compr
}

// <object_call> ::= <id> <shape_tuple>
fn parse_object_call<'a>(tokens: &mut Lexer, diag: &mut Accumulator) -> BoxAST<'a> {
    let start = tokens.get_position();
    let id = consume_identifier(tokens, diag);

    let (object, span) = match tokens.peek().get_kind() {
        TokenKind::LeftPar => {
            let tuple = parse_shape_tuple(tokens, diag);

            let end = tokens.get_position();
            let span = Span::new(start, end);

            let object = ObjectCall::boxed(id, Some(tuple), span);

            (object, span)
        }
        _ => {
            let end = tokens.get_position();
            let span = Span::new(start, end);

            let object = ObjectCall::boxed(id, None, span);

            (object, span)
        }
    };

    object.check(tokens, diag, span);
    object
}

// <shape_tuple_generic> ::= "(" (<int_const> | <id>) "," (<int_const> | <id>) ")"
fn parse_shape_tuple_generic<'a>(tokens: &mut Lexer, diag: &mut Accumulator) -> BoxAST<'a> {
    let start = tokens.get_position();

    consume_keyword(tokens, TokenKind::LeftPar, diag);

    let mut left: BoxAST = GenericTupleOption::boxed_none();
    let mut right: BoxAST = GenericTupleOption::boxed_none();

    if match_next(tokens, TokenKind::Int) {
        let int_const = consume_int_const(tokens, diag);

        left = GenericTupleOption::boxed_int(int_const);
    } else if match_next(tokens, TokenKind::Ident) {
        let id = consume_identifier(tokens, diag);

        left = GenericTupleOption::boxed_id(id);
    }

    // <inner_stmt> ::= <object_decl>
    consume_keyword(tokens, TokenKind::Comma, diag);

    if match_next(tokens, TokenKind::Int) {
        let int_const = consume_int_const(tokens, diag);

        right = GenericTupleOption::boxed_int(int_const);
    } else if match_next(tokens, TokenKind::Ident) {
        let id = consume_identifier(tokens, diag);

        right = GenericTupleOption::boxed_id(id);
    }

    consume_keyword(tokens, TokenKind::RightPar, diag);
    let end = tokens.get_position();
    let span = Span::new(start, end);

    GenericTuple::boxed(left, right, span)
}

// <shape_tuple> ::= "(" <int_const> "," <int_const> ")"
fn parse_shape_tuple<'a>(tokens: &mut Lexer, diag: &mut Accumulator) -> BoxAST<'a> {
    let span_start = tokens.get_position();

    consume_keyword(tokens, TokenKind::LeftPar, diag);

    let left = consume_int_const(tokens, diag);

    consume_keyword(tokens, TokenKind::Comma, diag);

    let right = consume_int_const(tokens, diag);

    consume_keyword(tokens, TokenKind::RightPar, diag);

    let span_end = tokens.get_position();
    let span = Span::new(span_start, span_end);

    Tuple::boxed(left, right, span)
}

// <shape> ::= <shape_tuple> | <shape_tuple_compr> | <object_call> | <id>
fn parse_shape<'a>(tokens: &mut Lexer, diag: &mut Accumulator) -> BoxAST<'a> {
    let kind = tokens.peek().get_kind();

    match kind {
        TokenKind::LeftPar => {
            let lookahead = tokens.lookahead_by(4);

            if matches!(lookahead, TokenKind::Pipe) {
                let compr = parse_shape_tuple_compr(tokens, diag);

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
    let start = tokens.get_position();
    consume_keyword(tokens, TokenKind::ObjectColor, diag);

    consume_keyword(tokens, TokenKind::Colon, diag);

    let int_const = consume_int_const(tokens, diag);

    let end = tokens.get_position();
    let span = Span::new(start, end);

    let color = ObjectColor::boxed(int_const, span);
    color.check(tokens, diag, span);

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
    let start = tokens.get_position();
    consume_keyword(tokens, TokenKind::ObjectShape, diag);

    consume_keyword(tokens, TokenKind::Colon, diag);

    let shapes = parse_list_of_shapes(tokens, diag);

    let end = tokens.get_position();
    let span = Span::new(start, end);

    let object_shape = ObjectShape::boxed(shapes, span);
    object_shape.check(tokens, diag, span);

    object_shape
}

// <object_desc> ::= <object_shape> "," <object_color> | <object_color> "," <object_shape>
fn parse_object_desc<'a>(tokens: &mut Lexer, diag: &mut Accumulator) -> BoxAST<'a> {
    let kind = tokens.peek().get_kind();

    let start = tokens.get_position();

    let (node, span) = match kind {
        TokenKind::ObjectShape => {
            let shape = parse_object_shape(tokens, diag);
            consume_keyword(tokens, TokenKind::Comma, diag);
            let color = parse_object_color(tokens, diag);
            let end = tokens.get_position();
            let span = Span::new(start, end);

            (ObjectDesc::boxed(shape, color, span), span)
        }
        TokenKind::ObjectColor => {
            let color = parse_object_color(tokens, diag);
            consume_keyword(tokens, TokenKind::Comma, diag);
            let shape = parse_object_shape(tokens, diag);
            let end = tokens.get_position();
            let span = Span::new(start, end);

            (ObjectDesc::boxed(shape, color, span), span)
        }
        _ => {
            let shape = parse_object_shape(tokens, diag);
            consume_keyword(tokens, TokenKind::Comma, diag);
            let color = parse_object_color(tokens, diag);
            let end = tokens.get_position();
            let span = Span::new(start, end);

            (ObjectDesc::boxed(shape, color, span), span)
        }
    };

    node.check(tokens, diag, span);

    node
}

// <example_decl> ::= "example" <id> <inner_compound_stmt>
fn parse_problem_example<'a>(tokens: &mut Lexer, diag: &mut Accumulator) -> BoxAST<'a> {
    let start = tokens.get_position();

    consume_keyword(tokens, TokenKind::ProblemExample, diag);

    let id = consume_identifier(tokens, diag);

    let stmts = parse_inner_compound_stmt(tokens, diag);

    let end = tokens.get_position();
    let span = Span::new(start, end);

    let example = ProblemExample::boxed(id, stmts, span);
    example.check(tokens, diag, span);

    example
}

// <problem_solution> ::= "solution" <id> <inner_compound_stmt>
fn parse_problem_solution<'a>(tokens: &mut Lexer, diag: &mut Accumulator) -> BoxAST<'a> {
    let start = tokens.get_position();

    consume_keyword(tokens, TokenKind::ProblemSolution, diag);

    let id = consume_identifier(tokens, diag);

    let stmts = parse_inner_compound_stmt(tokens, diag);

    let end = tokens.get_position();
    let span = Span::new(start, end);

    let solution = ProblemSolution::boxed(id, stmts, span);
    solution.check(tokens, diag, span);

    solution
}

// <problem_input> ::= "input" <id> <tuple> <inner_compound_stmt>
fn parse_problem_input<'a>(tokens: &mut Lexer, diag: &mut Accumulator) -> BoxAST<'a> {
    let start = tokens.get_position();

    consume_keyword(tokens, TokenKind::ProblemInput, diag);

    let id = consume_identifier(tokens, diag);

    let stmts = parse_inner_compound_stmt(tokens, diag);

    let end = tokens.get_position();
    let span = Span::new(start, end);

    let input = ProblemInput::boxed(id, stmts, span);
    input.check(tokens, diag, span);

    input
}

// <problem_output> ::= "output" <id> <tuple> <inner_compound_stmt>
fn parse_problem_output<'a>(tokens: &mut Lexer, diag: &mut Accumulator) -> BoxAST<'a> {
    let start = tokens.get_position();

    consume_keyword(tokens, TokenKind::ProblemOutput, diag);

    let id = consume_identifier(tokens, diag);

    let stmts = parse_inner_compound_stmt(tokens, diag);

    let end = tokens.get_position();
    let span = Span::new(start, end);

    let output = ProblemOutput::boxed(id, stmts, span);
    output.check(tokens, diag, span);

    output
}

// <stmt> ::= <object_decl> | <example_decl> | <var_def> | <problem_solution> |
// <problem_input> | <problem_output>
fn parse_stmt<'a>(tokens: &mut Lexer, diag: &mut Accumulator) -> BoxAST<'a> {
    let kind = tokens.peek().get_kind();

    let node = match kind {
        TokenKind::Object => parse_object_decl(tokens, diag),
        TokenKind::ProblemExample => parse_problem_example(tokens, diag),
        TokenKind::ProblemSolution => parse_problem_solution(tokens, diag),
        TokenKind::ProblemInput => parse_problem_input(tokens, diag),
        TokenKind::ProblemOutput => parse_problem_output(tokens, diag),
        TokenKind::LetKwd => parse_var_def(tokens, diag),
        _ => parse_object_decl(tokens, diag),
    };

    node
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
    let start = tokens.get_position();

    consume_keyword(tokens, TokenKind::Object, diag);

    let id = consume_identifier(tokens, diag);

    let object_desc = parse_object_compound_desc(tokens, diag);

    consume_keyword(tokens, TokenKind::SemiColon, diag);

    let end = tokens.get_position();
    let span = Span::new(start, end);

    let object_decl = ObjectDecl::boxed(id, object_desc, span);
    object_decl.check(tokens, diag, span);
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
    let start = tokens.get_position();

    let id = consume_identifier(tokens, diag);

    let int_const = consume_int_const(tokens, diag);

    let stmts = parse_compound_stmt(tokens, diag);

    let end = tokens.get_position();
    let span = Span::new(start, end);

    let program = Program::boxed(id, int_const, stmts, span);
    program.check(tokens, diag, span);

    program
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
                println!("This is the error: {}", d);
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
        let text = "(x,y) | x <- [0..1}, y <- [0..1]";

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

    #[test]
    fn test_parse_example_decl() {
        let text = "example hello {

            draw(1, foo(0,1), a);

        }";

        check_no_err_single_ast(text, parse_stmt)
    }

    #[test]
    fn test_range() {
        let text = "object Shape { shape : [(0,1), (1,1)], color: 1 };";

        check_no_err_single_ast(text, parse_object_decl)
    }
}
