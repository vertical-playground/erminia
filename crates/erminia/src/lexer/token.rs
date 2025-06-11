use crate::error::lexer_error::LexerError;

// ====================================================================================//
//                            Identifier Extension Trait                               //
// ====================================================================================//

trait Identifier {
    fn is_valid_indentifier(&self) -> bool;
}

impl Identifier for &str {
    fn is_valid_indentifier(&self) -> bool {
        let mut chars = self.chars();
        match chars.next() {
            Some(first) => {
                if first.is_alphabetic() && chars.all(|c| c.is_ascii_alphanumeric()) {
                    return true;
                }

                false
            },

            None => false
        }
    }
}

// ====================================================================================//
//                            TokenKind Enum                                           //
// ====================================================================================//

#[derive(Debug, PartialEq)]
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
    start: Position,
    end: Position
}

impl Default for Token<'_> {
    fn default() -> Self {
        Token {
            kind: TokenKind::Tab,
            text: "",
            size: 0,
            start: Position::new(0,0),
            end: Position::new(0,0)
        }
    }
}

impl Token<'_> {
    fn new<'a>(
        kind: TokenKind,
        text: &'a str,
        size: usize,
        start: Position,
        end: Position
    ) -> Token<'a> {
        Token {
            kind: kind,
            text: text,
            size: size,
            start: start,
            end: end
        }
    }
}

//====================================================================================//
//                                 Unit Test                                          //
//====================================================================================//

#[cfg(test)]
mod tests {
    use super::*;
    use std::str::FromStr;
    use crate::error::lexer_error::LexerResult;
    
    fn check_eq(input: &str, expected: LexerResult<TokenKind>) {
        let actual = TokenKind::from_str(input);
        assert_eq!(expected.expect(""), actual.expect(""));
    }

    fn check_ne(input: &str, not_expected: LexerResult<TokenKind>) {
        let actual = TokenKind::from_str(input);
        assert_ne!(not_expected.expect(""), actual.expect(""));
    }

    #[test]
    fn test_ident() {
        check_eq("dwad123", Ok(TokenKind::Ident))
    }

    #[test]
    fn test_not_ident() {
        check_ne("123", Ok(TokenKind::Ident))
    }

    #[test]
    fn test_int() {
        check_eq("123", Ok(TokenKind::Int))
    }

    #[test]
    fn test_float() {
        check_eq("123.123", Ok(TokenKind::Float))
    }

    #[test]
    fn test_def() {
        check_eq("def", Ok(TokenKind::ProblemDef))
    }

    #[test]
    fn test_let() {
        check_eq("let", Ok(TokenKind::LetKwd))
    }

    #[test]
    fn test_object() {
        check_eq("object", Ok(TokenKind::Object))
    }

    #[test]
    fn test_superobject() {
        check_eq("superobject", Ok(TokenKind::SuperObject))
    }

    #[test]
    fn test_shape() {
        check_eq("shape", Ok(TokenKind::ObjectShape))
    }

    #[test]
    fn test_color() {
        check_eq("color", Ok(TokenKind::ObjectColor))
    }

    #[test]
    fn test_example() {
        check_eq("example", Ok(TokenKind::ProblemExample))
    }

    #[test]
    fn test_input() {
        check_eq("input", Ok(TokenKind::ProblemInput))
    }

    #[test]
    fn test_output() {
        check_eq("output", Ok(TokenKind::ProblemOutput))
    }

    #[test]
    fn test_equals() {
        check_eq("==", Ok(TokenKind::Equals))
    }

    #[test]
    fn test_leftpar() {
        check_eq("(", Ok(TokenKind::LeftPar))
    }

    #[test]
    fn test_rightpar() {
        check_eq(")", Ok(TokenKind::RightPar))
    }

    #[test]
    fn test_leftbracket() {
        check_eq("[", Ok(TokenKind::LeftBracket))
    }

    #[test]
    fn test_rightbracket() {
        check_eq("]", Ok(TokenKind::RightBracket))
    }

    #[test]
    fn test_leftbrace() {
        check_eq("{", Ok(TokenKind::LeftBrace))
    }

    #[test]
    fn test_rightbrace() {
        check_eq("}", Ok(TokenKind::RightBrace))
    }

    #[test]
    fn test_comma() {
        check_eq(",", Ok(TokenKind::Comma))
    }

    #[test]
    fn test_colon() {
        check_eq(";", Ok(TokenKind::Colon))
    }

    #[test]
    fn test_semicolon() {
        check_eq(":", Ok(TokenKind::SemiColon))
    }

    #[test]
    fn test_range() {
        check_eq("..", Ok(TokenKind::Range))
    }

    #[test]
    fn test_commentstart() {
        check_eq("(*", Ok(TokenKind::CommentStart))
    }

    #[test]
    fn test_commentend() {
        check_eq("*)", Ok(TokenKind::CommentEnd))
    }

    #[test]
    fn test_newline() {
        check_eq("\n", Ok(TokenKind::NewLine))
    }

    #[test]
    fn test_tab() {
        check_eq("\t", Ok(TokenKind::Tab))
    }
}
