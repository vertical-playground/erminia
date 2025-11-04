use crate::diagnostics::diagnostics::Location;
use crate::error::lexer_error::LexerError;
use std::fmt;

// ==================================================================================== //
// Identifier Extension Trait                                                           //
// ==================================================================================== //

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
            }

            None => false,
        }
    }
}

// ==================================================================================== //
// TokenKind Enum                                                                       //
// ==================================================================================== //

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TokenKind {
    Error,
    Plus,
    Minus,
    Increment,
    Decrement,
    Multi,
    Div,
    FlatDiv,
    Mod,
    Greater,
    Lesser,
    ShiftLeft,
    ShiftRight,
    Member,
    Not,
    NotEquals,
    Pipe,
    ProblemDef,
    LetKwd,
    Object,
    SuperObject,
    ObjectShape,
    ObjectColor,
    ProblemExample,
    ProblemSolution,
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
    LeftArrow,
    CommentStart,
    CommentEnd,
    NewLine,
    Tab,
    Int,
    Float,
    Ident,
    String,
    EOF,
    START,
}

impl fmt::Display for TokenKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match *self {
            TokenKind::START => "[START]",
            TokenKind::Error => "[ERROR]",
            TokenKind::ProblemDef => "def",
            TokenKind::LetKwd => "let",
            TokenKind::Object => "object",
            TokenKind::SuperObject => "superobject",
            TokenKind::ObjectShape => "shape",
            TokenKind::ObjectColor => "color",
            TokenKind::ProblemExample => "example",
            TokenKind::ProblemSolution => "solution",
            TokenKind::ProblemInput => "input",
            TokenKind::ProblemOutput => "output",
            TokenKind::Plus => "+",
            TokenKind::Minus => "-",
            TokenKind::Increment => "++",
            TokenKind::Decrement => "--",
            TokenKind::Multi => "*",
            TokenKind::Div => "/",
            TokenKind::FlatDiv => "//",
            TokenKind::Mod => "%",
            TokenKind::Greater => ">",
            TokenKind::Lesser => "<",
            TokenKind::ShiftLeft => "<<",
            TokenKind::ShiftRight => ">>",
            TokenKind::Member => ".",
            TokenKind::Not => "!",
            TokenKind::NotEquals => "!=",
            TokenKind::Equals => "=",
            TokenKind::LeftPar => "(",
            TokenKind::RightPar => ")",
            TokenKind::LeftBracket => "[",
            TokenKind::RightBracket => "]",
            TokenKind::LeftBrace => "{",
            TokenKind::RightBrace => "}",
            TokenKind::Comma => ",",
            TokenKind::Colon => ":",
            TokenKind::SemiColon => ";",
            TokenKind::Range => "..",
            TokenKind::Pipe => "|",
            TokenKind::LeftArrow => "<-",
            TokenKind::CommentStart => "(*",
            TokenKind::CommentEnd => "*)",
            TokenKind::NewLine => "\n",
            TokenKind::Tab => "\t",
            TokenKind::Int => "[INT]",
            TokenKind::Float => "[FLOAT]",
            TokenKind::Ident => "[IDENT]",
            TokenKind::String => "[STRING]",
            TokenKind::EOF => "[EOF]",
        };

        fmt::Display::fmt(s, f)
    }
}

impl std::str::FromStr for TokenKind {
    type Err = LexerError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let token = match s {
            "[START]" => TokenKind::START,
            "def" => TokenKind::ProblemDef,
            "let" => TokenKind::LetKwd,
            "object" => TokenKind::Object,
            "superobject" => TokenKind::SuperObject,
            "shape" => TokenKind::ObjectShape,
            "color" => TokenKind::ObjectColor,
            "example" => TokenKind::ProblemExample,
            "solution" => TokenKind::ProblemSolution,
            "input" => TokenKind::ProblemInput,
            "output" => TokenKind::ProblemOutput,
            "+" => TokenKind::Plus,
            "-" => TokenKind::Minus,
            "++" => TokenKind::Increment,
            "--" => TokenKind::Decrement,
            "*" => TokenKind::Multi,
            "/" => TokenKind::Div,
            "//" => TokenKind::FlatDiv,
            "%" => TokenKind::Mod,
            "<<" => TokenKind::ShiftRight,
            ">>" => TokenKind::ShiftLeft,
            ">" => TokenKind::Greater,
            "<" => TokenKind::Lesser,
            "." => TokenKind::Member,
            "!" => TokenKind::Not,
            "!=" => TokenKind::NotEquals,
            "=" => TokenKind::Equals,
            "(" => TokenKind::LeftPar,
            ")" => TokenKind::RightPar,
            "[" => TokenKind::LeftBracket,
            "]" => TokenKind::RightBracket,
            "{" => TokenKind::LeftBrace,
            "}" => TokenKind::RightBrace,
            "," => TokenKind::Comma,
            ";" => TokenKind::SemiColon,
            ":" => TokenKind::Colon,
            ".." => TokenKind::Range,
            "|" => TokenKind::Pipe,
            "<-" => TokenKind::LeftArrow,
            "(*" => TokenKind::CommentStart,
            "*)" => TokenKind::CommentEnd,
            "\n" => TokenKind::NewLine,
            "\t" => TokenKind::Tab,
            _ => {
                if s.parse::<i64>().is_ok() {
                    TokenKind::Int
                } else if s.parse::<f64>().is_ok() {
                    TokenKind::Float
                } else if s.is_valid_indentifier() {
                    TokenKind::Ident
                } else {
                    return Err(LexerError::TokenError(Location::new(Position::default())));
                }
            }
        };

        Ok(token)
    }
}

// ==================================================================================== //
// Position Struct                                                                      //
// ==================================================================================== //

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Position {
    x: usize,
    y: usize,
}

impl Default for Position {
    fn default() -> Self {
        Position { x: 0, y: 0 }
    }
}

impl Position {
    pub fn new(x: usize, y: usize) -> Position {
        Position { x: x, y: y }
    }
}

impl std::fmt::Display for Position {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = format!("({}, {})", self.x, self.y);

        fmt::Display::fmt(&s, f)
    }
}

// ==================================================================================== //
// Token Struct                                                                         //
// ==================================================================================== //

#[warn(unused)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Token<'a> {
    kind: TokenKind,
    pub text: &'a str,
    size: usize,
    start: Position,
    end: Position,
}

impl Default for Token<'_> {
    fn default() -> Self {
        Token {
            kind: TokenKind::START,
            text: "",
            size: 0,
            start: Position::new(0, 0),
            end: Position::new(0, 0),
        }
    }
}

impl Token<'_> {
    pub fn new<'a>(kind: TokenKind, text: &'a str, row: usize, col: usize) -> Token<'a> {
        let size = text.len();
        let start = Position::new(col, row);
        let end = Position::new(col + size, row);
        Token {
            kind: kind,
            text: text,
            size: size,
            start: start,
            end: end,
        }
    }

    // fn new_verbose<'a>(
    //     kind: TokenKind,
    //     text: &'a str,
    //     size: usize,
    //     start: Position,
    //     end: Position,
    // ) -> Token<'a> {
    //     Token {
    //         kind: kind,
    //         text: text,
    //         size: size,
    //         start: start,
    //         end: end,
    //     }
    // }

    pub fn get_kind(&self) -> TokenKind {
        self.kind
    }

    pub fn get_text(&self) -> &str {
        self.text
    }

    pub fn get_size(&self) -> usize {
        self.size
    }

    pub fn get_start(&self) -> Position {
        self.start
    }

    pub fn get_end(&self) -> Position {
        self.end
    }
}

impl std::fmt::Display for Token<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = format!(
            "Token({}, {}, {}, {}, {})",
            self.kind, self.text, self.size, self.start, self.end
        );

        fmt::Display::fmt(&s, f)
    }
}

// ==================================================================================== //
// Token Test Suite                                                                     //
// ==================================================================================== //

#[cfg(test)]
mod tests {
    use super::*;
    use crate::error::lexer_error::LexerResult;
    use std::str::FromStr;

    // Token Kind Tests
    fn check_tk_eq(input: &str, expected: LexerResult<TokenKind>) {
        let actual = TokenKind::from_str(input);
        assert_eq!(expected.expect(""), actual.expect(""));
    }

    fn check_tk_ne(input: &str, not_expected: LexerResult<TokenKind>) {
        let actual = TokenKind::from_str(input);
        assert_ne!(not_expected.expect(""), actual.expect(""));
    }

    #[test]
    fn test_tk_ident() {
        check_tk_eq("dwad123", Ok(TokenKind::Ident))
    }

    #[test]
    fn test_tk_not_ident() {
        check_tk_ne("123", Ok(TokenKind::Ident))
    }

    #[test]
    fn test_tk_int() {
        check_tk_eq("123", Ok(TokenKind::Int))
    }

    #[test]
    fn test_tk_float() {
        check_tk_eq("123.123", Ok(TokenKind::Float))
    }

    #[test]
    fn test_tk_def() {
        check_tk_eq("def", Ok(TokenKind::ProblemDef))
    }

    #[test]
    fn test_tk_let() {
        check_tk_eq("let", Ok(TokenKind::LetKwd))
    }

    #[test]
    fn test_tk_object() {
        check_tk_eq("object", Ok(TokenKind::Object))
    }

    #[test]
    fn test_tk_superobject() {
        check_tk_eq("superobject", Ok(TokenKind::SuperObject))
    }

    #[test]
    fn test_tk_shape() {
        check_tk_eq("shape", Ok(TokenKind::ObjectShape))
    }

    #[test]
    fn test_tk_color() {
        check_tk_eq("color", Ok(TokenKind::ObjectColor))
    }

    #[test]
    fn test_tk_example() {
        check_tk_eq("example", Ok(TokenKind::ProblemExample))
    }

    #[test]
    fn test_tk_input() {
        check_tk_eq("input", Ok(TokenKind::ProblemInput))
    }

    #[test]
    fn test_tk_output() {
        check_tk_eq("output", Ok(TokenKind::ProblemOutput))
    }

    #[test]
    fn test_tk_equals() {
        check_tk_eq("=", Ok(TokenKind::Equals))
    }

    #[test]
    fn test_tk_leftpar() {
        check_tk_eq("(", Ok(TokenKind::LeftPar))
    }

    #[test]
    fn test_tk_rightpar() {
        check_tk_eq(")", Ok(TokenKind::RightPar))
    }

    #[test]
    fn test_tk_leftbracket() {
        check_tk_eq("[", Ok(TokenKind::LeftBracket))
    }

    #[test]
    fn test_tk_rightbracket() {
        check_tk_eq("]", Ok(TokenKind::RightBracket))
    }

    #[test]
    fn test_tk_leftbrace() {
        check_tk_eq("{", Ok(TokenKind::LeftBrace))
    }

    #[test]
    fn test_tk_rightbrace() {
        check_tk_eq("}", Ok(TokenKind::RightBrace))
    }

    #[test]
    fn test_tk_comma() {
        check_tk_eq(",", Ok(TokenKind::Comma))
    }

    #[test]
    fn test_tk_colon() {
        check_tk_eq(";", Ok(TokenKind::SemiColon))
    }

    #[test]
    fn test_tk_semicolon() {
        check_tk_eq(":", Ok(TokenKind::Colon))
    }

    #[test]
    fn test_tk_range() {
        check_tk_eq("..", Ok(TokenKind::Range))
    }

    #[test]
    fn test_tk_commentstart() {
        check_tk_eq("(*", Ok(TokenKind::CommentStart))
    }

    #[test]
    fn test_tk_commentend() {
        check_tk_eq("*)", Ok(TokenKind::CommentEnd))
    }

    #[test]
    fn test_tk_newline() {
        check_tk_eq("\n", Ok(TokenKind::NewLine))
    }

    #[test]
    fn test_tk_tab() {
        check_tk_eq("\t", Ok(TokenKind::Tab))
    }

    // Token Tests
    fn check_t_eq(input: &str, expected: LexerResult<Token>) {
        let kind = TokenKind::from_str(input).expect("");
        let actual = Token::new(kind, input, 0, 0);

        assert_eq!(actual, expected.expect(""));
    }

    #[test]
    fn test_t_tab() {
        check_t_eq(
            "\t",
            Ok(Token {
                kind: TokenKind::Tab,
                text: "\t",
                size: "\t".len(),
                start: Position::new(0, 0),
                end: Position::new("\t".len(), 0),
            }),
        )
    }

    #[test]
    fn test_t_def() {
        check_t_eq(
            "def",
            Ok(Token {
                kind: TokenKind::ProblemDef,
                text: "def",
                size: "def".len(),
                start: Position::new(0, 0),
                end: Position::new("def".len(), 0),
            }),
        );
    }

    #[test]
    fn test_t_let() {
        check_t_eq(
            "let",
            Ok(Token {
                kind: TokenKind::LetKwd,
                text: "let",
                size: "let".len(),
                start: Position::new(0, 0),
                end: Position::new("let".len(), 0),
            }),
        );
    }

    #[test]
    fn test_t_ident() {
        check_t_eq(
            "dwad123",
            Ok(Token {
                kind: TokenKind::Ident,
                text: "dwad123",
                size: "dwad123".len(),
                start: Position::new(0, 0),
                end: Position::new("dwad123".len(), 0),
            }),
        );
    }

    #[test]
    fn test_t_int() {
        check_t_eq(
            "123",
            Ok(Token {
                kind: TokenKind::Int,
                text: "123",
                size: "123".len(),
                start: Position::new(0, 0),
                end: Position::new("123".len(), 0),
            }),
        );
    }

    #[test]
    fn test_t_float() {
        check_t_eq(
            "123.123",
            Ok(Token {
                kind: TokenKind::Float,
                text: "123.123",
                size: "123.123".len(),
                start: Position::new(0, 0),
                end: Position::new("123.123".len(), 0),
            }),
        );
    }

    #[test]
    fn test_t_object() {
        check_t_eq(
            "object",
            Ok(Token {
                kind: TokenKind::Object,
                text: "object",
                size: "object".len(),
                start: Position::new(0, 0),
                end: Position::new("object".len(), 0),
            }),
        );
    }

    #[test]
    fn test_t_superobject() {
        let kwd = "superobject";
        check_t_eq(
            kwd,
            Ok(Token {
                kind: TokenKind::SuperObject,
                text: kwd,
                size: kwd.len(),
                start: Position::new(0, 0),
                end: Position::new(kwd.len(), 0),
            }),
        );
    }

    #[test]
    fn test_t_shape() {
        let kwd = "shape";
        check_t_eq(
            kwd,
            Ok(Token {
                kind: TokenKind::from_str(kwd).expect(""),
                text: kwd,
                size: kwd.len(),
                start: Position::new(0, 0),
                end: Position::new(kwd.len(), 0),
            }),
        );
    }

    #[test]
    fn test_t_color() {
        let kwd = "color";
        check_t_eq(
            kwd,
            Ok(Token {
                kind: TokenKind::from_str(kwd).expect(""),
                text: kwd,
                size: kwd.len(),
                start: Position::new(0, 0),
                end: Position::new(kwd.len(), 0),
            }),
        );
    }

    #[test]
    fn test_t_example() {
        let kwd = "example";
        check_t_eq(
            kwd,
            Ok(Token {
                kind: TokenKind::from_str(kwd).expect(""),
                text: kwd,
                size: kwd.len(),
                start: Position::new(0, 0),
                end: Position::new(kwd.len(), 0),
            }),
        );
    }

    #[test]
    fn test_t_input() {
        let kwd = "input";
        check_t_eq(
            kwd,
            Ok(Token {
                kind: TokenKind::from_str(kwd).expect(""),
                text: kwd,
                size: kwd.len(),
                start: Position::new(0, 0),
                end: Position::new(kwd.len(), 0),
            }),
        );
    }

    #[test]
    fn test_t_output() {
        let kwd = "output";
        check_t_eq(
            kwd,
            Ok(Token {
                kind: TokenKind::from_str(kwd).expect(""),
                text: kwd,
                size: kwd.len(),
                start: Position::new(0, 0),
                end: Position::new(kwd.len(), 0),
            }),
        );
    }

    #[test]
    fn test_t_equals() {
        let kwd = "=";
        check_t_eq(
            kwd,
            Ok(Token {
                kind: TokenKind::from_str(kwd).expect(""),
                text: kwd,
                size: kwd.len(),
                start: Position::new(0, 0),
                end: Position::new(kwd.len(), 0),
            }),
        );
    }

    #[test]
    fn test_t_leftpar() {
        let kwd = "(";
        check_t_eq(
            kwd,
            Ok(Token {
                kind: TokenKind::from_str(kwd).expect(""),
                text: kwd,
                size: kwd.len(),
                start: Position::new(0, 0),
                end: Position::new(kwd.len(), 0),
            }),
        );
    }

    #[test]
    fn test_t_rightpar() {
        let kwd = ")";
        check_t_eq(
            kwd,
            Ok(Token {
                kind: TokenKind::from_str(kwd).expect(""),
                text: kwd,
                size: kwd.len(),
                start: Position::new(0, 0),
                end: Position::new(kwd.len(), 0),
            }),
        );
    }

    #[test]
    fn test_t_leftbracket() {
        let kwd = "[";
        check_t_eq(
            kwd,
            Ok(Token {
                kind: TokenKind::from_str(kwd).expect(""),
                text: kwd,
                size: kwd.len(),
                start: Position::new(0, 0),
                end: Position::new(kwd.len(), 0),
            }),
        );
    }

    #[test]
    fn test_t_rightbracket() {
        let kwd = "]";
        check_t_eq(
            kwd,
            Ok(Token {
                kind: TokenKind::from_str(kwd).expect(""),
                text: kwd,
                size: kwd.len(),
                start: Position::new(0, 0),
                end: Position::new(kwd.len(), 0),
            }),
        );
    }

    #[test]
    fn test_t_leftbrace() {
        let kwd = "{";
        check_t_eq(
            kwd,
            Ok(Token {
                kind: TokenKind::from_str(kwd).expect(""),
                text: kwd,
                size: kwd.len(),
                start: Position::new(0, 0),
                end: Position::new(kwd.len(), 0),
            }),
        );
    }

    #[test]
    fn test_t_rightbrace() {
        let kwd = "}";
        check_t_eq(
            kwd,
            Ok(Token {
                kind: TokenKind::from_str(kwd).expect(""),
                text: kwd,
                size: kwd.len(),
                start: Position::new(0, 0),
                end: Position::new(kwd.len(), 0),
            }),
        );
    }

    #[test]
    fn test_t_comma() {
        let kwd = ",";
        check_t_eq(
            kwd,
            Ok(Token {
                kind: TokenKind::from_str(kwd).expect(""),
                text: kwd,
                size: kwd.len(),
                start: Position::new(0, 0),
                end: Position::new(kwd.len(), 0),
            }),
        );
    }

    #[test]
    fn test_t_colon() {
        let kwd = ";";
        check_t_eq(
            kwd,
            Ok(Token {
                kind: TokenKind::from_str(kwd).expect(""),
                text: kwd,
                size: kwd.len(),
                start: Position::new(0, 0),
                end: Position::new(kwd.len(), 0),
            }),
        );
    }

    #[test]
    fn test_t_semicolon() {
        let kwd = ";";
        check_t_eq(
            kwd,
            Ok(Token {
                kind: TokenKind::from_str(kwd).expect(""),
                text: kwd,
                size: kwd.len(),
                start: Position::new(0, 0),
                end: Position::new(kwd.len(), 0),
            }),
        );
    }

    #[test]
    fn test_t_range() {
        let kwd = "..";
        check_t_eq(
            kwd,
            Ok(Token {
                kind: TokenKind::from_str(kwd).expect(""),
                text: kwd,
                size: kwd.len(),
                start: Position::new(0, 0),
                end: Position::new(kwd.len(), 0),
            }),
        );
    }

    #[test]
    fn test_t_commentstart() {
        let kwd = "(*";
        check_t_eq(
            kwd,
            Ok(Token {
                kind: TokenKind::from_str(kwd).expect(""),
                text: kwd,
                size: kwd.len(),
                start: Position::new(0, 0),
                end: Position::new(kwd.len(), 0),
            }),
        );
    }

    #[test]
    fn test_t_commentend() {
        let kwd = "*)";
        check_t_eq(
            kwd,
            Ok(Token {
                kind: TokenKind::from_str(kwd).expect(""),
                text: kwd,
                size: kwd.len(),
                start: Position::new(0, 0),
                end: Position::new(kwd.len(), 0),
            }),
        );
    }

    #[test]
    fn test_t_newline() {
        let kwd = "\n";
        check_t_eq(
            kwd,
            Ok(Token {
                kind: TokenKind::from_str(kwd).expect(""),
                text: kwd,
                size: kwd.len(),
                start: Position::new(0, 0),
                end: Position::new(kwd.len(), 0),
            }),
        );
    }
}
