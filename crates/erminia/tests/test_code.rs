use erminia::diagnostics::code::*;

mod test_code {

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
