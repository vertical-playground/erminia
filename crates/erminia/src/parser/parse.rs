use crate::error::parser_error::{ParserError, ParserResult};
use crate::lexer::lex_s::Lexer;
use crate::lexer::token::TokenKind;

// ====================================================================================//
//                                Consumers                                            //
// ====================================================================================//

fn consume_identifier(tokens: &mut Lexer) -> ParserResult<()> {
    match tokens.peek().get_kind() {
        TokenKind::Ident => {
            tokens.advance()?;
            Ok(())
        }
        _ => Err(ParserError::ParserError),
    }
}

fn consume_keyword(tokens: &mut Lexer, kind: TokenKind) -> ParserResult<()> {
    if tokens.peek().get_kind() == kind {
        tokens.advance()?;
        Ok(())
    } else {
        Err(ParserError::ParserError)
    }
}

// ====================================================================================//
//                                Parsers                                              //
// ====================================================================================//

fn parse_object_shape_desc<T>(tokens: &mut Lexer) -> ParserResult<T> {
    Ok(())
}

fn parse_object_prior_decl<T>(tokens: &mut Lexer) -> ParserResult<T> {
    Ok(())
}

// <shapes_list> ::= "[" <shape_desc> ("," <shape_desc>)* "]"
fn parse_list_of_shapes<T>(tokens: &mut Lexer) -> ParserResult<Vec<T>> {
    let shape_descs = vec![];
    consume_keyword(tokens, TokenKind::LeftBracket)?;
    while let Some(c) = parse_object_shape_desc(tokens) {

    }
    consume_keyword(tokens, TokenKind::RightBracket)?;
    Ok(vec![])
}

// <stmts_list> ::= <stmt> ("," <stmt>)*
fn parse_list_of_stmts<T>(tokens: &mut Lexer) -> ParserResult<Vec<T>> {
    let stmts_list = vec![];
    Ok(vec![])
}

// <object_desc> ::= "{" <object_prior_decl> "}"
fn parse_object_desc<T>(tokens: &mut Lexer) -> ParserResult<T> {
    consume_keyword(tokens, TokenKind::LeftBrace)?;
    let object_desc = parse_object_prior_decl(tokens)?;
    consume_keyword(tokens, TokenKind::RightBrace)?;
    Ok(object_desc)
}

// <object_def> ::= "object" <id> <object_desc>
fn parse_object_decl(tokens: &mut Lexer) -> ParserResult<()> {
    consume_keyword(tokens, TokenKind::Object)?;
    let id = tokens.peek().get_text();
    consume_identifier(tokens)?;
    let object_desc = parse_object_desc(tokens)?;
    Ok(())
}

// <compound_stmt> ::= "{" [<stmt_list>] "}"
fn parse_compound_stmt<T>(tokens: &mut Lexer) -> ParserResult<Vec<T>> {
    consume_keyword(tokens, TokenKind::LeftBrace)?;
    let stmts: Vec<T> = parse_list_of_stmts(tokens)?;
    consume_keyword(tokens, TokenKind::RightBrace)?;
    Ok(stmts)
}

// <int_const> ::= [0-9]([0-9])*
fn consume_int_const(tokens: &mut Lexer) -> ParserResult<()> {
    if tokens.peek().get_kind() == TokenKind::Int {
        tokens.advance()?;
        Ok(())
    } else {
        Err(ParserError::ParserError)
    }
}

// <problem_declaration> ::= "def" <id> "(" [<int_const>] ")" <compound_stmt>
fn parse_problem_decl(tokens: &mut Lexer) -> ParserResult<()> {
    consume_keyword(tokens, TokenKind::ProblemDef)?;
    let id = tokens.peek().get_text();
    consume_identifier(tokens)?;
    consume_keyword(tokens, TokenKind::LeftPar)?;
    let int_const = tokens.peek().get_text();
    consume_int_const(tokens)?;
    consume_keyword(tokens, TokenKind::RightPar)?;
    let stmts = parse_compound_stmt(tokens)?;
    // TODO: Probably should work a little something like this
    // let program = Program::new(id, int_const, stmts);
    // Ok((program))
    Ok(())
}

// <program> ::= <problem_declaration>
fn parse_program<T>(tokens: &mut Lexer) -> ParserResult<T> {
    let program = parse_problem_decl(tokens)?;
    Ok(program)
}

#[cfg(test)]
mod test {
    use super::*;
}
