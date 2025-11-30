use derive_more::Display;

// ==================================================================================== //
// Struct                                                                               //
// ==================================================================================== //

#[derive(Debug, Clone, Copy, PartialEq, Eq, Display, PartialOrd, Ord)]
pub enum Code {
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

#[derive(Display, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum DiagnosticLevel {
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

mod tests {

    #[test]
    fn test_diagnostic_level_hierarchy() {
        use super::{Code, DiagnosticLevel};

        let internal_code = Code::I0001;
        let error_code = Code::E0001;
        let warning_code = Code::W000X;
        let note_code = Code::N000X;
        let help_code = Code::H000X;

        let internal_level = DiagnosticLevel::from_code(&internal_code);
        let error_level = DiagnosticLevel::from_code(&error_code);
        let warning_level = DiagnosticLevel::from_code(&warning_code);
        let note_level = DiagnosticLevel::from_code(&note_code);
        let help_level = DiagnosticLevel::from_code(&help_code);

        assert!(internal_level < error_level);
        assert!(error_level < warning_level);
        assert!(warning_level < note_level);
        assert!(note_level < help_level);
    }

    #[test]
    fn test_code_hierarchy() {
        use super::Code;

        let code0 = Code::I0001;
        let code1 = Code::E0001;
        let code2 = Code::E0002;
        let code3 = Code::W000X;
        let code4 = Code::N000X;

        assert!(code0 < code1);
        assert!(code1 < code2);
        assert!(code2 < code3);
        assert!(code3 < code4);
    }
}
