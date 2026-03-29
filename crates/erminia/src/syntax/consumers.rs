use crate::diag;
use crate::diagnostics::{DiagnosticAccumulator, Span};
use crate::lexer::lex::Lexer;
use crate::lexer::lex::PositionalOffset;
use crate::lexer::token::TokenKind;
use crate::types::ErminiaType;

// ==================================================================================== //
//  Utilities                                                                           //
// ==================================================================================== //

pub fn is_next_right_inclusive(
    tokens: &mut Lexer,
    diag: &mut DiagnosticAccumulator,
    start: PositionalOffset,
) -> ErminiaType {
    if tokens.is_poisoned() {
        return ErminiaType::Poisoned;
    }

    let token = tokens.token;

    let end = tokens.get_position();
    let span = Span::new(start, end);

    let res = match token.get_kind() {
        TokenKind::RightPar => ErminiaType::Bool(false),
        TokenKind::RightBracket => ErminiaType::Bool(true),
        TokenKind::Poisoned => {
            diag!(
                Lexer,
                E0002,
                ExpectedRightInclusive(token.text.to_string()),
                ConsiderChangingToInclusive,
                tokens,
                diag,
                span
            );

            ErminiaType::Poisoned
        }
        _ => {
            diag!(
                Parser,
                E0002,
                ExpectedRightInclusive(token.text.to_string()),
                ConsiderChangingToInclusive,
                tokens,
                diag,
                span
            );

            tokens.set_poisoned(true);

            tokens.loop_to_kind(TokenKind::SemiColon);

            return ErminiaType::Poisoned;
        }
    };

    tokens.advance();

    res
}

pub fn is_next_left_inclusive(
    tokens: &mut Lexer,
    diag: &mut DiagnosticAccumulator,
    start: PositionalOffset,
) -> ErminiaType {
    if tokens.is_poisoned() {
        return ErminiaType::Poisoned;
    }

    let token = tokens.token;

    let end = tokens.get_position();
    let span = Span::new(start, end);

    let res = match token.get_kind() {
        TokenKind::LeftPar => ErminiaType::Bool(false),
        TokenKind::LeftBracket => ErminiaType::Bool(true),
        TokenKind::Poisoned => {
            diag!(
                Lexer,
                E0002,
                ExpectedLeftInclusive(token.text.to_string()),
                ConsiderChangingToInclusive,
                tokens,
                diag,
                span
            );

            ErminiaType::Poisoned
        }
        _ => {
            diag!(
                Parser,
                E0002,
                ExpectedLeftInclusive(token.text.to_string()),
                ConsiderChangingToInclusive,
                tokens,
                diag,
                span
            );

            tokens.set_poisoned(true);

            tokens.loop_to_kind(TokenKind::SemiColon);

            return ErminiaType::Poisoned;
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
    diag: &mut DiagnosticAccumulator,
    start: PositionalOffset,
) -> ErminiaType {
    if tokens.is_poisoned() {
        return ErminiaType::Poisoned;
    }

    let token = tokens.token;

    let end = tokens.get_position();
    let span = Span::new(start, end);

    // TODO: Map TokenKind to ErminiaType
    println!("Consuming data type: {:?}", token.get_kind());
    let res = match token.get_kind() {
        TokenKind::Object => ErminiaType::Object,
        TokenKind::Poisoned => {
            diag!(
                Lexer,
                E0002,
                ExpectedDataType(token.text.to_string()),
                tokens,
                diag,
                span
            );

            ErminiaType::Poisoned
        }
        _ => {
            diag!(
                Parser,
                E0002,
                ExpectedDataType(token.text.to_string()),
                tokens,
                diag,
                span
            );

            tokens.set_poisoned(true);

            tokens.loop_to_kind(TokenKind::SemiColon);

            return ErminiaType::Poisoned;
        }
    };

    tokens.advance();

    res
}

pub fn consume_int_const(
    tokens: &mut Lexer,
    diag: &mut DiagnosticAccumulator,
    start: PositionalOffset,
) -> ErminiaType {
    if tokens.is_poisoned() {
        return ErminiaType::Poisoned;
    }

    let int_const = tokens.token;

    let end = tokens.get_position();
    let span = Span::new(start, end);

    let res = if int_const.get_kind() == TokenKind::Int {
        ErminiaType::Integer(int_const.text.parse::<i32>().unwrap())
    } else {
        diag!(
            Parser,
            E0003,
            ExpectedInteger(int_const.text.to_string()),
            tokens,
            diag,
            span
        );

        tokens.set_poisoned(true);

        tokens.loop_to_kind(TokenKind::SemiColon);

        return ErminiaType::Poisoned;
    };

    tokens.advance();

    res
}

pub fn consume_identifier(
    tokens: &mut Lexer,
    diag: &mut DiagnosticAccumulator,
    start: PositionalOffset,
) -> ErminiaType {
    if tokens.is_poisoned() {
        return ErminiaType::Poisoned;
    }

    let id = tokens.token;

    let end = tokens.get_position();
    let span = Span::new(start, end);

    let res = match id.get_kind() {
        TokenKind::Ident => ErminiaType::Ident(id.text.to_string()),
        _ => {
            diag!(
                Parser,
                E0001,
                ExpectedIdentifier(id.text.to_string()),
                tokens,
                diag,
                span
            );

            tokens.set_poisoned(true);

            tokens.loop_to_kind(TokenKind::SemiColon);

            return ErminiaType::Poisoned;
        }
    };

    tokens.advance();

    res
}

pub fn consume_keyword(
    tokens: &mut Lexer,
    expected: TokenKind,
    diag: &mut DiagnosticAccumulator,
    start: PositionalOffset,
) -> ErminiaType {
    if tokens.is_poisoned() {
        return ErminiaType::Poisoned;
    }

    let token = tokens.token;

    let end = tokens.get_position();
    let span = Span::new(start, end);

    let res = if token.get_kind() == expected {
        ErminiaType::Void
    } else {
        if let TokenKind::Poisoned = token.get_kind() {
            diag!(
                Lexer,
                E0001,
                ExpectedSomethingElse(expected.to_string(), token.text.to_string()),
                tokens,
                diag,
                span
            );

            return ErminiaType::Poisoned;
        }

        diag!(
            Parser,
            E0001,
            ExpectedSomethingElse(expected.to_string(), token.text.to_string()),
            tokens,
            diag,
            span
        );

        tokens.set_poisoned(true);

        tokens.loop_to_kind(TokenKind::SemiColon);

        return ErminiaType::Poisoned;
    };

    tokens.advance();

    res
}
