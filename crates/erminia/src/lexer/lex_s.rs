use std::collections::HashSet;
use std::str::Chars;

use crate::error::lexer_error::LexerError;

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
pub struct Lexer<'a> {
    content: Chars<'a>,
    current: Option<char>,
    cursor: usize,
    line: usize,
    keywords: HashSet<&'a str>,
    operators: HashSet<&'a str>,
}

impl Default for Lexer<'_> {
    fn default() -> Self {
        let keywords: HashSet<&str> = KEYWORDS.into_iter().collect::<HashSet<&str>>();

        let operators: HashSet<&str> = OPERATORS.into_iter().collect::<HashSet<&str>>();

        let chars = "".chars();

        Lexer {
            content: chars,
            current: None,
            cursor: 0,
            line: 0,
            keywords: keywords,
            operators: operators,
        }
    }
}

impl<'a> Lexer<'a> {
    fn new(content: &'a str) -> Lexer<'a> {
        let keywords: HashSet<&str> = KEYWORDS.into_iter().collect::<HashSet<&str>>();

        let operators: HashSet<&str> = OPERATORS.into_iter().collect::<HashSet<&str>>();

        let chars = content.chars();

        Lexer {
            content: chars,
            current: None,
            cursor: 0,
            line: 0,
            keywords: keywords,
            operators: operators,
        }
    }

    fn advance(&mut self) -> Option<char> {
        self.current = self.content.next();
        self.current
    }

    fn consume_whitespace(&mut self) {
        while let Some(c) = self.current {
            if !c.is_whitespace() {
                break;
            }

            self.advance();
        }
    }

    fn set_content(&mut self, content: &'a str) {
        self.content = content.chars();
    }

    fn get_cursor(&self) -> usize {
        self.cursor
    }
}
