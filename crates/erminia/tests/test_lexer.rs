use erminia::lexer::lex::*;
use erminia::lexer::token::*;

#[cfg(test)]
mod test_lexer {
    use super::*;

    fn check_lex(text: &str, expected: Vec<Token>) {
        let mut lexer = Lexer::new(text);
        let actual = lexer.lex_with_separate_pass();

        assert_eq!(expected, actual);
    }

    #[test]
    fn test_lex_def_leftpar_comment_start() {
        let text = "   def ((*!!=";

        let expected: Vec<Token> = vec![
            Token::new(TokenKind::ProblemDef, "def", 1, 3),
            Token::new(TokenKind::LeftPar, "(", 1, 7),
            Token::new(TokenKind::CommentStart, "(*", 1, 8),
            Token::new(TokenKind::Not, "!", 1, 10),
            Token::new(TokenKind::NotEquals, "!=", 1, 11),
            Token::new(TokenKind::EOF, "", 1, 13),
        ];

        check_lex(text, expected);
    }

    #[test]
    fn test_lex_multiple_pluses() {
        let text = "++++ ++ ++ +";

        let expected: Vec<Token> = vec![
            Token::new(TokenKind::Increment, "++", 1, 0),
            Token::new(TokenKind::Increment, "++", 1, 2),
            Token::new(TokenKind::Increment, "++", 1, 5),
            Token::new(TokenKind::Increment, "++", 1, 8),
            Token::new(TokenKind::Plus, "+", 1, 11),
            Token::new(TokenKind::EOF, "", 1, 12),
        ];

        check_lex(text, expected);
    }

    #[test]
    fn test_lex_unfinished_string() {
        let text = "\"hello";

        let expected: Vec<Token> = vec![
            Token::new(TokenKind::String, "\"hello", 1, 0),
            Token::new(TokenKind::EOF, "", 1, 6),
        ];

        check_lex(text, expected);
    }

    #[test]
    fn test_lex_string() {
        let text = "\"poustiiiii hliaaaa\"";

        let expected: Vec<Token> = vec![
            Token::new(TokenKind::String, "\"poustiiiii hliaaaa\"", 1, 0),
            Token::new(TokenKind::EOF, "", 1, 20),
        ];

        check_lex(text, expected);
    }

    #[test]
    fn test_float_member_int() {
        let text = "123.123.123";

        let expected: Vec<Token> = vec![
            Token::new(TokenKind::Float, "123.123", 1, 0),
            Token::new(TokenKind::Member, ".", 1, 7),
            Token::new(TokenKind::Int, "123", 1, 8),
            Token::new(TokenKind::EOF, "", 1, 11),
        ];

        check_lex(text, expected);
    }

    #[test]
    fn test_int_floats() {
        let text = "123.123 123";

        let expected: Vec<Token> = vec![
            Token::new(TokenKind::Float, "123.123", 1, 0),
            Token::new(TokenKind::Int, "123", 1, 8),
            Token::new(TokenKind::EOF, "", 1, 11),
        ];

        check_lex(text, expected);
    }

    #[test]
    fn test_floats() {
        let text = "123.123";

        let expected: Vec<Token> = vec![
            Token::new(TokenKind::Float, "123.123", 1, 0),
            Token::new(TokenKind::EOF, "", 1, 7),
        ];

        check_lex(text, expected);
    }

    #[test]
    fn test_lex_leftarrow() {
        let text = "<-";

        let expected: Vec<Token> = vec![
            Token::new(TokenKind::LeftArrow, "<-", 1, 0),
            Token::new(TokenKind::EOF, "", 1, 2),
        ];

        check_lex(text, expected);
    }

    #[test]
    fn test_poisoned_token() {
        let text = "@   object";

        let expected: Vec<Token> = vec![
            Token::new(TokenKind::Poisoned, "@", 1, 0),
            Token::new(TokenKind::Object, "object", 1, 4),
            Token::new(TokenKind::EOF, "", 1, 10),
        ];

        check_lex(text, expected);
    }

    #[test]
    fn test_lex_fn() {
        let text = "def \n hello_ () { 103 }";

        let expected: Vec<Token> = vec![
            Token::new(TokenKind::ProblemDef, "def", 1, 0),
            Token::new(TokenKind::Ident, "hello_", 2, 6),
            Token::new(TokenKind::LeftPar, "(", 2, 13),
            Token::new(TokenKind::RightPar, ")", 2, 14),
            Token::new(TokenKind::LeftBrace, "{", 2, 16),
            Token::new(TokenKind::Int, "103", 2, 18),
            Token::new(TokenKind::RightBrace, "}", 2, 22),
            Token::new(TokenKind::EOF, "", 2, 23),
        ];

        check_lex(text, expected);
    }

    #[test]
    fn test_start_with_for_keyword_with_symbol_after() {
        let text = "color,";

        assert!(text.starts_with("color"))
    }
}
