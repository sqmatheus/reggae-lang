use std::vec;

use reggae_lang::{
    lexer::{Keyword, Token},
    parser::{Expression, Parser, ParserError, Statement},
};

#[test]
fn valid_variable_declaration() {
    let mut parser = Parser::new(vec![
        Token::Keyword(Keyword::Roots),
        Token::Identifier("x".to_string()),
        Token::Equals,
        Token::NumberLiteral(1),
        Token::SemiColon,
    ]);

    assert_eq!(
        parser.next(),
        Ok(Statement::VariableDeclaration {
            name: "x".to_string(),
            expression: Expression::Literal(Token::NumberLiteral(1))
        })
    )
}

#[test]
fn invalid_variable_declaration_without_semicolon() {
    let mut parser = Parser::new(vec![
        Token::Keyword(Keyword::Roots),
        Token::Identifier("x".to_string()),
        Token::Equals,
        Token::NumberLiteral(1),
    ]);

    assert_eq!(
        parser.next(),
        Err(ParserError::ExpectedToken(Token::SemiColon))
    )
}

#[test]
fn invalid_variable_declaration_without_expression() {
    let mut parser = Parser::new(vec![
        Token::Keyword(Keyword::Roots),
        Token::Identifier("x".to_string()),
        Token::Equals,
        Token::SemiColon,
    ]);

    assert_eq!(parser.next(), Err(ParserError::ExpectedExpression))
}
