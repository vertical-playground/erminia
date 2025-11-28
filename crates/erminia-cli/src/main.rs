use erminia::syntax::parse::Parser;
use std::io::{self, Write};

fn main() -> io::Result<()> {
    let stdin = io::stdin();

    let mut stdout = io::stdout();

    let mut input = String::new();

    loop {
        write!(stdout, "-> ")?;
        stdout.flush()?;

        input.clear();
        stdin.read_line(&mut input)?;

        let mut parser = Parser::new(&input);

        let program = parser.parse();

        println!("{:?}", program);

        // parse to AST Tree
        // check semantics
        // generate json
    }
}
