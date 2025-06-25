use crate::lexer::lex_s::Lexer;
use crate::lexer::token::TokenKind;
use crate::error::parser_error::{ParserResult, ParserError};

fn consume_identifier(tokens: &mut Lexer) -> ParserResult<()> {
    match tokens.peek().get_kind() {
        TokenKind::Ident => {
            tokens.advance()?;
            Ok(())
        }
        _ => Err(ParserError::ParserError)
    }
}

fn consume_keyword(tokens: &mut Lexer, kind: TokenKind) -> ParserResult<()> {
    if tokens.peek().get_kind()  == kind {
        tokens.advance()?;
        Ok(())
    } else {
        Err(ParserError::ParserError)
    }
}

// <object_def> ::= "object" <id> <object_desc>
fn parse_object_decl(tokens: &mut Lexer) -> ParserResult<()> {
    consume_keyword(tokens, TokenKind::Object)?;
    let id = tokens.peek().get_text();
    consume_identifier(tokens)?;
    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    




}
