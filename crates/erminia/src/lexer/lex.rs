use std::fmt;
use std::str::FromStr;

use crate::diagnostics::diagnostics::Location;
use crate::error::lexer_error::{LexerError, LexerResult};
use crate::lexer::token::*;

static KEYWORDS: [&'static str; 9] = [
    "def",
    "let",
    "object",
    "superobject",
    "shape",
    "color",
    "example",
    "input",
    "output",
];

// static OPERATORS: [&str; 26] = [
//     "+", "-", "*", "/", "//", "%", "<<", ">>", "<", ">", ".", "!", "!=", "=", "(", ")", "[", "]",
//     "{", "}", ",", ";", ":", "..", "(*", "*)",
// ];

// ==================================================================================== //
// Positional Offset Struct                                                             //
// ==================================================================================== //

#[derive(Debug, Clone, Copy)]
pub struct PositionalOffset {
    pos: usize,
    cursor: usize,
    line: usize,
}

impl Default for PositionalOffset {
    fn default() -> Self {
        PositionalOffset {
            pos: 0,
            cursor: 1,
            line: 1,
        }
    }
}

impl PositionalOffset {
    // fn new(pos: usize, cursor: usize, line: usize) -> PositionalOffset {
    //     PositionalOffset {
    //         pos: pos,
    //         cursor: cursor,
    //         line: line,
    //     }
    // }

    fn increment_pos(&mut self, val: usize) {
        self.pos += val;
    }

    fn increment_cursor(&mut self, val: usize) {
        self.cursor += val;
    }

    fn increment_line(&mut self, val: usize) {
        self.line += val;
    }

    // fn reset_pos(&mut self) {
    //     self.pos = 0;
    // }

    fn reset_cursor(&mut self) {
        self.cursor = 1;
    }

    // fn reset_line(&mut self) {
    //     self.line = 1;
    // }

    fn get_cursor(&self) -> usize {
        self.pos
    }

    // fn get_position(&self) -> usize {
    //     self.pos
    // }

    fn get_line(&self) -> usize {
        self.line
    }
}

impl fmt::Display for PositionalOffset {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = format!(
            "PositionalOffset: (cursor: {}, position: {}, line: {})",
            self.cursor, self.pos, self.line
        );
        fmt::Display::fmt(&s, f)
    }
}

// ==================================================================================== //
// Lexer Struct                                                                         //
// ==================================================================================== //

pub struct Lexer<'input> {
    content: &'input str,
    start: PositionalOffset,
    pub token: Token<'input>,
}

impl Default for Lexer<'_> {
    fn default() -> Self {
        Lexer {
            content: "",
            start: PositionalOffset::default(),
            token: Token::default(),
        }
    }
}

impl<'input> Lexer<'input> {
    pub fn new(content: &'input str) -> Lexer<'input> {
        Lexer {
            content: content,
            start: PositionalOffset::default(),
            token: Token::default(),
        }
    }

    pub fn peek(&mut self) -> Token {
        self.token
    }

    pub fn advance(&mut self) -> LexerResult<()> {
        let start_pos = trim_starting_whitespace(self.content, self.start);

        let (kind, end_pos) = get_next_token_kind(self.content, start_pos)?;

        let lexeme = &self.content[start_pos.pos..end_pos.pos];

        let token = Token::new(kind, lexeme, start_pos.get_line(), start_pos.get_cursor());

        self.start = end_pos;

        self.token = token;

        Ok(())
    }

    pub fn lookahead(&self) -> LexerResult<(TokenKind, PositionalOffset)> {
        let start_pos = trim_starting_whitespace(self.content, self.start);

        let (kind, end_pos) = get_next_token_kind(self.content, start_pos)?;

        Ok((kind, end_pos))
    }

    pub fn lookahead2(
        &self,
    ) -> LexerResult<(TokenKind, TokenKind, PositionalOffset, PositionalOffset)> {
        let first_start_pos = trim_starting_whitespace(self.content, self.start);

        let (first, first_end_pos) = get_next_token_kind(self.content, first_start_pos)?;

        let start_pos = trim_starting_whitespace(self.content, first_end_pos);

        let (second, second_end_pos) = get_next_token_kind(self.content, start_pos)?;

        Ok((first, second, first_end_pos, second_end_pos))
    }

    pub fn lex_with_separate_pass(&mut self) -> LexerResult<Vec<Token>> {
        let mut tokens: Vec<Token> = Vec::new();

        loop {
            let (token, pos) = &_advance(self.content, self.start)?;

            self.set_start(*pos);

            tokens.push(*token);

            if token.get_kind() == TokenKind::EOF {
                break;
            }
        }

        Ok(tokens)
    }

    fn set_start(&mut self, pos: PositionalOffset) {
        self.start = pos
    }
}

// ==================================================================================== //
// Lexer Utilities                                                                      //
// ==================================================================================== //

fn _advance(text: &str, po: PositionalOffset) -> LexerResult<(Token, PositionalOffset)> {
    let start_pos = trim_starting_whitespace(text, po);

    let (kind, end_pos) = get_next_token_kind(text, start_pos)?;

    let lexeme = &text[start_pos.pos..end_pos.pos];

    let token = Token::new(kind, lexeme, start_pos.get_line(), start_pos.get_cursor());

    Ok((token, end_pos))
}

fn trim_starting_whitespace(text: &str, mut pos: PositionalOffset) -> PositionalOffset {
    let starting_text = &text[pos.pos..];

    let mut chars = starting_text.chars();

    while let Some(c) = chars.next() {
        match c {
            ' ' | '\t' => {
                pos.increment_pos(1);
                pos.increment_cursor(1);
            }
            '\n' => {
                pos.increment_pos(1);
                pos.increment_line(1);
                pos.reset_cursor();
            }
            '\r' if matches!(chars.next(), Some('\n')) => {
                pos.increment_pos(2);
                pos.increment_line(1);
                pos.reset_cursor();
            }
            _ => break,
        }
    }

    pos
}

fn slice_from_position(text: &str, pos: PositionalOffset) -> LexerResult<&str> {
    if pos.pos > text.len() {
        Err(LexerError::TokenError(Location::new(Position::default())))
    } else {
        Ok(&text[pos.pos..])
    }
}

fn get_next_keyword(
    text: &str,
    keywords: &[&str],
    mut pos: PositionalOffset,
) -> Option<(TokenKind, PositionalOffset)> {
    let starting_text = &text[pos.pos..];

    for &kwd in keywords {
        if starting_text.starts_with(kwd) {
            pos.increment_pos(kwd.len());
            pos.increment_cursor(kwd.len());
            let next_text = match slice_from_position(text, pos) {
                Ok(text) => text,
                Err(_) => {
                    return Some((TokenKind::from_str(kwd).expect(""), pos));
                }
            };
            let mut chars = next_text.chars();
            let c = chars.next();

            match c {
                Some(c) => {
                    if c.is_alphanumeric() {
                        return None;
                    }

                    return Some((TokenKind::from_str(kwd).expect(""), pos));
                }

                None => {
                    return Some((TokenKind::from_str(kwd).expect(""), pos));
                }
            }
        }
    }

    None
}

fn get_next_symbol(
    text: &str,
    mut pos: PositionalOffset,
) -> LexerResult<Option<(TokenKind, PositionalOffset)>> {
    let starting_text = &text[pos.pos..];

    let mut chars = starting_text.chars();

    let token = match chars.next() {
        Some('+') => {
            let token = if matches!(chars.next(), Some('+')) {
                pos.increment_pos(2);
                pos.increment_cursor(2);
                TokenKind::Increment
            } else {
                pos.increment_pos(1);
                pos.increment_cursor(1);
                TokenKind::Plus
            };

            token
        }
        Some('-') => {
            let token = if matches!(chars.next(), Some('-')) {
                pos.increment_pos(2);
                pos.increment_cursor(2);
                TokenKind::Decrement
            } else {
                pos.increment_pos(1);
                pos.increment_cursor(1);
                TokenKind::Minus
            };

            token
        }
        Some('*') => {
            let token = if matches!(chars.next(), Some(')')) {
                pos.increment_pos(2);
                pos.increment_cursor(2);
                TokenKind::CommentEnd
            } else {
                pos.increment_pos(1);
                pos.increment_cursor(1);
                TokenKind::Multi
            };

            token
        }
        Some('/') => {
            let token = if matches!(chars.next(), Some('/')) {
                pos.increment_pos(2);
                pos.increment_cursor(1);
                TokenKind::FlatDiv
            } else {
                pos.increment_pos(1);
                pos.increment_cursor(1);
                TokenKind::Div
            };

            token
        }
        Some('%') => {
            pos.increment_pos(1);
            pos.increment_cursor(1);
            TokenKind::Mod
        }
        Some('<') => {
            let token = if matches!(chars.next(), Some('<')) {
                pos.increment_pos(2);
                pos.increment_cursor(2);
                TokenKind::ShiftLeft
            } else if matches!(chars.next(), Some('-')) {
                pos.increment_pos(2);
                pos.increment_cursor(2);
                TokenKind::LeftArrow
            } else {
                pos.increment_pos(1);
                pos.increment_cursor(1);
                TokenKind::Lesser
            };

            token
        }
        Some('>') => {
            let token = if matches!(chars.next(), Some('>')) {
                pos.increment_pos(2);
                pos.increment_cursor(2);
                TokenKind::ShiftRight
            } else {
                pos.increment_pos(1);
                pos.increment_cursor(1);
                TokenKind::Greater
            };

            token
        }
        Some('=') => {
            pos.increment_pos(1);
            pos.increment_cursor(1);
            TokenKind::Equals
        }
        Some('(') => {
            let token = if matches!(chars.next(), Some('*')) {
                pos.increment_pos(2);
                pos.increment_cursor(2);
                TokenKind::CommentStart
            } else {
                pos.increment_pos(1);
                pos.increment_cursor(1);
                TokenKind::LeftPar
            };

            token
        }
        Some('.') => {
            let token = if matches!(chars.next(), Some('.')) {
                pos.increment_pos(2);
                pos.increment_cursor(2);
                TokenKind::Range
            } else {
                pos.increment_pos(1);
                pos.increment_cursor(1);
                TokenKind::Member
            };

            token
        }
        Some('!') => {
            let token = if matches!(chars.next(), Some('=')) {
                pos.increment_pos(2);
                pos.increment_cursor(2);
                TokenKind::NotEquals
            } else {
                pos.increment_pos(1);
                pos.increment_cursor(1);
                TokenKind::Not
            };

            token
        }
        Some(')') => {
            pos.increment_pos(1);
            pos.increment_cursor(1);
            TokenKind::RightPar
        }
        Some('[') => {
            pos.increment_pos(1);
            pos.increment_cursor(1);
            TokenKind::LeftBracket
        }
        Some(']') => {
            pos.increment_pos(1);
            pos.increment_cursor(1);
            TokenKind::RightBracket
        }
        Some('{') => {
            pos.increment_pos(1);
            pos.increment_cursor(1);
            TokenKind::LeftBrace
        }
        Some('}') => {
            pos.increment_pos(1);
            pos.increment_cursor(1);
            TokenKind::RightBrace
        }
        Some(',') => {
            pos.increment_pos(1);
            pos.increment_cursor(1);
            TokenKind::Comma
        }
        Some(';') => {
            pos.increment_pos(1);
            pos.increment_cursor(1);
            TokenKind::SemiColon
        }
        Some(':') => {
            pos.increment_pos(1);
            pos.increment_cursor(1);
            TokenKind::Colon
        }
        Some('|') => {
            pos.increment_pos(1);
            pos.increment_cursor(1);
            TokenKind::Pipe
        }
        Some('"') => {
            pos.increment_pos(1);
            pos.increment_cursor(1);
            let mut string_flag = false;

            while let Some(ch) = chars.next() {
                match ch {
                    '"' => {
                        pos.increment_pos(1);
                        pos.increment_cursor(1);
                        string_flag = true;
                        break;
                    }
                    '\n' => {
                        pos.increment_pos(1);
                        pos.increment_line(1);
                        pos.reset_cursor();
                    }
                    _ => {
                        if ch == '"' && string_flag {
                            break;
                        } else {
                            pos.increment_pos(1);
                            pos.increment_cursor(1);
                        }
                    }
                }
            }

            if string_flag {
                TokenKind::String
            } else {
                return Err(LexerError::UnfinishedStringError(Location::new(
                    Position::default(),
                )));
            }
        }
        _ => TokenKind::EOF,
    };

    Ok(Some((token, pos)))
}

fn get_next_ident(text: &str, mut pos: PositionalOffset) -> Option<(TokenKind, PositionalOffset)> {
    let starting_text = &text[pos.pos..];

    let mut chars = starting_text.chars();

    let first_char = chars.next();

    match first_char {
        Some(c) => {
            if !c.is_alphabetic() {
                return None;
            }

            pos.increment_pos(1);
            pos.increment_cursor(1);

            while let Some(c) = chars.next() {
                if c.is_alphanumeric() | (c == '_') {
                    pos.increment_pos(1);
                    pos.increment_cursor(1);
                } else {
                    break;
                }
            }

            Some((TokenKind::Ident, pos))
        }

        None => return None,
    }
}

fn get_next_numeric(
    text: &str,
    mut pos: PositionalOffset,
) -> Option<(TokenKind, PositionalOffset)> {
    let starting_text = &text[pos.pos..];
    let starting_pos = pos.pos;
    let mut float_flag = false;

    let mut chars = starting_text.chars();

    let first_char = chars.next();

    match first_char {
        Some(c) => {
            if !c.is_numeric() {
                return None;
            }

            pos.increment_pos(1);
            pos.increment_cursor(1);

            while let Some(c) = chars.next() {
                if c.is_numeric() || (c == '_') && (pos.pos - starting_pos % 3 == 0) {
                    pos.increment_pos(1);
                    pos.increment_cursor(1);
                } else if (c == '_') && ((pos.pos - starting_pos) % 3 == 0) {
                    pos.increment_pos(1);
                    pos.increment_cursor(1);
                } else if float_flag == false && c == '.' {
                    float_flag = true;
                    pos.increment_pos(1);
                    pos.increment_cursor(1);
                } else if c == '.' && float_flag == true {
                    break;
                } else {
                    break;
                }
            }

            if !float_flag {
                return Some((TokenKind::Int, pos));
            }

            Some((TokenKind::Float, pos))
        }

        None => return None,
    }
}

fn get_next_token_kind(
    text: &str,
    pos: PositionalOffset,
) -> LexerResult<(TokenKind, PositionalOffset)> {
    // it's a keywords
    if let Some((token, pos)) = get_next_keyword(text, &KEYWORDS, pos) {
        return Ok((token, pos));
    }

    // it's a ident
    if let Some((token, pos)) = get_next_ident(text, pos) {
        return Ok((token, pos));
    }

    // it's a numeric
    if let Some((token, pos)) = get_next_numeric(text, pos) {
        return Ok((token, pos));
    }

    // it's a symbol
    if let Some((token, pos)) = get_next_symbol(text, pos)? {
        return Ok((token, pos));
    }

    return Ok((TokenKind::EOF, pos));
}

// ==================================================================================== //
// Lexer Test Suite                                                                     //
// ==================================================================================== //

#[cfg(test)]
mod test {
    use super::*;

    fn check_lex(text: &str, expected: Vec<Token>) {
        let mut lexer = Lexer::new(text);
        let actual = lexer.lex_with_separate_pass().expect("");

        assert_eq!(expected, actual);
    }

    // #[test]
    fn test_lex_def_leftpar_comment_start() {
        let text = "   def ((*!!=";

        let expected: Vec<Token> = vec![
            Token::new(TokenKind::ProblemDef, "def", 1, 3),
            Token::new(TokenKind::LeftPar, "(", 1, 7),
            Token::new(TokenKind::CommentStart, "(*", 1, 8),
            Token::new(TokenKind::Not, "!", 1, 10),
            Token::new(TokenKind::NotEquals, "!=", 1, 11),
            Token::new(TokenKind::EOF, "", 1, 13),
        ];

        let _ = check_lex(text, expected);
    }

    // #[test]
    fn test_lex_multiple_pluses() {
        let text = "++++ ++ ++ +";

        let expected: Vec<Token> = vec![
            Token::new(TokenKind::Increment, "++", 1, 0),
            Token::new(TokenKind::Increment, "++", 1, 2),
            Token::new(TokenKind::Increment, "++", 1, 5),
            Token::new(TokenKind::Increment, "++", 1, 8),
            Token::new(TokenKind::Plus, "+", 1, 11),
            Token::new(TokenKind::EOF, "", 1, 12),
        ];

        let _ = check_lex(text, expected);
    }

    // #[test]
    // #[should_panic]
    fn test_lex_unfinished_string() {
        let text = "\"hello";

        let expected: Vec<Token> = vec![
            Token::new(TokenKind::String, "\"poustiiiii hliaaaa\"", 1, 0),
            Token::new(TokenKind::EOF, "", 1, 20),
        ];

        let _ = check_lex(text, expected);
    }

    // #[test]
    fn test_lex_string() {
        let text = "\"poustiiiii hliaaaa\"";

        let expected: Vec<Token> = vec![
            Token::new(TokenKind::String, "\"poustiiiii hliaaaa\"", 1, 0),
            Token::new(TokenKind::EOF, "", 1, 20),
        ];

        let _ = check_lex(text, expected);
    }

    #[test]
    fn test_float_member_int() {
        let text = "123.123.123";

        let expected: Vec<Token> = vec![
            Token::new(TokenKind::Float, "123.123", 1, 0),
            Token::new(TokenKind::Member, ".", 1, 7),
            Token::new(TokenKind::Int, "123", 1, 8),
            Token::new(TokenKind::EOF, "", 1, 11),
        ];

        let _ = check_lex(text, expected);
    }

    #[test]
    fn test_int_floats() {
        let text = "123.123 123";

        let expected: Vec<Token> = vec![
            Token::new(TokenKind::Float, "123.123", 1, 0),
            Token::new(TokenKind::Int, "123", 1, 8),
            Token::new(TokenKind::EOF, "", 1, 11),
        ];

        let _ = check_lex(text, expected);
    }

    #[test]
    fn test_floats() {
        let text = "123.123";

        let expected: Vec<Token> = vec![
            Token::new(TokenKind::Float, "123.123", 1, 0),
            Token::new(TokenKind::EOF, "", 1, 7),
        ];

        let _ = check_lex(text, expected);
    }

    #[test]
    fn test_lex_fn() {
        let text = "def \n hello_ () { 103 }";

        let expected: Vec<Token> = vec![
            Token::new(TokenKind::ProblemDef, "def", 1, 0),
            Token::new(TokenKind::Ident, "hello_", 2, 6),
            Token::new(TokenKind::LeftPar, "(", 2, 13),
            Token::new(TokenKind::RightPar, ")", 2, 14),
            Token::new(TokenKind::LeftBrace, "{", 2, 16),
            Token::new(TokenKind::Int, "103", 2, 18),
            Token::new(TokenKind::RightBrace, "}", 2, 22),
            Token::new(TokenKind::EOF, "", 2, 23),
        ];

        let _ = check_lex(text, expected);
    }

    #[test]
    fn test_start_with_for_keyword_with_symbol_after() {
        let text = "color,";

        assert!(text.starts_with("color"))
    }
}
