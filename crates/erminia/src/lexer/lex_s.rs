use std::collections::HashSet;
use std::str::FromStr;

use crate::error::lexer_error::{LexerResult,LexerError};
use crate::lexer::token::*;

static KEYWORDS: [&str; 9] = [
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

static OPERATORS: [&str; 13] = [
    "=", "(", ")", "[", "]", "{", "}", ",", ";", ":", "..", "(*", "*)",
];

#[derive(Clone, Copy)]
pub struct PositionalOffset {
    pos: usize, 
    cursor: usize,
    line: usize
}

impl Default for PositionalOffset {
    fn default() -> Self {
        PositionalOffset {
            pos: 0,
            cursor: 0,
            line: 0
        }
    }
}

impl PositionalOffset {
    fn new(pos: usize, cursor: usize, line: usize) -> PositionalOffset {
        PositionalOffset {
            pos: pos,
            cursor: cursor,
            line: line
        }
    }

    fn new_from_po(pos: PositionalOffset) -> PositionalOffset {
        PositionalOffset {
            pos: pos.pos,
            cursor: pos.cursor,
            line: pos.line,
        }
    }

    fn increment_pos(&mut self, val: usize) {
        self.pos += val;
    }

    fn increment_cursor(&mut self, val: usize) {
        self.cursor += val;
    }

    fn increment_line(&mut self, val: usize) {
        self.line += val;
    }

    fn reset_pos(&mut self) {
        self.pos = 0;
    }

    fn reset_cursor(&mut self) {
        self.cursor = 0;
    }

    fn reset_line(&mut self) {
        self.line = 0;
    }
}

#[warn(dead_code)]
pub struct Lexer<'input> {
    content: &'input str,
    current: Option<char>,
    cursor: usize,
    line: usize,
    position: usize,
    keywords: HashSet<&'input str>,
    operators: HashSet<&'input str>,
}

impl Default for Lexer<'_> {
    fn default() -> Self {
        let keywords: HashSet<&str> = KEYWORDS.into_iter().collect::<HashSet<&str>>();

        let operators: HashSet<&str> = OPERATORS.into_iter().collect::<HashSet<&str>>();

        Lexer {
            content: "",
            current: None,
            cursor: 1,
            line: 1,
            position: 0,
            keywords: keywords,
            operators: operators, 
        }
    }
}

impl<'input> Lexer<'input> {

    pub fn new(content: &'input str) -> Lexer<'input> {
        let keywords: HashSet<&str> = KEYWORDS.into_iter().collect::<HashSet<&str>>();

        let operators: HashSet<&str> = OPERATORS.into_iter().collect::<HashSet<&str>>();

        Lexer {
            content: content,
            current: None,
            cursor: 1,
            line: 1,
            position: 0,
            keywords: keywords,
            operators: operators,
        }
    }
    
    fn advance(&mut self) -> (TokenKind, PositionalOffset) {
        let mut text = self.content;
        let mut pos = PositionalOffset::new_from_po(self.get_po());

        pos = trim_starting_whitespace(text, pos);

        text = &text[pos.pos..];

        let (token, pos) = check_for_symbols(text, pos)
            .unwrap_or((TokenKind::Error, pos));

        if !token.eq(&TokenKind::Error) {
            self.set_positional_offset(pos);
            return (token, pos);
        }

        let (token, pos) = check_for_kwds(text, KEYWORDS.to_vec(), pos)
            .unwrap_or((TokenKind::Error, pos));

        if !token.eq(&TokenKind::Error) {
            self.set_positional_offset(pos);
            return (token, pos);
        }

        let (token, pos) = check_for_string(text, pos)
            .unwrap_or((TokenKind::Error, pos));

        if !token.eq(&TokenKind::Error) {
            self.set_positional_offset(pos);
            return (token, pos);
        }

        (TokenKind::EOF, pos)
    }

    fn set_positional_offset(&mut self, pos: PositionalOffset) {
        self.cursor = pos.cursor;
        self.position = pos.pos;
        self.line = pos.line;
    }

    fn get_po(&self) -> PositionalOffset {
        PositionalOffset {
            pos: self.position,
            cursor: self.cursor,
            line: self.line
        }
    }
}

fn trim_starting_whitespace(
    text: &str,
    mut pos: PositionalOffset,
) -> PositionalOffset {
    let mut chars = text.chars();

    while let Some(c) = chars.next() {
        match c {
            ' ' | '\t' => {
                pos.pos += 1;
                pos.cursor += 1;
            } ,
            '\n' => {
                pos.pos += 1;
                pos.line += 1;
                pos.cursor = 0;
            }
            '\r' if matches!(chars.next(), Some('\n')) => {
                pos.pos += 2;
                pos.line += 1;
                pos.cursor = 0;
            }
            _ => break
        }
    }

    pos
}

fn check_for_string(
    text: &str,
    mut pos: PositionalOffset
) -> Option<(TokenKind, PositionalOffset)> {
    let mut chars = text.chars();

    let token: TokenKind = match chars.next() {
        Some('"') =>  { 
            while let Some(ch) = chars.next() {
                match ch {
                    '"' => {
                        pos.increment_pos(1);
                        break
                    }
                    '\n' => {
                        pos.increment_pos(1);
                        pos.increment_cursor(1);
                        pos.reset_cursor();
                    }
                    _ => {
                        pos.increment_pos(1);
                    }

                }
            };

            TokenKind::String
        },
        _ => TokenKind::Error

    };

    if token.eq(&TokenKind::Error) {
        return None
    };

    Some((TokenKind::String, pos))
}

fn check_for_kwds(
    mut text: &str,
    keywords: Vec<&str>,
    mut pos: PositionalOffset
) -> Option<(TokenKind, PositionalOffset)> {

    for &kwd in &keywords {
        if text.starts_with(kwd) {
            pos.pos = kwd.len();
            text = &text[pos.pos..];
            let mut chars = text.chars();

            if matches!(chars.next(), Some(' ')) {
                pos.pos += 1;
                return Some((TokenKind::from_str(kwd).expect(""), pos));
            }
        }
    }

    None
}

fn check_for_symbols(
    text: &str,
    mut pos: PositionalOffset
) -> Option<(TokenKind, PositionalOffset)> {
    let mut chars = text.chars();

    let token = match chars.next() {
        Some('=') => {
            pos.increment_pos(1);
            TokenKind::Equals
        }
        Some('(') => {
            let token = if matches!(chars.next(), Some('*')) {
                pos.increment_pos(1);
                TokenKind::CommentStart
            } else {
                pos.increment_pos(1);
                TokenKind::LeftPar
            };

            token
        },
        Some('.') => {
            let token = if matches!(chars.next(), Some('.')) {
                pos.increment_pos(2);
                TokenKind::Range
            } else {
                pos.increment_pos(1);
                TokenKind::Error
            };

            token
        },
        Some(')') => {
            pos.increment_pos(1);
            TokenKind::RightPar
        }
        Some('[') => {
            pos.increment_pos(1);
            TokenKind::LeftBrace
        }
        Some(']') => {
            pos.increment_pos(1);
            TokenKind::RightBrace
        },
        Some('{') => {
            pos.increment_pos(1);
            TokenKind::LeftBracket
        },
        Some('}') => {
            pos.increment_pos(1);
            TokenKind::RightBracket
        },
        Some(',') => {
            pos.increment_pos(1);
            TokenKind::Comma
        },
        Some(';') => {
            pos.increment_pos(1);
            TokenKind::SemiColon
        },
        Some(':') => {
            pos.increment_pos(1);
            TokenKind::Colon

        },
        Some('*') => {
            let token = if matches!(chars.next(), Some(')')) {
                pos.increment_pos(1);
                TokenKind::CommentEnd
            } else {
                pos.increment_pos(1);
                TokenKind::Error
            };

            token
        }
        _ => {
            pos.increment_pos(1);
            TokenKind::Error
        } 
    };

    if token.eq(&TokenKind::Error) {
        return None;
    }

    Some((token, pos))
}

#[cfg(test)]
mod test {
    use super::*;

}
