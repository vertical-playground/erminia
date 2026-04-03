use std::fs::File;
use std::io::Read;
use std::path::Path;

use erminia::diag;
use erminia::diagnostics::{DiagnosticAccumulator, Span};
use erminia::lexer::lex::Lexer;

fn get_file_from_path(path: &Path, diag: &mut DiagnosticAccumulator) -> Result<File, ()> {
    if let Ok(file) = File::open(path) {
        Ok(file)
    } else {
        diag!(
            Internal,
            I0002,
            FileNotFound(path.display().to_string()),
            &mut Lexer::default(),
            diag,
            Span::default()
        );

        Err(())
    }
}
fn get_string_from_file(path: &Path, file: &mut File, diag: &mut DiagnosticAccumulator) -> String {
    let mut content: String = String::new();

    if file.read_to_string(&mut content).is_err() {
        diag!(
            Internal,
            I0002,
            FileNotFound(path.display().to_string()),
            &mut Lexer::default(),
            diag,
            Span::default()
        );
    };

    content
}

fn validate_path(path: &Path, diag: &mut DiagnosticAccumulator) {
    if path.extension().is_none_or(|ext| ext == "erm") {
        diag!(
            Internal,
            I0002,
            FileNotFound(path.display().to_string()),
            &mut Lexer::default(),
            diag,
            Span::default()
        );
    }
}

pub fn from_file(path_str: String, diag: &mut DiagnosticAccumulator) -> Result<String, ()> {
    let path: &Path = Path::new(&path_str);

    validate_path(path, diag);

    if let Ok(mut file) = get_file_from_path(path, diag) {
        Ok(get_string_from_file(path, &mut file, diag))
    } else {
        Err(())
    }
}
