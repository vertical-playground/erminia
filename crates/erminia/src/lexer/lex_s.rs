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

static OPERATORS: [&str; 26] = [
    "+", "-", "*", "/", "//", "%", "<<", ">>",
    "<", ">", ".", "!", "!=", "=", "(", ")", "[",
    "]", "{", "}", ",", ";", ":", "..", "(*", "*)",
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
    
    fn advance(&mut self) -> LexerResult<(TokenKind, PositionalOffset)> {
        let mut text = self.content;
        let mut pos = PositionalOffset::new_from_po(self.get_po());

        pos = trim_starting_whitespace(text, pos);

        text = &text[pos.pos..];

        let (token, pos) = get_next_token_kind(text, KEYWORDS.to_vec(), pos)?;

        Ok((token, pos))
    }

    fn set_po(&mut self, pos: PositionalOffset) {
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

fn get_next_token_kind(
    mut text: &str,
    keywords: Vec<&str>,
    mut pos: PositionalOffset
) -> LexerResult<(TokenKind, PositionalOffset)> {

    for &kwd in &keywords {
        if text.starts_with(kwd) {
            pos.pos = kwd.len();
            text = &text[pos.pos..];
            let mut chars = text.chars();

            if matches!(chars.next(), Some(' ')) {
                pos.pos += 1;
                return Ok((TokenKind::from_str(kwd).expect(""), pos));
            }
        }
    }

    let mut chars = text.chars();

    let token = match chars.next() {
        Some('+') => {
            pos.increment_pos(1);
            TokenKind::Plus
        },
        Some('-') => {
            pos.increment_pos(1);
            TokenKind::Minus
        },
        Some('*') => {
            let token = if matches!(chars.next(), Some(')')) {
                pos.increment_pos(2);
                TokenKind::CommentEnd
            } else {
                pos.increment_pos(1);
                TokenKind::Multi
            };

            token
        },
        Some('/') => {
            let token = if matches!(chars.next(), Some('/')) {
                pos.increment_pos(2);
                TokenKind::FlatDiv
            } else {
                pos.increment_pos(1);
                TokenKind::Div
            };

            token
        },
        Some('%') => {
            pos.increment_pos(1);
            TokenKind::Mod
        },
        Some('<') => {
            let token = if matches!(chars.next(), Some('<')) {
                pos.increment_pos(2);
                TokenKind::ShiftLeft
            } else {
                pos.increment_pos(1);
                TokenKind::Lesser
            };

            token
        },
        Some('>') => {
            let token = if matches!(chars.next(), Some('>')) {
                pos.increment_pos(2);
                TokenKind::ShiftRight
            } else {
                pos.increment_pos(1);
                TokenKind::Greater
            };

            token
        },
        Some('=') => {
            pos.increment_pos(1);
            TokenKind::Equals
        }
        Some('(') => {
            let token = if matches!(chars.next(), Some('*')) {
                pos.increment_pos(2);
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
                TokenKind::Member
            };

            token
        },
        Some('!') => {
            let token = if matches!(chars.next(), Some('=')) {
                pos.increment_pos(2);
                TokenKind::NotEquals
            } else {
                pos.increment_pos(1);
                TokenKind::Not
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
        Some('"') =>  { 
            let mut flag = false;

            while let Some(ch) = chars.next() {
                match ch {
                    '"' => {
                        pos.increment_pos(1);
                        flag = true;
                        break
                    }
                    '\n' => {
                        pos.increment_pos(1);
                        pos.increment_line(1);
                        pos.reset_cursor();
                    }
                    _ => {
                        pos.increment_pos(1);
                    }
                }
            };

            if flag {
                TokenKind::String
            } else {
                TokenKind::EOF
            }

        },
        _ => {
            return Err(LexerError::TokenError);
        } 
    };

    Ok((token, pos))
}

#[cfg(test)]
mod test {
    use super::*;

}
