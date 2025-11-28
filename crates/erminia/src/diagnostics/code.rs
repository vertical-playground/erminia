use derive_more::Display;

// ==================================================================================== //
// Struct                                                                               //
// ==================================================================================== //

#[derive(Display)]
pub enum Code {
    E000X,
    W000X,
    N000X,
    H000X,
}

pub enum DiagnosticLevel {
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
            Code::E000X => "An error occurred.".to_string(),
            Code::W000X => "This is a warning.".to_string(),
            Code::N000X => "This is a note.".to_string(),
            Code::H000X => "This is a help message.".to_string(),
        }
    }
}

impl DiagnosticLevel {
    pub fn from_code(code: &Code) -> Self {
        if code.to_string().starts_with('E') {
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
