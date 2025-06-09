use crate::error::lexer_error::LexerError;

// ====================================================================================//
//                            Identifier Extension Trait                               //
// ====================================================================================//

trait Identifier {
    fn is_valid_indentifier(&self) -> bool;
}

impl Identifier for &str {
    fn is_valid_indentifier(&self) -> bool { true }
}

// ====================================================================================//
//                            TokenKind Enum                                           //
// ====================================================================================//

#[derive(PartialEq)]
pub enum TokenKind {
    Error,
    ProblemDef,
    LetKwd,
    Object,
    SuperObject,
    ObjectShape,
    ObjectColor,
    ProblemExample,
    ProblemInput,
    ProblemOutput,
    Equals,
    LeftPar,
    RightPar,
    LeftBracket,
    RightBracket,
    LeftBrace,
    RightBrace,
    Comma,
    Colon,
    SemiColon,
    Range,
    CommentStart,
    CommentEnd,
    NewLine,
    Tab,
    Int,
    Float,
    Ident,
}

impl std::str::FromStr for TokenKind {
    type Err = LexerError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let token = match s {
            "def" => TokenKind::ProblemDef,
            "let" => TokenKind::LetKwd,
            "object" => TokenKind::Object,
            "superobject" => TokenKind::SuperObject,
            "shape" => TokenKind::ObjectShape,
            "color" => TokenKind::ObjectColor,
            "example" => TokenKind::ProblemExample,
            "input" => TokenKind::ProblemInput,
            "output" => TokenKind::ProblemOutput,
            "==" => TokenKind::Equals,
            "(" => TokenKind::LeftPar,
            ")" => TokenKind::RightPar,
            "[" => TokenKind::LeftBracket,
            "]" => TokenKind::RightBracket,
            "{" => TokenKind::LeftBrace,
            "}" => TokenKind::RightBrace,
            "," => TokenKind::Comma,
            ";" => TokenKind::Colon,
            ":" => TokenKind::SemiColon,
            ".." => TokenKind::Range,
            "(*" => TokenKind::CommentStart,
            "*)" => TokenKind::CommentEnd,
            "\n" => TokenKind::NewLine,
            "\t" => TokenKind::Tab,
            _ => { 
                if s.parse::<i64>().is_ok() { TokenKind::Int }
                else if s.parse::<f64>().is_ok() { TokenKind::Float }
                else if s.is_valid_indentifier() { TokenKind::Ident }
                else { return Err(LexerError::TokenError) }
            }
        };

        Ok(token)
    }
}

// ====================================================================================//
//                                  Position Struct                                    //
// ====================================================================================//

struct Position {
    x: usize, 
    y: usize
}

impl Default for Position {
    fn default() -> Self {
        Position { x: 0, y: 0 }
    }
}

impl Position {
    fn new(x: usize, y: usize) -> Position {
        Position { x: x, y: y }
    }
}

// ====================================================================================//
//                                  Token Struct                                       //
// ====================================================================================//

pub struct Token<'a> {
    kind: TokenKind,
    text: &'a str,
    size: usize,
    pos: Position
}

impl Default for Token<'_> {
    fn default() -> Self {
        Token {
            kind: TokenKind::Tab,
            text: "",
            size: 0,
            pos: Position { x: 0, y: 0 }
        }
    }
}

impl Token<'_> {
    fn new<'a>(
        kind: TokenKind,
        text: &'a str,
        size: usize,
        pos: Position
    ) -> Token<'a> {
        Token {
            kind: kind,
            text: text,
            size: size,
            pos: pos
        }
    }
}

//====================================================================================//
//                                 Lexer Struct                                       //
//====================================================================================//

pub struct Lexer<'a> {
    content: &'a str,
    cursor: usize,
    line: usize, 
}

impl Default for Lexer<'_> {
    fn default() -> Self {
        Lexer {
            content: "",
            cursor: 0,
            line: 0
        }
    }
}

impl<'a> Lexer<'a> {

    fn new(
        content: &'a str,
    ) -> Lexer<'a> {
        Lexer {
            content: content, 
            cursor: 0,
            line: 0
        }
    }

    fn set_content(&mut self, content: &'a str) {
        self.content = content;
    }

    fn get_content(&self) -> &'a str {
        self.content 
    }

    fn next_char(&mut self) -> Option<char> {
        let next = self.content[self.get_cursor()..].chars().next();
        match next {
            Some(n) => {
                self.cursor += 1;
                Some(n)
            },
            None => None
        }
    }

    fn isspace(&mut self, c: char) -> bool {
        c == ' '
    }

    fn starts_with<'b>(&mut self, prefix: &'b str) -> bool {
        let prefix_len: usize = prefix.len();

        if prefix_len == 0 {
            return false;
        }

        if self.get_cursor() + prefix_len - 1 >= self.get_content().len() {
            return false;
        }

        false
    }

    fn get_cursor(&self) -> usize { self.cursor }

}
