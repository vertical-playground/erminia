use erminia::ast::ast::BoxAST;
use erminia::config::CompilerPass;
use erminia::diagnostics::DiagnosticAccumulator;
use erminia::lexer::lex::Lexer;
use erminia::lexer::token::TokenKind;
use erminia::syntax::consumers::*;
use erminia::syntax::parse::*;
use erminia::types::ErminiaType;

#[cfg(test)]
mod test_parser {
    use super::*;

    fn check_no_err_single_ast<'a, F>(text: &'a str, parser: F)
    where
        F: FnOnce(&mut Lexer, &mut DiagnosticAccumulator) -> BoxAST<'a>,
    {
        let mut tokens = Lexer::new(text);
        let mut diag = DiagnosticAccumulator::new();

        tokens.advance();

        let res = parser(&mut tokens, &mut diag);

        if res.is_err() {
            println!("{:?}", res);
            for d in diag.get(CompilerPass::Parser) {
                println!("{}", d);
            }
        }

        assert!(res.is_ok())
    }

    // fn check_no_err_multiple_ast<'a, F>(text: &'a str, parser: F)
    // where
    //     F: FnOnce(&mut Lexer, &mut DiagnosticAccumulator) -> Vec<BoxAST<'a>>,
    // {
    //     let mut tokens = Lexer::new(text);
    //     let mut diag = DiagnosticAccumulator::new();
    //
    //     tokens.advance();
    //
    //     let res = parser(&mut tokens, &mut diag);
    //
    //     println!("{:?}", res);
    //
    //     if res.iter().any(|ast| ast.is_err()) {
    //         println!("{:?}: \n {:?}", text, res);
    //         for d in diag.get(CompilerPass::AST) {
    //             println!("{}", d);
    //         }
    //     }
    //
    //     assert!(res.iter().all(|ast| ast.is_ok()))
    // }

    // fn check_no_err_single_ast_with_syntax_ret<'a, F>(
    //     text: &'a str,
    //     parser: F,
    // ) where
    //     F: FnOnce(&mut Lexer, &mut DiagnosticAccumulator) -> (BoxAST<'a>, Vec<ErminiaType>),
    // {
    //     let mut tokens = Lexer::new(text);
    //     let mut diag = DiagnosticAccumulator::new();
    //
    //     tokens.advance();
    //
    //     let (res, _) = parser(&mut tokens, &mut diag);
    //
    //     if res.is_err() {
    //         println!("Error in parsing for input {:?}: \n {:?}", text, res);
    //         for d in diag.get(CompilerPass::AST) {
    //             println!("This is the error: {}", d);
    //         }
    //     }
    //
    //     assert!(res.is_ok());
    // }

    fn check_no_err_multiple_ast_with_syntax_ret<'a, F>(text: &'a str, parser: F)
    where
        F: FnOnce(&mut Lexer, &mut DiagnosticAccumulator) -> (Vec<BoxAST<'a>>, Vec<ErminiaType>),
    {
        let mut tokens = Lexer::new(text);
        let mut diag = DiagnosticAccumulator::new();

        tokens.advance();

        let (res, _) = parser(&mut tokens, &mut diag);

        if res.iter().any(|ast| ast.is_err()) {
            println!("{:?}", res);
            for d in diag.get(CompilerPass::Lexer) {
                println!("{}", d);
            }
        }

        assert!(res.iter().all(|ast| ast.is_ok()))
    }

    fn check_type(text: &str, expected_type: ErminiaType) {
        let mut tokens = Lexer::new(text);
        let mut diag = DiagnosticAccumulator::new();

        let start = tokens.get_previous_position();

        tokens.advance();

        let _ = consume_keyword(&mut tokens, TokenKind::LetKwd, &mut diag, start);
        let _ = consume_identifier(&mut tokens, &mut diag, start);

        let actual_type = if match_next(&mut tokens, TokenKind::Colon) {
            let _ = consume_keyword(&mut tokens, TokenKind::Colon, &mut diag, start);
            consume_data_type(&mut tokens, &mut diag, start)
        // TODO: Add logic for type inference
        } else {
            ErminiaType::default()
        };

        assert_eq!(actual_type, expected_type);
    }

    #[test]
    fn test_parse_object_decl() {
        let text = "object HA { shape: [(0,1), (0,2)], color: 1 };";

        check_no_err_single_ast(text, parse_object_decl)
    }

    #[test]
    fn test_parse_list_of_shapes() {
        let text = "[(0,1), (0,2), obj, obj(1,1), (x,y) | x <- [0..1], y <- [0..2]]";

        check_no_err_multiple_ast_with_syntax_ret(text, parse_list_of_shapes)
    }

    #[test]
    fn test_parse_object_decl2() {
        let text =
            "object HA { shape: [(0,1), (0,2), (x,y) | x <- [0..1], y <- [0..2]], color: 1 };";

        check_no_err_single_ast(text, parse_object_decl)
    }

    #[test]
    fn test_parse_object_desc() {
        let text = "shape : [(0,1), (0,2)], color : 1";

        check_no_err_single_ast(text, parse_object_desc)
    }

    #[test]
    fn test_parse_shape() {
        let text = "shape : [(0,1), (0,2)]";

        check_no_err_single_ast(text, parse_object_shape)
    }

    #[test]
    fn test_parse_color() {
        let text = "color : 1";

        check_no_err_single_ast(text, parse_object_color)
    }

    #[test]
    fn test_parse_shape_tuple_compr() {
        let text = "(x,y) | x <- [0..1], y <- [0..1]";

        check_no_err_single_ast(text, parse_shape)
    }

    #[test]
    fn test_parse_var_def() {
        let text = "let x: object = HA(0,1);";

        check_no_err_single_ast(text, parse_var_def)
    }

    #[test]
    fn test_parse_var_def_explicit_object_type() {
        let text = "let x: object = HA(0,1);";

        check_type(text, ErminiaType::Object);
    }

    #[test]
    fn test_parse_var_def_default_object() {
        let text = "let x: object = HA;";

        check_no_err_single_ast(text, parse_var_def)
    }

    #[test]
    fn test_parse_var_def_default_explicit_object_type() {
        let text = "let x: object = HA;";

        check_type(text, ErminiaType::Object);
    }

    #[test]
    fn test_parse_var_def_default_object_no_type() {
        let text = "let x = HA;";

        check_no_err_single_ast(text, parse_var_def)
    }

    #[test]
    fn test_range() {
        let text = "object Shape { shape : [(0,1), (1,1)], color: 1 };";

        check_no_err_single_ast(text, parse_object_decl)
    }

    #[test]
    fn test_parse_problem_example() {
        let text = "example sol1 (2) {

            input i1 (0, 1) {
                let x: object = HA(0,1);
                draw(1, x, a);
            };

            output o1 (0, 1) {
                let y: object = HA(1,1);
                draw(1, y, b);
            };
        };";

        check_no_err_single_ast(text, parse_problem_example)
    }

    #[test]
    fn test_parse_problem_solution() {
        let text = "solution sol1 (1) {

            input i1 (0, 1) {
                let x: object = HA(0,1);
                draw(1, x, a);
            };

            output o1 (0, 1) {
                let y: object = HA(1,1);
                draw(1, y, b);
            };
        };";

        check_no_err_single_ast(text, parse_problem_solution)
    }

    #[test]
    fn test_parse_problem_input() {
        let text = "input in1 (0, 1) {
            let x: object = HA(0,1);
            draw(1, x, a);
        };";

        check_no_err_single_ast(text, parse_problem_input)
    }

    #[test]
    fn test_parse_problem_output() {
        let text = "output out1 (0, 1) {
            let y: object = HA(1,1);
            draw(1, y, b);
        };";

        check_no_err_single_ast(text, parse_problem_output)
    }

    #[test]
    fn test_parse_program() {
        let text = "def problem1 (2) {
            object HA { shape: [(0,1), (0,2)], color: 1 };

            example ex1 (1) {
                input in1 (0, 1) {
                    let x: object = HA(0,1);
                    draw(1, x, a);
                };

                output out1 (0, 1) {
                    let y: object = HA(1,1);
                    draw(1, y, b);
                };
            };

            solution sol1 (1) {
                input in1 (0, 1) {
                    let x: object = HA(0,1);
                    draw(1, x, a);
                };

                output out1 (0, 1) {
                    let y: object = HA(1,1);
                    draw(1, y, b);
                };
            };

        }";

        check_no_err_single_ast(text, parse_problem_decl)
    }

    #[test]
    fn test_parse_program_2() {
        let text = "def hello (2) {};";

        check_no_err_single_ast(text, parse_problem_decl)
    }

    #[test]
    #[should_panic]
    fn test_parse_with_unexpected_token() {
        let text = "let x: ∂ = HA(0, 1);";

        println!("{}", text);

        check_no_err_single_ast(text, parse_var_def)
    }

    #[test]
    #[should_panic]
    fn test_parse_with_other_unexpected_token() {
        let text = "let x: @ = HA(0, 1);";

        println!("{}", text);

        check_no_err_single_ast(text, parse_object_decl)
    }

    #[test]
    #[should_panic]
    fn test_parse_with_unexpected_token_parse_errors() {
        let text = "def pr (2) { let x @ HA(0, 1); let y @ HA(1, 1); };";

        println!("{}", text);

        check_no_err_single_ast(text, parse_problem_decl)
    }
}
