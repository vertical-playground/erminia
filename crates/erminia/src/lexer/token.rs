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
    Poisoned,
}

impl fmt::Display for TokenKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match *self {
            TokenKind::START => "[START]",
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
            TokenKind::Poisoned => "[POISONED]",
        };

        fmt::Display::fmt(s, f)
    }
}

impl std::str::FromStr for TokenKind {
    type Err = String;

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
                    return Err(format!("Unknown token kind: {}", s));
                }
            }
        };

        Ok(token)
    }
}

// ==================================================================================== //
// Position Struct                                                                      //
// ==================================================================================== //

#[derive(Default, Debug, Clone, Copy, PartialEq)]
pub struct Position {
    x: usize,
    y: usize,
}

impl Position {
    pub fn new(x: usize, y: usize) -> Position {
        Position { x, y }
    }

    pub fn x(&self) -> usize {
        self.x
    }

    pub fn y(&self) -> usize {
        self.y
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
    pub kind: TokenKind,
    pub text: &'a str,
    pub size: usize,
    pub start: Position,
    pub end: Position,
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
            kind,
            text,
            size,
            start,
            end,
        }
    }

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
