use crate::ast::ast::BoxAST;

use crate::ast::expr::*;
use crate::ast::stmt::*;
use crate::diag;
use crate::diagnostics::{DiagnosticAccumulator, Span};
use crate::lexer::lex::Lexer;
use crate::lexer::lex::PositionalOffset;
use crate::lexer::token::TokenKind;
use crate::syntax::consumers::*;
use crate::types::ErminiaType;

// ==================================================================================== //
// Parsers                                                                              //
// ==================================================================================== //

// <expr> ::= <object_call> | <id> | <int_const>
pub fn parse_expr<'a>(
    tokens: &mut Lexer,
    diag: &mut DiagnosticAccumulator,
    start: PositionalOffset,
) -> BoxAST<'a> {
    let kind = tokens.peek().get_kind();

    match kind {
        TokenKind::Ident => {
            let lookahead = tokens.lookahead();

            if matches!(lookahead.0, TokenKind::LeftPar) {
                parse_object_call(tokens, diag)
            } else {
                let id = consume_identifier(tokens, diag, start);
                RValue::boxed_id(id.to_string())
            }
        }
        TokenKind::Int => RValue::boxed_int(consume_int_const(tokens, diag, start).to_int()),
        _ => {
            let line = tokens.get_position().get_line();
            let span = tokens.get_line_span(line);

            diag!(
                Parser,
                E0001,
                ExpectedIDorInteger(kind.to_string()),
                tokens,
                diag,
                span
            );

            parse_object_call(tokens, diag)
        }
    }
}

// <list_of_exprs> ::= <expr> ("," <expr>)*
pub fn parse_list_of_exprs<'a>(
    tokens: &mut Lexer,
    diag: &mut DiagnosticAccumulator,
) -> (Vec<BoxAST<'a>>, Vec<ErminiaType>) {
    let mut syntax: Vec<ErminiaType> = vec![];

    let start = tokens.get_previous_position();

    let mut exprs: Vec<BoxAST> = vec![];

    while next_is_expr(tokens) {
        let expr = parse_expr(tokens, diag, start);

        exprs.push(expr);

        let next = tokens.peek().get_kind();

        if matches!(next, TokenKind::Comma) {
            syntax.push(consume_keyword(tokens, TokenKind::Comma, diag, start));
        }
    }

    (exprs, syntax)
}

// <func_call> ::= <id> "(" [<list_of_exprs>] ")" ";"
pub fn parse_func_call<'a>(tokens: &mut Lexer, diag: &mut DiagnosticAccumulator) -> BoxAST<'a> {
    let mut syntax: Vec<ErminiaType> = vec![];

    let start = tokens.get_previous_position();

    let id = consume_identifier(tokens, diag, start);

    syntax.push(consume_keyword(tokens, TokenKind::LeftPar, diag, start));

    let (exprs, inner_syntax) = parse_list_of_exprs(tokens, diag);

    syntax.extend(inner_syntax);

    syntax.push(consume_keyword(tokens, TokenKind::RightPar, diag, start));

    tokens.set_poisoned(false);

    syntax.push(consume_keyword(tokens, TokenKind::SemiColon, diag, start));

    let end = tokens.get_position();
    let span = Span::new(start, end);

    FuncCall::boxed(id, exprs, span, syntax)
}

// <inner_stmt> ::= <object_decl> | <var_def> | <func_call>
pub fn parse_inner_stmt<'a>(tokens: &mut Lexer, diag: &mut DiagnosticAccumulator) -> BoxAST<'a> {
    let kind = tokens.peek().get_kind();

    match kind {
        TokenKind::Object => parse_object_decl(tokens, diag),
        TokenKind::LetKwd => parse_var_def(tokens, diag),
        TokenKind::Ident => parse_func_call(tokens, diag),
        TokenKind::ProblemInput => parse_problem_input(tokens, diag),
        TokenKind::ProblemOutput => parse_problem_output(tokens, diag),
        _ => parse_object_decl(tokens, diag),
    }
}

// <inner_stmt_list> ::= (<inner_stmt>)*
pub fn parse_inner_stmt_list<'a>(
    tokens: &mut Lexer,
    diag: &mut DiagnosticAccumulator,
) -> Vec<BoxAST<'a>> {
    let mut stmts: Vec<BoxAST> = vec![];
    while next_is_stmt(tokens) {
        let stmt = parse_inner_stmt(tokens, diag);
        stmts.push(stmt);
    }
    stmts
}

// <inner_compound_stmt> ::= "{" [<inner_stmt_list>] "}"
pub fn parse_inner_compound_stmt<'a>(
    tokens: &mut Lexer,
    diag: &mut DiagnosticAccumulator,
) -> (Vec<BoxAST<'a>>, Vec<ErminiaType>) {
    let mut syntax: Vec<ErminiaType> = vec![];

    let start = tokens.get_previous_position();

    syntax.push(consume_keyword(tokens, TokenKind::LeftBrace, diag, start));
    let stmts = parse_inner_stmt_list(tokens, diag);
    syntax.push(consume_keyword(tokens, TokenKind::RightBrace, diag, start));
    (stmts, syntax)
}

// TODO: handle type inference
// <var_def> ::= "let" <id> ":" <data_type> "=" <expr> ";"
pub fn parse_var_def<'a>(tokens: &mut Lexer, diag: &mut DiagnosticAccumulator) -> BoxAST<'a> {
    let mut syntax: Vec<ErminiaType> = vec![];

    let start = tokens.get_previous_position();

    let mut data_type: ErminiaType = ErminiaType::default();

    syntax.push(consume_keyword(tokens, TokenKind::LetKwd, diag, start));

    let id = consume_identifier(tokens, diag, start);

    if match_next(tokens, TokenKind::Colon) {
        syntax.push(consume_keyword(tokens, TokenKind::Colon, diag, start));

        // change here if it's explicit about data type
        data_type = consume_data_type(tokens, diag, start);
    }

    syntax.push(consume_keyword(tokens, TokenKind::Equals, diag, start));

    let expr = parse_expr(tokens, diag, start);

    tokens.set_poisoned(false);

    syntax.push(consume_keyword(tokens, TokenKind::SemiColon, diag, start));

    let end = tokens.get_position();
    let span = Span::new(start, end);

    VarDef::boxed(id, data_type, expr, span, syntax)
}

// <range> ::= ("[" | "(") <int_const> ".." <int_const> ("]" | ")")
pub fn parse_range<'a>(tokens: &mut Lexer, diag: &mut DiagnosticAccumulator) -> BoxAST<'a> {
    let mut syntax: Vec<ErminiaType> = vec![];

    let start = tokens.get_previous_position();

    let is_left_inclusive = is_next_left_inclusive(tokens, diag, start);

    let left = consume_int_const(tokens, diag, start);

    syntax.push(consume_keyword(tokens, TokenKind::Range, diag, start));

    let right = consume_int_const(tokens, diag, start);

    let is_right_inclusive = is_next_right_inclusive(tokens, diag, start);

    let end = tokens.get_position();

    let span = Span::new(start, end);

    Range::boxed(
        is_left_inclusive,
        is_right_inclusive,
        left,
        right,
        span,
        syntax,
    )
}

// <shape_tuple_iter> ::= <id> "<-" <range>
pub fn parse_shape_tuple_iter<'a>(
    tokens: &mut Lexer,
    diag: &mut DiagnosticAccumulator,
) -> BoxAST<'a> {
    let mut syntax: Vec<ErminiaType> = vec![];

    let start = tokens.get_previous_position();
    let coord = consume_identifier(tokens, diag, start);

    syntax.push(consume_keyword(tokens, TokenKind::LeftArrow, diag, start));

    let range = parse_range(tokens, diag);

    let end = tokens.get_position();
    let span = Span::new(start, end);

    TupleIterator::boxed(coord, range, span, syntax)
}

// <shape_tuple_iter_pair> ::= <shape_tuple_iter> ("," <shape_tuple_iter>)
pub fn parse_shape_tuple_iter_pair<'a>(
    tokens: &mut Lexer,
    diag: &mut DiagnosticAccumulator,
) -> (Vec<BoxAST<'a>>, Vec<ErminiaType>) {
    let mut syntax: Vec<ErminiaType> = vec![];

    let start = tokens.get_previous_position();

    let mut pairs: Vec<BoxAST> = vec![];

    let first_tuple_iter = parse_shape_tuple_iter(tokens, diag);

    pairs.push(first_tuple_iter);

    if next_is_comma(tokens) {
        syntax.push(consume_keyword(tokens, TokenKind::Comma, diag, start));
        let second_tuple_iter = parse_shape_tuple_iter(tokens, diag);
        pairs.push(second_tuple_iter);
    }

    (pairs, syntax)
}

// <shape_tuple_compr> ::= <shape_tuple> "|" <shape_tuple_iter_pair>
pub fn parse_shape_tuple_compr<'a>(
    tokens: &mut Lexer,
    diag: &mut DiagnosticAccumulator,
) -> BoxAST<'a> {
    let mut syntax: Vec<ErminiaType> = vec![];

    let start = tokens.get_previous_position();

    let tuple = parse_shape_tuple_generic(tokens, diag);

    syntax.push(consume_keyword(tokens, TokenKind::Pipe, diag, start));

    let (iter_pair, inner_syntax) = parse_shape_tuple_iter_pair(tokens, diag);

    syntax.extend(inner_syntax);

    let end = tokens.get_position();
    let span = Span::new(start, end);

    TupleComprehension::boxed(tuple, iter_pair, span, syntax)
}

// <object_call> ::= <id> <shape_tuple>
pub fn parse_object_call<'a>(tokens: &mut Lexer, diag: &mut DiagnosticAccumulator) -> BoxAST<'a> {
    let syntax: Vec<ErminiaType> = vec![];

    let start = tokens.get_previous_position();

    let id = consume_identifier(tokens, diag, start);

    match tokens.peek().get_kind() {
        TokenKind::LeftPar => {
            let tuple = parse_shape_tuple(tokens, diag);

            let end = tokens.get_position();
            let span = Span::new(start, end);

            ObjectCall::boxed(id, Some(tuple), span, syntax)
        }
        _ => {
            let end = tokens.get_position();
            let span = Span::new(start, end);

            ObjectCall::boxed(id, None, span, syntax)
        }
    }
}

// <shape_tuple_generic> ::= "(" (<int_const> | <id>) "," (<int_const> | <id>) ")"
pub fn parse_shape_tuple_generic<'a>(
    tokens: &mut Lexer,
    diag: &mut DiagnosticAccumulator,
) -> BoxAST<'a> {
    let mut syntax: Vec<ErminiaType> = vec![];

    let start = tokens.get_previous_position();

    syntax.push(consume_keyword(tokens, TokenKind::LeftPar, diag, start));

    let left: BoxAST;
    let right: BoxAST;

    if match_next(tokens, TokenKind::Int) {
        let int_const = consume_int_const(tokens, diag, start);

        left = GenericTupleOption::boxed_int(int_const);
    } else if match_next(tokens, TokenKind::Ident) {
        let id = consume_identifier(tokens, diag, start);

        left = GenericTupleOption::boxed_id(id);
    } else {
        diag!(
            Parser,
            E0003,
            ExpectedIDorInteger(tokens.peek().get_kind().to_string()),
            tokens,
            diag,
            Span::default()
        );

        left = GenericTupleOption::boxed_int(ErminiaType::Poisoned);
    }

    syntax.push(consume_keyword(tokens, TokenKind::Comma, diag, start));

    if match_next(tokens, TokenKind::Int) {
        let int_const = consume_int_const(tokens, diag, start);

        right = GenericTupleOption::boxed_int(int_const);
    } else if match_next(tokens, TokenKind::Ident) {
        let id = consume_identifier(tokens, diag, start);

        right = GenericTupleOption::boxed_id(id);
    } else {
        diag!(
            Parser,
            E0003,
            ExpectedIDorInteger(tokens.peek().get_kind().to_string()),
            tokens,
            diag,
            Span::default()
        );

        right = GenericTupleOption::boxed_int(ErminiaType::Poisoned);
    }

    syntax.push(consume_keyword(tokens, TokenKind::RightPar, diag, start));

    let end = tokens.get_position();
    let span = Span::new(start, end);

    GenericTuple::boxed(left, right, span, syntax)
}

// <shape_tuple> ::= "(" <int_const> "," <int_const> ")"
pub fn parse_shape_tuple<'a>(tokens: &mut Lexer, diag: &mut DiagnosticAccumulator) -> BoxAST<'a> {
    let mut syntax: Vec<ErminiaType> = vec![];

    let start = tokens.get_previous_position();

    syntax.push(consume_keyword(tokens, TokenKind::LeftPar, diag, start));

    let left = consume_int_const(tokens, diag, start);

    syntax.push(consume_keyword(tokens, TokenKind::Comma, diag, start));

    let right = consume_int_const(tokens, diag, start);

    syntax.push(consume_keyword(tokens, TokenKind::RightPar, diag, start));

    let end = tokens.get_position();

    let span = Span::new(start, end);

    Tuple::boxed(left, right, span, syntax)
}

// <shape> ::= <shape_tuple> | <shape_tuple_compr> | <object_call> | <id>
pub fn parse_shape<'a>(tokens: &mut Lexer, diag: &mut DiagnosticAccumulator) -> BoxAST<'a> {
    let kind = tokens.peek().get_kind();

    match kind {
        TokenKind::LeftPar => {
            let lookahead = tokens.lookahead_by(4);

            if matches!(lookahead, TokenKind::Pipe) {
                let compr = parse_shape_tuple_compr(tokens, diag);

                return compr;
            }

            parse_shape_tuple(tokens, diag)
        }
        TokenKind::Ident => parse_object_call(tokens, diag),
        _ => {
            diag!(
                Parser,
                E0003,
                ExpectedTypeofTuple(kind.to_string()),
                tokens,
                diag,
                Span::default()
            );

            //TODO: Do some sort of skip here
            while !match_next(tokens, TokenKind::SemiColon) {
                tokens.advance();
            }

            PoisonedStmt::boxed(Span::default())
        }
    }
}

// <object_color> ::= "color" ":" <int_const>
pub fn parse_object_color<'a>(tokens: &mut Lexer, diag: &mut DiagnosticAccumulator) -> BoxAST<'a> {
    let mut syntax: Vec<ErminiaType> = vec![];

    let start = tokens.get_previous_position();

    syntax.push(consume_keyword(tokens, TokenKind::ObjectColor, diag, start));

    syntax.push(consume_keyword(tokens, TokenKind::Colon, diag, start));

    let int_const = consume_int_const(tokens, diag, start);

    let end = tokens.get_position();
    let span = Span::new(start, end);

    ObjectColor::boxed(int_const, span, syntax)
}

// <list_of_shapes> ::= "[" <shape> ("," <shape>)* "]"
pub fn parse_list_of_shapes<'a>(
    tokens: &mut Lexer,
    diag: &mut DiagnosticAccumulator,
) -> (Vec<BoxAST<'a>>, Vec<ErminiaType>) {
    let mut syntax: Vec<ErminiaType> = vec![];

    let start = tokens.get_previous_position();

    let mut shapes: Vec<BoxAST> = vec![];

    syntax.push(consume_keyword(tokens, TokenKind::LeftBracket, diag, start));

    let shape = parse_shape(tokens, diag);

    shapes.push(shape);

    while next_is_comma(tokens) {
        syntax.push(consume_keyword(tokens, TokenKind::Comma, diag, start));

        let shape = parse_shape(tokens, diag);

        shapes.push(shape);
    }
    syntax.push(consume_keyword(
        tokens,
        TokenKind::RightBracket,
        diag,
        start,
    ));

    (shapes, syntax)
}

// <object_shape> ::= "shape" ":" <list_of_shapes>
pub fn parse_object_shape<'a>(tokens: &mut Lexer, diag: &mut DiagnosticAccumulator) -> BoxAST<'a> {
    let mut syntax: Vec<ErminiaType> = vec![];

    let start = tokens.get_previous_position();

    syntax.push(consume_keyword(tokens, TokenKind::ObjectShape, diag, start));

    syntax.push(consume_keyword(tokens, TokenKind::Colon, diag, start));

    let (shapes, inner_syntax) = parse_list_of_shapes(tokens, diag);

    syntax.extend(inner_syntax);

    let end = tokens.get_position();
    let span = Span::new(start, end);

    ObjectShape::boxed(shapes, span, syntax)
}

// <object_desc> ::= <object_shape> "," <object_color> | <object_color> "," <object_shape>
pub fn parse_object_desc<'a>(tokens: &mut Lexer, diag: &mut DiagnosticAccumulator) -> BoxAST<'a> {
    let mut syntax: Vec<ErminiaType> = vec![];

    let kind = tokens.peek().get_kind();

    let start = tokens.get_previous_position();

    match kind {
        TokenKind::ObjectShape => {
            let shape = parse_object_shape(tokens, diag);
            syntax.push(consume_keyword(tokens, TokenKind::Comma, diag, start));
            let color = parse_object_color(tokens, diag);
            let end = tokens.get_position();
            let span = Span::new(start, end);

            ObjectDesc::boxed(shape, color, span, syntax)
        }
        TokenKind::ObjectColor => {
            let color = parse_object_color(tokens, diag);
            syntax.push(consume_keyword(tokens, TokenKind::Comma, diag, start));
            let shape = parse_object_shape(tokens, diag);
            let end = tokens.get_position();
            let span = Span::new(start, end);

            ObjectDesc::boxed(shape, color, span, syntax)
        }
        _ => {
            diag!(
                Parser,
                E0003,
                ExpectedShapeOrColor(kind.to_string()),
                tokens,
                diag,
                Span::default()
            );

            PoisonedStmt::boxed(Span::default())
        }
    }
}

// <example_decl> ::= "example" <id> '(' <int_const> ')' <inner_compound_stmt> ';'
pub fn parse_problem_example<'a>(
    tokens: &mut Lexer,
    diag: &mut DiagnosticAccumulator,
) -> BoxAST<'a> {
    let mut syntax: Vec<ErminiaType> = vec![];

    let start = tokens.get_previous_position();

    syntax.push(consume_keyword(
        tokens,
        TokenKind::ProblemExample,
        diag,
        start,
    ));

    let id = consume_identifier(tokens, diag, start);

    syntax.push(consume_keyword(tokens, TokenKind::LeftPar, diag, start));

    let int_const = consume_int_const(tokens, diag, start);

    syntax.push(consume_keyword(tokens, TokenKind::RightPar, diag, start));

    let (stmts, inner_syntax) = parse_inner_compound_stmt(tokens, diag);

    syntax.extend(inner_syntax);

    tokens.set_poisoned(false);

    syntax.push(consume_keyword(tokens, TokenKind::SemiColon, diag, start));

    let end = tokens.get_position();
    let span = Span::new(start, end);

    ProblemExample::boxed(id, int_const, stmts, span, syntax)
}

// <problem_solution> ::= "solution" <id> '(' <int_const> ')' <inner_compound_stmt> ';'
pub fn parse_problem_solution<'a>(
    tokens: &mut Lexer,
    diag: &mut DiagnosticAccumulator,
) -> BoxAST<'a> {
    let mut syntax: Vec<ErminiaType> = vec![];

    let start = tokens.get_previous_position();

    syntax.push(consume_keyword(
        tokens,
        TokenKind::ProblemSolution,
        diag,
        start,
    ));

    let id = consume_identifier(tokens, diag, start);

    syntax.push(consume_keyword(tokens, TokenKind::LeftPar, diag, start));

    let int_const = consume_int_const(tokens, diag, start);

    syntax.push(consume_keyword(tokens, TokenKind::RightPar, diag, start));

    let (stmts, inner_syntax) = parse_inner_compound_stmt(tokens, diag);

    syntax.extend(inner_syntax);

    tokens.set_poisoned(false);

    syntax.push(consume_keyword(tokens, TokenKind::SemiColon, diag, start));

    let end = tokens.get_position();
    let span = Span::new(start, end);

    ProblemSolution::boxed(id, int_const, stmts, span, syntax)
}

// <problem_input> ::= "input" <id> <tuple> <inner_compound_stmt> ';'
pub fn parse_problem_input<'a>(tokens: &mut Lexer, diag: &mut DiagnosticAccumulator) -> BoxAST<'a> {
    let mut syntax: Vec<ErminiaType> = vec![];

    let start = tokens.get_previous_position();

    syntax.push(consume_keyword(
        tokens,
        TokenKind::ProblemInput,
        diag,
        start,
    ));

    let id = consume_identifier(tokens, diag, start);

    let tuple = parse_shape_tuple(tokens, diag);

    let (stmts, inner_syntax) = parse_inner_compound_stmt(tokens, diag);

    syntax.extend(inner_syntax);

    tokens.set_poisoned(false);

    syntax.push(consume_keyword(tokens, TokenKind::SemiColon, diag, start));

    let end = tokens.get_position();
    let span = Span::new(start, end);

    ProblemInput::boxed(id, tuple, stmts, span, syntax)
}

// <problem_output> ::= "output" <id> <tuple> <inner_compound_stmt> ';'
pub fn parse_problem_output<'a>(
    tokens: &mut Lexer,
    diag: &mut DiagnosticAccumulator,
) -> BoxAST<'a> {
    let mut syntax: Vec<ErminiaType> = vec![];
    let start = tokens.get_previous_position();

    syntax.push(consume_keyword(
        tokens,
        TokenKind::ProblemOutput,
        diag,
        start,
    ));

    let id = consume_identifier(tokens, diag, start);

    let tuple = parse_shape_tuple(tokens, diag);

    let (stmts, inner_syntax) = parse_inner_compound_stmt(tokens, diag);

    syntax.extend(inner_syntax);

    tokens.set_poisoned(false);

    syntax.push(consume_keyword(tokens, TokenKind::SemiColon, diag, start));

    let end = tokens.get_position();
    let span = Span::new(start, end);

    ProblemOutput::boxed(id, tuple, stmts, span, syntax)
}

// <stmt> ::= <object_decl> | <example_decl> | <var_def> | <problem_solution> |
// <problem_input> | <problem_output>
pub fn parse_stmt<'a>(
    tokens: &mut Lexer,
    diag: &mut DiagnosticAccumulator,
    _start: PositionalOffset,
) -> BoxAST<'a> {
    let kind = tokens.peek().get_kind();

    let node = match kind {
        TokenKind::Object => parse_object_decl(tokens, diag),
        TokenKind::ProblemExample => parse_problem_example(tokens, diag),
        TokenKind::ProblemSolution => parse_problem_solution(tokens, diag),
        TokenKind::ProblemInput => parse_problem_input(tokens, diag),
        TokenKind::ProblemOutput => parse_problem_output(tokens, diag),
        TokenKind::LetKwd => parse_var_def(tokens, diag),
        _ => {
            let span = tokens.get_line_span(tokens.get_position().get_line());

            diag!(
                Parser,
                E0002,
                ExpectedStatement(tokens.peek().get_text().to_string()),
                DidYouMeanStmtKeyword,
                tokens,
                diag,
                span
            );

            // TODO: Do some sort of skip here
            while !match_next(tokens, TokenKind::SemiColon) {
                tokens.advance();
            }

            // Return a poisoned node
            PoisonedStmt::boxed(span)
        }
    };

    node
}

// <stmts_list> ::= (<stmt>)*
pub fn parse_stmt_list<'a>(
    tokens: &mut Lexer,
    diag: &mut DiagnosticAccumulator,
) -> Vec<BoxAST<'a>> {
    let mut stmts: Vec<BoxAST> = vec![];

    let start = tokens.get_previous_position();

    while next_is_stmt(tokens) {
        let stmt = parse_stmt(tokens, diag, start);
        stmts.push(stmt);
    }

    stmts
}

// <object_compound_desc> ::= "{" <object_desc> "}"
pub fn parse_object_compound_desc<'a>(
    tokens: &mut Lexer,
    diag: &mut DiagnosticAccumulator,
) -> (BoxAST<'a>, Vec<ErminiaType>) {
    let mut syntax: Vec<ErminiaType> = vec![];

    let start = tokens.get_previous_position();

    syntax.push(consume_keyword(tokens, TokenKind::LeftBrace, diag, start));
    let object_desc = parse_object_desc(tokens, diag);
    syntax.push(consume_keyword(tokens, TokenKind::RightBrace, diag, start));
    (object_desc, syntax)
}

// <object_decl> ::= "object" <id> <object_compound_desc> ";"
pub fn parse_object_decl<'a>(tokens: &mut Lexer, diag: &mut DiagnosticAccumulator) -> BoxAST<'a> {
    let mut syntax: Vec<ErminiaType> = vec![];

    let start = tokens.get_previous_position();

    syntax.push(consume_keyword(tokens, TokenKind::Object, diag, start));

    let id = consume_identifier(tokens, diag, start);

    let (object_desc, inner_syntax) = parse_object_compound_desc(tokens, diag);

    syntax.extend(inner_syntax);

    tokens.set_poisoned(false);

    syntax.push(consume_keyword(tokens, TokenKind::SemiColon, diag, start));

    let end = tokens.get_position();
    let span = Span::new(start, end);

    let object_decl = ObjectDecl::boxed(id, object_desc, span, syntax);

    object_decl
}

// <compound_stmt> ::= "{" [<stmt_list>] "}"
pub fn parse_compound_stmt<'a>(
    tokens: &mut Lexer,
    diag: &mut DiagnosticAccumulator,
) -> (Vec<BoxAST<'a>>, Vec<ErminiaType>) {
    let mut syntax: Vec<ErminiaType> = vec![];

    let start = tokens.get_previous_position();

    syntax.push(consume_keyword(tokens, TokenKind::LeftBrace, diag, start));

    let stmts = parse_stmt_list(tokens, diag);

    syntax.push(consume_keyword(tokens, TokenKind::RightBrace, diag, start));

    (stmts, syntax)
}

// <problem_declaration> ::= "def" <id> "(" <int_const> ")" <compound_stmt>
pub fn parse_problem_decl<'a>(tokens: &mut Lexer, diag: &mut DiagnosticAccumulator) -> BoxAST<'a> {
    let mut syntax: Vec<ErminiaType> = vec![];

    let start = tokens.get_previous_position();

    syntax.push(consume_keyword(tokens, TokenKind::ProblemDef, diag, start));

    let id = consume_identifier(tokens, diag, start);

    syntax.push(consume_keyword(tokens, TokenKind::LeftPar, diag, start));

    let int_const = consume_int_const(tokens, diag, start);

    syntax.push(consume_keyword(tokens, TokenKind::RightPar, diag, start));

    let (stmts, inner_syntax) = parse_compound_stmt(tokens, diag);

    syntax.extend(inner_syntax);

    let end = tokens.get_position();

    let span = Span::new(start, end);

    let program = Program::boxed(id, int_const, stmts, span, syntax);

    program.check_poisoning(tokens, diag);

    program
}

// <program> ::= <problem_declaration>
pub fn parse_program<'a>(tokens: &mut Lexer, diag: &mut DiagnosticAccumulator) -> BoxAST<'a> {
    tokens.advance();

    let program = parse_problem_decl(tokens, diag);

    program
}
