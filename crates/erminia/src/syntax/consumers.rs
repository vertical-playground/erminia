use crate::config::CompilerPass;
use crate::diagnostics::code::Code;
use crate::diagnostics::location::*;
use crate::diagnostics::messages::*;
use crate::diagnostics::DiagnosticBuilder;
use crate::lexer::lex::Lexer;
use crate::lexer::lex::PositionalOffset;
use crate::lexer::token::TokenKind;
use crate::types::ErminiaType;

type DB = DiagnosticBuilder;
const PARSER_PASS: CompilerPass = CompilerPass::Parser;

// ==================================================================================== //
//  Utilities                                                                           //
// ==================================================================================== //

pub fn is_next_right_inclusive(
    tokens: &mut Lexer,
    diag: &mut Accumulator,
    start: PositionalOffset,
) -> ErminiaType {
    let kind = tokens.peek().get_kind();

    let end = tokens.get_position();
    let span = Span::new(start, end);

    let res = match kind {
        TokenKind::RightPar => ErminiaType::Bool(false),
        TokenKind::RightBracket => ErminiaType::Bool(true),
        _ => {
            diag.add_diag(
                DB::build(PARSER_PASS, Code::E0002)
                    .with_note(Note::ExpectedRightInclusive)
                    .with_args(MessageKind::Note, vec![kind.to_string()])
                    .with_help(Help::ConsiderChangingToInclusive)
                    .emmit(tokens, span),
            );

            ErminiaType::Poisoned
        }
    };

    tokens.advance();

    res
}

pub fn is_next_left_inclusive(
    tokens: &mut Lexer,
    diag: &mut Accumulator,
    start: PositionalOffset,
) -> ErminiaType {
    let kind = tokens.peek().get_kind();

    let end = tokens.get_position();
    let span = Span::new(start, end);

    let res = match kind {
        TokenKind::LeftPar => ErminiaType::Bool(false),
        TokenKind::LeftBracket => ErminiaType::Bool(true),
        _ => {
            diag.add_diag(
                DB::build(PARSER_PASS, Code::E0002)
                    .with_note(Note::ExpectedLeftInclusive)
                    .with_args(MessageKind::Note, vec![kind.to_string()])
                    .emmit(tokens, span),
            );

            ErminiaType::Poisoned
        }
    };

    tokens.advance();

    res
}

pub fn next_is_comma(tokens: &mut Lexer) -> bool {
    matches!(tokens.peek().get_kind(), TokenKind::Comma)
}

pub fn next_is_expr(tokens: &mut Lexer) -> bool {
    matches!(tokens.peek().get_kind(), TokenKind::Ident | TokenKind::Int)
}

pub fn next_is_stmt(tokens: &mut Lexer) -> bool {
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

pub fn match_next(tokens: &mut Lexer, matched: TokenKind) -> bool {
    tokens.peek().get_kind() == matched
}

// ==================================================================================== //
//  Consumers                                                                           //
// ==================================================================================== //

// TODO: handle tuple & list types
pub fn consume_data_type(
    tokens: &mut Lexer,
    diag: &mut Accumulator,
    start: PositionalOffset,
) -> ErminiaType {
    let kind = tokens.peek().get_kind();

    let end = tokens.get_position();
    let span = Span::new(start, end);

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
                    .emmit(tokens, span),
            );

            ErminiaType::Poisoned
        }
    };

    tokens.advance();

    res
}

pub fn consume_int_const(
    tokens: &mut Lexer,
    diag: &mut Accumulator,
    start: PositionalOffset,
) -> ErminiaType {
    let int_const = tokens.token;

    let end = tokens.get_position();
    let span = Span::new(start, end);

    let res = if int_const.get_kind() == TokenKind::Int {
        ErminiaType::Integer(int_const.text.parse::<i32>().unwrap())
    } else {
        diag.add_diag(
            DB::build(PARSER_PASS, Code::E0003)
                .with_note(Note::ExpectedInteger)
                .with_args(MessageKind::Note, vec![int_const.get_kind().to_string()])
                .emmit(tokens, span),
        );

        ErminiaType::Poisoned
    };

    tokens.advance();

    res
}

pub fn consume_identifier(
    tokens: &mut Lexer,
    diag: &mut Accumulator,
    start: PositionalOffset,
) -> ErminiaType {
    let id = tokens.token;

    let end = tokens.get_position();
    let span = Span::new(start, end);

    let res = match id.get_kind() {
        TokenKind::Ident => ErminiaType::Ident(id.text.to_string()),
        _ => {
            diag.add_diag(
                DB::build(PARSER_PASS, Code::E0001)
                    .with_note(Note::ExpectedIdentifier)
                    .with_args(MessageKind::Note, vec![id.get_kind().to_string()])
                    .emmit(tokens, span),
            );

            ErminiaType::Poisoned
        }
    };

    tokens.advance();

    res
}

pub fn consume_keyword(
    tokens: &mut Lexer,
    expected: TokenKind,
    diag: &mut Accumulator,
    start: PositionalOffset,
) -> ErminiaType {
    let actual = tokens.peek().get_kind();

    let end = tokens.get_position();
    let span = Span::new(start, end);

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
                .emmit(tokens, span),
        );

        ErminiaType::Poisoned
    };

    tokens.advance();

    res
}
