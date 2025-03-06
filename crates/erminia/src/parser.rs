mod expr;

use crate::lexer::{Lexer, SyntaxKind};
use crate::syntax::{ErminiaLanguage, SyntaxNode};
use expr::expr;
use rowan::{GreenNodeBuilder, GreenNode, Language, Checkpoint};
use std::iter::Peekable;

pub struct Parser<'a> {
    lexer: Peekable<Lexer<'a>>,
    builder: GreenNodeBuilder<'static>
}

impl<'a> Parser<'a> {
    pub fn new(input: &'a str) -> Self {
        Self {
            lexer: Lexer::new(input).peekable(),
            builder: GreenNodeBuilder::new()
        }
    }

     pub fn parse(mut self) -> Parse {
        self.start_node(SyntaxKind::Root);

        match self.peek() {
            Some(SyntaxKind::Number) | Some(SyntaxKind::Ident) => self.bump(),
            _ => {}
        }

        self.finish_node();

        Parse {
            green_node: self.builder.finish(),
        }
    }
    
    fn peek(&mut self) -> Option<SyntaxKind> {
        self.lexer.peek().map(|(kind,_)| *kind)
    }

    fn bump(&mut self) {
        let (kind, text) = self.lexer.next().unwrap();

        self.builder
            .token(ErminiaLanguage::kind_to_raw(kind), text.into());
    }

    fn start_node_at(&mut self, checkpoint: Checkpoint, kind: SyntaxKind) {
        self.builder
            .start_node_at(checkpoint, ErminiaLanguage::kind_to_raw(kind));
    }

    fn checkpoint(&self) -> Checkpoint {
        self.builder.checkpoint()
    }

    fn start_node(&mut self, kind: SyntaxKind) {
        self.builder.start_node(ErminiaLanguage::kind_to_raw(kind));
    }

    fn finish_node(&mut self) {
        self.builder.finish_node()
    }
}

pub struct Parse {
    green_node: GreenNode,
}

impl Parse { 
    pub fn debug_tree(&self) -> String { 
        let syntax_node = SyntaxNode::new_root(self.green_node.clone());
        let formatted = format!("{:#?}", syntax_node);

        formatted[0..formatted.len() - 1].to_string()
    }
}

#[cfg(test)]
fn check(input: &str, expect_test: expect_test::Expect) {
    let parse = Parser::new(input).parse();
    expect_test.assert_eq(&parse.debug_tree());
}

#[cfg(test)]
mod tests {
    use super::*;
    use expect_test::expect;

    #[test]
    fn parse_nothing() {
        check("", expect![[r#"Root@0..0"#]]);
    }

}
