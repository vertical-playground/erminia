use erminia::config::CompilerPass;

fn parse_example(filename: &str) -> (bool, usize) {
    let manifest_dir = env!("CARGO_MANIFEST_DIR");
    let path = std::path::Path::new(manifest_dir)
        .join("../../examples")
        .join(filename);
    let source = std::fs::read_to_string(&path)
        .unwrap_or_else(|_| panic!("failed to read example: {filename}"));
    let mut parser = erminia::syntax::Parser::new(&source);
    let program = parser.parse();
    let ok = program.is_ok();
    let diag_count = parser.get_diagnostics().get(CompilerPass::Parser).len();
    (ok, diag_count)
}

mod test_cli {
    use super::*;

    #[test]
    fn basic_objects_parses_ok() {
        let (ok, count) = parse_example("basic_objects.erm");
        assert!(ok, "basic_objects.erm should parse without errors");
        assert_eq!(count, 0);
        assert!(ok);
    }

    #[test]
    fn full_problem_parses_ok() {
        let (ok, count) = parse_example("full_problem.erm");
        assert!(ok, "full_problem.erm should parse without errors");
        assert_eq!(count, 0);
        assert!(ok);
    }

    #[test]
    fn comment_parses_ok() {
        let (ok, count) = parse_example("comment.erm");
        assert!(ok, "comment.erm should parse without errors");
        assert_eq!(count, 0);
        assert!(ok);
    }

    #[test]
    fn bad_missing_semicolons_has_diagnostics() {
        let (ok, count) = parse_example("bad_missing_semicolons.erm");
        assert!(!ok, "bad_missing_semicolons.erm should fail to parse");
        assert_eq!(count, 1);
        assert!(!ok);
    }

    #[test]
    fn bad_malformed_shape_has_diagnostics() {
        let (ok, count) = parse_example("bad_malformed_shape.erm");
        assert!(!ok, "bad_malformed_shape.erm should fail to parse");
        assert_eq!(count, 2);
        assert!(!ok);
    }

    #[test]
    fn bad_object_desc_has_diagnostics() {
        let (ok, count) = parse_example("bad_object_desc.erm");
        assert!(!ok, "bad_object_desc.erm should fail to parse");
        assert_eq!(count, 4);
        assert!(!ok);
    }

    #[test]
    fn bad_wrong_stmt_keyword_has_diagnostics() {
        let (ok, count) = parse_example("bad_wrong_stmt_keyword.erm");
        assert!(!ok, "bad_wrong_stmt_keyword.erm should fail to parse");
        assert_eq!(count, 2);
        assert!(!ok);
    }

    #[test]
    fn bad_multiple_errors_has_diagnostics() {
        let (ok, count) = parse_example("bad_multiple_errors.erm");
        assert!(!ok, "bad_multiple_errors.erm should fail to parse");
        assert_eq!(count, 1);
        assert!(!ok);
    }

    #[test]
    fn hello_has_diagnostics() {
        let (ok, count) = parse_example("hello.erm");
        assert_eq!(count, 1);
        assert!(!ok);
    }

    #[test]
    fn var_and_calls_has_diagnostics() {
        let (ok, count) = parse_example("var_and_calls.erm");
        assert_eq!(count, 2);
        assert!(!ok);
    }

    #[test]
    fn comprehensions_has_diagnostics() {
        let (ok, count) = parse_example("comprehensions.erm");
        assert_eq!(count, 1);
        assert!(!ok);
    }

    #[test]
    fn template_script_has_diagnostics() {
        let (ok, count) = parse_example("template_script.erm");
        assert_eq!(count, 1);
        assert!(!ok);
    }
}
