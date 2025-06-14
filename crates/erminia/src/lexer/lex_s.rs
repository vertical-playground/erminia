use std::collections::HashSet;
use std::str::FromStr;
use std::str::Chars;

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

#[warn(dead_code)]
pub struct Lexer<'input> {
    content: &'input str,
    current: Option<char>,
    cursor: usize,
    line: usize,
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
            keywords: keywords,
            operators: operators, 
        }
    }
}

impl<'a> Lexer<'a> {
    fn new(content: &'a str) -> Lexer<'a> {
        let keywords: HashSet<&str> = KEYWORDS.into_iter().collect::<HashSet<&str>>();

        let operators: HashSet<&str> = OPERATORS.into_iter().collect::<HashSet<&str>>();

        Lexer {
            content: content,
            current: None,
            cursor: 1,
            line: 1,
            keywords: keywords,
            operators: operators,
        }
    }

    fn check_for_kwds(&mut self) -> Option<(TokenKind, usize)> {
        let mut text = self.trim_starting_whitespace().to_owned();
        let mut pos: usize;

        for &kwd in &self.keywords {
            if text.starts_with(kwd) {
                pos = kwd.len();
                text = text[pos..].to_owned();
                let mut chars = text.chars();

                if matches!(chars.next(), Some(' ')) {
                    return Some((TokenKind::from_str(kwd).expect(""), pos));
                }
            }
        }

        None
    }

    fn check_for_symbols(&mut self) -> Option<(TokenKind, usize)> {
        let text = self.trim_starting_whitespace();
        let mut chars = text.chars();
        let mut pos = 0;

        let token = match chars.next() {
            Some('=') => {
                pos += 1;
                TokenKind::Equals
            }
            Some('(') => {
                let token = if matches!(chars.next(), Some('*')) {
                    pos += 2;
                    TokenKind::CommentStart
                } else {
                    pos += 1;
                    TokenKind::LeftPar
                };

                token
            },
            Some('.') => {
                let token = if matches!(chars.next(), Some('.')) {
                    pos += 2;
                    TokenKind::Range
                } else {
                    pos += 1;
                    TokenKind::Error
                };

                token
            },
            Some(')') => {
                pos += 1;
                TokenKind::RightPar
            }
            Some('[') => {
                pos += 1;
                TokenKind::LeftBrace
            }
            Some(']') => {
                pos += 1;
                TokenKind::RightBrace
            },
            Some('{') => {
                pos += 1;
                TokenKind::LeftBracket
            },
            Some('}') => {
                pos += 1;
                TokenKind::RightBracket
            },
            Some(',') => {
                pos += 1;
                TokenKind::Comma
            },
            Some(';') => {
                pos += 1;
                TokenKind::SemiColon
            },
            Some(':') => {
                pos += 1;
                TokenKind::Colon

            },
            Some('*') => {
                let token = if matches!(chars.next(), Some(')')) {
                    pos += 1;
                    TokenKind::CommentEnd
                } else {
                    pos += 1;
                    TokenKind::Error
                };

                token
            }
            _ => {
                pos += 1;
                TokenKind::Error
            } 
        };

        if token.eq(&TokenKind::Error) {
            return None;
        }

        Some((token, pos))
    }

    fn trim_starting_whitespace(&mut self) -> &str {
        let text = &self.content;
        let mut chars = text.chars();
        let mut pos = 0;

        while let Some(c) = chars.next() {
            match c {
                ' ' | '\t' => {
                    pos += 1;
                    self.cursor += 1;
                } ,
                '\n' => {
                    pos += 1;
                    self.line += 1;
                    self.cursor = 0;
                }
                '\r' if matches!(chars.next(), Some('\n')) => {
                    pos += 2;
                    self.line += 1;
                    self.cursor = 0;
                }
                _ => break
            }
        }

        &text[pos..]
    }

    fn get_line(&self) -> usize {
        self.line
    }

    fn get_cursor(&self) -> usize {
        self.cursor
    }

    fn set_content(&mut self, content: &'a str) { 
        self.content = content;
    }

}


#[cfg(test)]
mod test {
    use super::*;

    fn check_lex(actual: &str, expected: &str) {
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_trim_start_ws() {
        let mut lex = Lexer::new("       \n Hello bossman!");
        let actual = lex.trim_starting_whitespace();
        let expected = "Hello bossman!";
        check_lex(actual, expected);
    }

    #[test]
    fn test_no_trim_line() {
        let lex = Lexer::new("       \n Hello bossman!");
        assert_eq!(1, lex.get_line());
    }

    #[test]
    fn test_trim_start_line() {
        let mut lex = Lexer::new("       \n Hello bossman!");
        let _ = lex.trim_starting_whitespace();
        println!("{}", lex.get_line());
        assert_eq!(2, lex.get_line());
    }

    #[test]
    fn test_trim_start_cursor() {
        let mut lex = Lexer::new("       ");
        let _ = lex.trim_starting_whitespace();
        println!("{}", lex.get_cursor());
        assert_eq!(8, lex.get_cursor());
    }

    #[test]
    fn test_trim_start_cursor_return() {
        let mut lex = Lexer::new("       \n Hello bossman!");
        let _ = lex.trim_starting_whitespace();
        println!("{}", lex.get_cursor());
        assert_eq!(1, lex.get_cursor());
    }

    #[test]
    fn test_check_for_kwd_let() -> LexerResult<()> {
        let mut lex = Lexer::new("   let def");
        let token = lex.check_for_kwds().unwrap();
        assert_eq!(token.0, TokenKind::LetKwd);
        Ok(())
    }

    #[test]
    fn test_check_for_kwd_let_def() -> LexerResult<()> {
        let text = "    let def";
        let mut lex = Lexer::new(text);
        let (token,pos) = lex.check_for_kwds().unwrap();
        assert_eq!(token, TokenKind::LetKwd);
        lex.set_content(&text[pos..]);
        let (token,_) = lex.check_for_kwds().unwrap();
        assert_eq!(token, TokenKind::ProblemDef);
        Ok(())

    }
}
