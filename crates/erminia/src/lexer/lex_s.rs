use std::collections::HashSet;
use std::fmt;
use std::str::FromStr;

use crate::error::lexer_error::{LexerError, LexerResult};
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
    "+", "-", "*", "/", "//", "%", "<<", ">>", "<", ">", ".", "!", "!=", "=", "(", ")", "[", "]",
    "{", "}", ",", ";", ":", "..", "(*", "*)",
];

// ====================================================================================//
//                            Positional Offset Struct                                 //
// ====================================================================================//

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
    fn new(pos: usize, cursor: usize, line: usize) -> PositionalOffset {
        PositionalOffset {
            pos: pos,
            cursor: cursor,
            line: line,
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
        self.cursor = 1;
    }

    fn reset_line(&mut self) {
        self.line = 1;
    }

    fn get_cursor(&self) -> usize {
        self.pos
    }

    fn get_position(&self) -> usize {
        self.pos
    }

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

// ====================================================================================//
//                                  Lexer Struct                                       //
// ====================================================================================//

pub struct Lexer<'input> {
    content: &'input str,
    current: Option<char>,
    position: PositionalOffset,
}

impl Default for Lexer<'_> {
    fn default() -> Self {
        Lexer {
            content: "",
            current: None,
            position: PositionalOffset::default(),
        }
    }
}

impl<'input> Lexer<'input> {
    pub fn new(content: &'input str) -> Lexer<'input> {
        Lexer {
            content: content,
            current: None,
            position: PositionalOffset::default(),
        }
    }

    fn lex(&mut self) -> LexerResult<Vec<Token>> {
        let mut tokens: Vec<Token> = Vec::new();

        loop {
            let (token, pos) = &advance(self.content, self.position)?;

            self.set_po(*pos);

            tokens.push(*token);

            if token.get_kind() == TokenKind::EOF {
                break;
            }
        }

        Ok(tokens)
    }

    fn set_po(&mut self, pos: PositionalOffset) {
        self.position = pos
    }

    fn get_content(&self) -> &str {
        self.content
    }

    fn get_po(&self) -> PositionalOffset {
        self.position
    }
}

fn advance(text: &str, po: PositionalOffset) -> LexerResult<(Token, PositionalOffset)> {
    let pos = trim_starting_whitespace(text, po);

    println!("[ADVANCE] started here: {}", pos);

    let (kind, next_pos) = get_next_token_kind(text, KEYWORDS.to_vec(), pos).expect("oopsie");

    println!("[ADVANCE] ended here: {}", next_pos);

    let lexeme = &text[pos.pos..next_pos.pos];

    println!("[ADVANCE] lexeme: |{}|", lexeme);
    // This is incorrect, have to fix positioning
    // let token = Token::new(kind, lexeme, pos.pos, next_pos.pos - 1);
    let token = Token::new(kind, lexeme, pos.get_line(), pos.get_cursor());

    println!("[NEW TOKEN] result: {} {}", token, next_pos);

    Ok((token, next_pos))
}

fn trim_starting_whitespace(text: &str, mut pos: PositionalOffset) -> PositionalOffset {

    let starting_text = &text[pos.pos..];

    println!("[WHITESPACE TRIM] before trim: {}", text);

    let mut chars = starting_text.chars();

    while let Some(c) = chars.next() {
        match c {
            ' ' | '\t' => {
                println!("Should be here");
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

    println!("[WHITESPACE TRIM] after trim: {}", &text[pos.pos..]);

    pos
}

fn slice_from_position(text: &str, pos: PositionalOffset) -> LexerResult<&str> {
    if pos.pos > text.len() {
        Err(LexerError::TokenError)
    } else {
        Ok(&text[pos.pos..])
    }
}

fn get_next_token_kind(
    text: &str,
    keywords: Vec<&str>,
    mut pos: PositionalOffset,
) -> LexerResult<(TokenKind, PositionalOffset)> {
    let starting_text = &text[pos.pos..];

    // It's a keywords
    for &kwd in &keywords {
        if starting_text.starts_with(kwd) {
            println!("[NEXT TOKEN] matched a keyword '{}'", kwd);
            pos.increment_pos(kwd.len());
            println!("[NEXT TOKEN] position after kwd match: {}", pos.get_position());
            println!("[NEXT TOKEN] kwd len: '{}'", kwd.len());
            let next_text = match slice_from_position(text, pos) {
                Ok(text) => text,
                Err(_) => {
                    return Ok((TokenKind::from_str(kwd).expect(""), pos));
                }
            };
            println!("[NEXT TOKEN] second print: {}", next_text);
            let mut chars = next_text.chars();
            let c = chars.next();
            println!("[NEXT TOKEN] The next character is:\"{:?}\"", c);
            if matches!(c, Some(' ')) {
                return Ok((TokenKind::from_str(kwd).expect(""), pos));
            } else if matches!(c, None) {
                return Ok((TokenKind::from_str(kwd).expect(""), pos));
            }
        }
    }

    // TODO: It's an ident
    // TODO: It's numerical


    // It's a symbol
    let mut chars = starting_text.chars();

    println!("[NEXT TOKEN] third print: {:?}", chars);

    let token = match chars.next() {
        Some('+') => {
            pos.increment_pos(1);
            TokenKind::Plus
        }
        Some('-') => {
            pos.increment_pos(1);
            TokenKind::Minus
        }
        Some('*') => {
            let token = if matches!(chars.next(), Some(')')) {
                pos.increment_pos(2);
                TokenKind::CommentEnd
            } else {
                pos.increment_pos(1);
                TokenKind::Multi
            };

            token
        }
        Some('/') => {
            let token = if matches!(chars.next(), Some('/')) {
                pos.increment_pos(2);
                TokenKind::FlatDiv
            } else {
                pos.increment_pos(1);
                TokenKind::Div
            };

            token
        }
        Some('%') => {
            pos.increment_pos(1);
            TokenKind::Mod
        }
        Some('<') => {
            let token = if matches!(chars.next(), Some('<')) {
                pos.increment_pos(2);
                TokenKind::ShiftLeft
            } else {
                pos.increment_pos(1);
                TokenKind::Lesser
            };

            token
        }
        Some('>') => {
            let token = if matches!(chars.next(), Some('>')) {
                pos.increment_pos(2);
                TokenKind::ShiftRight
            } else {
                pos.increment_pos(1);
                TokenKind::Greater
            };

            token
        }
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
        }
        Some('.') => {
            let token = if matches!(chars.next(), Some('.')) {
                pos.increment_pos(2);
                TokenKind::Range
            } else {
                pos.increment_pos(1);
                TokenKind::Member
            };

            token
        }
        Some('!') => {
            let token = if matches!(chars.next(), Some('=')) {
                pos.increment_pos(2);
                TokenKind::NotEquals
            } else {
                pos.increment_pos(1);
                TokenKind::Not
            };

            token
        }
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
        }
        Some('{') => {
            pos.increment_pos(1);
            TokenKind::LeftBracket
        }
        Some('}') => {
            pos.increment_pos(1);
            TokenKind::RightBracket
        }
        Some(',') => {
            pos.increment_pos(1);
            TokenKind::Comma
        }
        Some(';') => {
            pos.increment_pos(1);
            TokenKind::SemiColon
        }
        Some(':') => {
            pos.increment_pos(1);
            TokenKind::Colon
        }
        Some('\"') => {
            let mut flag = false;
            println!("it's a string");

            while let Some(ch) = chars.next() {
                match ch {
                    '\"' => {
                        pos.increment_pos(1);
                        flag = true;
                        break;
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
            }

            if flag {
                TokenKind::String
            } else {
                TokenKind::EOF
            }
        }
        _ => TokenKind::EOF,
    };

    Ok((token, pos))
}

#[cfg(test)]
mod test {
    use super::*;

    fn check_lex(text: &str, expected: Vec<Token>) {
        let mut lexer = Lexer::new(text);
        let actual = lexer.lex().expect("");

        assert_eq!(expected, actual);
    }

    fn check_advance(text: &str, expected: Token) {
        let lexer = Lexer::new(text);

        let (token, _) = advance(text, lexer.get_po()).expect("Something went wrong with advance");

        assert_eq!(token, expected);
    }

    // #[test]
    fn test_advance_def_leftpar_comment_start() {
        let text = "def ((*";
        let lexer = Lexer::new(text);
        let (token, next_pos) =
            advance(text, lexer.get_po()).expect("Something went wrong with advance");

        println!("{}", token);

        assert_eq!(token, Token::new(TokenKind::ProblemDef, "def", 0, 0));

        let (token, next_pos) = advance(text, next_pos).expect("Something went wrong with advance");

        assert_eq!(token, Token::new(TokenKind::LeftPar, "(", 0, 0));

        let (token, _) = advance(text, next_pos).expect("Something went wrong with advance");

        assert_eq!(token, Token::new(TokenKind::CommentStart, "(*", 0, 0));
    }

    // #[test]
    fn test_advance_leftpar() {
        let text = "   (";
        check_advance(text, Token::new(TokenKind::LeftPar, "(", 0, 0));
    }

    // #[test]
    fn test_advance_def() {
        let text = "   def";

        check_advance(text, Token::new(TokenKind::ProblemDef, "def", 0, 0))
    }

    // #[test]
    fn test_advance_comment_start() {
        let text = "   (*";

        check_advance(text, Token::new(TokenKind::CommentStart, "(*", 0, 0))
    }

    // #[test]
    fn test_lex_def_def() {
        let text = "def def";

        let expected: Vec<Token> = vec![
            Token::new(TokenKind::ProblemDef, "def", 0, 0),
            Token::new(TokenKind::ProblemDef, "def", 0, 0),
            Token::new(TokenKind::EOF, "", 0, 0),
        ];

        let _ = check_lex(text, expected);
    }

    // #[test]
    fn test_stuffs_2() {
        let text = " def    (*)";

        let expected: Vec<Token> = vec![
            Token::new(TokenKind::ProblemDef, "def", 0, 0),
            Token::new(TokenKind::CommentStart, "(*", 0, 0),
            Token::new(TokenKind::RightPar, ")", 0, 0),
            Token::new(TokenKind::EOF, "", 0, 0),
        ];

        let _ = check_lex(text, expected);
    }

    #[test]
    fn test_lex_def_leftpar_comment_start() {
        let text = "   def ((*!!=";

        println!("[TEST] text: {}", text);

        let expected: Vec<Token> = vec![
            Token::new(TokenKind::ProblemDef, "def", 0, 1),
            Token::new(TokenKind::LeftPar, "(", 0, 1),
            Token::new(TokenKind::CommentStart, "(*", 0, 1),
            Token::new(TokenKind::Not, "!", 0, 1),
            Token::new(TokenKind::NotEquals, "!=", 0, 1),
            Token::new(TokenKind::EOF, "", 0, 1),
        ];

        let _ = check_lex(text, expected);
    }






}
