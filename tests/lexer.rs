use reggae_lang::lexer::{Keyword, Lexer, LexerError, Token};

#[test]
fn parse_empty_content() {
    let mut lexer = Lexer::new("".to_string());
    assert_eq!(lexer.parse(), Ok(vec![]));
}

#[test]
fn parse_identifier() {
    let mut lexer = Lexer::new(
        r#"
        abc
        a
        b2b
        a123de
        A321d
        1abc
    "#
        .to_string(),
    );

    assert_eq!(lexer.next(), Ok(Token::Identifier("abc".to_string())));
    assert_eq!(lexer.next(), Ok(Token::Identifier("a".to_string())));
    assert_eq!(lexer.next(), Ok(Token::Identifier("b2b".to_string())));
    assert_eq!(lexer.next(), Ok(Token::Identifier("a123de".to_string())));
    assert_eq!(lexer.next(), Ok(Token::Identifier("A321d".to_string())));
    assert_eq!(lexer.next(), Err(LexerError::InvalidNumberLiteral))
}

#[test]
fn parse_keyword() {
    let mut lexer = Lexer::new(
        r#"
        roots
        root
    "#
        .to_string(),
    );

    assert_eq!(lexer.next(), Ok(Token::Keyword(Keyword::Roots)));
    assert_eq!(lexer.next(), Ok(Token::Identifier("root".to_string())));
}

#[test]
fn parse_number() {
    let mut lexer = Lexer::new(
        r#"
        0
        1
        123
        1234
        1e
    "#
        .to_string(),
    );

    assert_eq!(lexer.next(), Ok(Token::NumberLiteral(0)));
    assert_eq!(lexer.next(), Ok(Token::NumberLiteral(1)));
    assert_eq!(lexer.next(), Ok(Token::NumberLiteral(123)));
    assert_eq!(lexer.next(), Ok(Token::NumberLiteral(1234)));
    assert_eq!(lexer.next(), Err(LexerError::InvalidNumberLiteral));
}

#[test]
fn parse_string_literal() {
    let mut lexer = Lexer::new(
        r#"
        "Hello World"
        'Reggae Music!'
        "Unclosed string literal
    "#
        .to_string(),
    );

    assert_eq!(
        lexer.next(),
        Ok(Token::StringLiteral("Hello World".to_string()))
    );
    assert_eq!(
        lexer.next(),
        Ok(Token::StringLiteral("Reggae Music!".to_string()))
    );
    assert_eq!(lexer.next(), Err(LexerError::StringLiteralNotClosed));
}

#[test]
fn parse_symbols() {
    let mut lexer = Lexer::new(";()=,_".to_string());
    assert_eq!(lexer.next(), Ok(Token::SemiColon));
    assert_eq!(lexer.next(), Ok(Token::OpenParen));
    assert_eq!(lexer.next(), Ok(Token::CloseParen));
    assert_eq!(lexer.next(), Ok(Token::Equals));
    assert_eq!(lexer.next(), Ok(Token::Colon));
    assert_eq!(lexer.next(), Err(LexerError::UnexpectedChar('_')));
}
