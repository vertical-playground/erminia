use derive_more::Display;

// ==================================================================================== //
// Struct                                                                               //
// ==================================================================================== //

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, Display, PartialOrd, Ord)]
pub enum Code {
    #[default]
    I0001, // Internal Compiler Error
    E0001, // Expected keyword token but found something else
    E0002, // Expected symbol token but found something else
    E0003, // Expected integer constant but found something else
    E0004, // Poisoned AST Node detected
    E000X,
    W000X,
    N000X,
    H000X,
}

#[derive(Default, Display, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum DiagnosticLevel {
    #[default]
    Internal,
    Error,
    Warning,
    Note,
    Help,
}

// ==================================================================================== //
// Traits                                                                               //
// ==================================================================================== //

pub trait FromCode {
    fn from_code(code: &Code) -> Self;
}

// ==================================================================================== //
// Implementations                                                                      //
// ==================================================================================== //

impl FromCode for DiagnosticLevel {
    fn from_code(code: &Code) -> Self {
        DiagnosticLevel::from_code(code)
    }
}

impl FromCode for String {
    fn from_code(code: &Code) -> Self {
        match code {
            Code::I0001 => "Internal Compiler Error occurred".to_string(),
            Code::E0001 => "Expected keyword but something else was found".to_string(),
            Code::E0002 => "Expected symbol but something else was found".to_string(),
            Code::E0003 => "Expected integer constant but something else was found".to_string(),
            Code::E0004 => "Poisoned AST Node detected".to_string(),
            Code::E000X => "An error occurred.".to_string(),
            Code::W000X => "This is a warning.".to_string(),
            Code::N000X => "This is a note.".to_string(),
            Code::H000X => "This is a help message.".to_string(),
        }
    }
}

impl DiagnosticLevel {
    pub fn from_code(code: &Code) -> Self {
        if code.to_string().starts_with('I') {
            DiagnosticLevel::Internal
        } else if code.to_string().starts_with('E') {
            DiagnosticLevel::Error
        } else if code.to_string().starts_with('W') {
            DiagnosticLevel::Warning
        } else if code.to_string().starts_with('N') {
            DiagnosticLevel::Note
        } else {
            DiagnosticLevel::Help
        }
    }
}
