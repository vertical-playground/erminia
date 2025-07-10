use erminia::lexer::lex::Lexer;
use erminia::syntax::parse::Parser;
use std::io::{self, Write};

fn main() -> io::Result<()> {
    let stdin = io::stdin();

    let mut stdout = io::stdout();

    let mut input = String::new();

    loop {
        write!(stdout, "-> ")?;
        stdout.flush()?;

        stdin.read_line(&mut input)?;

        let mut lexer = Lexer::new(&input);
        let tokens = lexer.lex_with_separate_pass();

        let mut parser = Parser::new(&input);

        let program = parser.parse();

        println!("TOKENS: {:?}", tokens);
        println!("PROGRAM: {:?}", program.unwrap());

        // parse to AST Tree
        // check semantics
        // generate json

        input.clear();
    }
}
