use std::{
    error::Error,
    fmt::{Display, Formatter},
};

#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq)]
pub enum Keyword {
    Roots,
    If,
    Else,
    While,
}

impl Display for Keyword {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        match self {
            Keyword::Roots => write!(f, "roots"),
            Keyword::If => todo!(),
            Keyword::Else => todo!(),
            Keyword::While => todo!(),
        }
    }
}

#[allow(dead_code)]
#[derive(Debug, PartialEq)]
pub enum LexerError {
    InvalidNumberLiteral,
    StringLiteralNotClosed,
    UnexpectedChar(char),
    Eof,
    Unknown,
}

impl Display for LexerError {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        match self {
            LexerError::InvalidNumberLiteral => write!(f, "Invalid number literal"),
            LexerError::StringLiteralNotClosed => write!(f, "String literal not closed"),
            LexerError::UnexpectedChar(ch) => write!(f, "Unexpected {}", ch),
            LexerError::Eof => write!(f, "End of file"),
            LexerError::Unknown => write!(f, "Unknown"),
        }
    }
}

impl Error for LexerError {}

#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Identifier(String),
    Keyword(Keyword),
    // Literals
    StringLiteral(String),
    NumberLiteral(i64),
    // Symbols
    SemiColon,
    OpenParen,
    CloseParen,
    Equals,
    Colon,
}

impl Display for Token {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        match self {
            Token::Identifier(_) => write!(f, "identifier"),
            Token::Keyword(keyword) => write!(f, "{}", keyword),
            Token::StringLiteral(_) => write!(f, "string literal"),
            Token::NumberLiteral(_) => write!(f, "number literal"),
            Token::SemiColon => write!(f, ";"),
            Token::OpenParen => write!(f, "("),
            Token::CloseParen => write!(f, ")"),
            Token::Equals => write!(f, "="),
            Token::Colon => write!(f, ","),
        }
    }
}

#[derive(Debug)]
pub struct Lexer {
    content: Vec<char>,
    cursor: usize,
}

impl Lexer {
    pub fn new(content: String) -> Self {
        Self {
            content: content.chars().collect(),
            cursor: 0,
        }
    }

    fn chop(&mut self) {
        while let Some(char) = self.content.get(self.cursor) {
            if char.is_whitespace() {
                self.cursor += 1
            } else {
                break;
            }
        }
    }

    fn peek(&self) -> Option<char> {
        if let Some(ch) = self.content.get(self.cursor) {
            Some(*ch)
        } else {
            None
        }
    }

    fn consume(&mut self) -> Option<char> {
        let ch = self.peek();
        self.cursor += 1;
        ch
    }

    fn try_parse_string_literal(&mut self, delimiter: char) -> Result<Token, LexerError> {
        let start = self.cursor;
        while let Some(ch) = self.consume() {
            if ch == delimiter {
                return Ok(Token::StringLiteral(
                    self.content[start..self.cursor - 1]
                        .iter()
                        .collect::<String>(),
                ));
            }
        }
        Err(LexerError::StringLiteralNotClosed)
    }

    fn parse_identifier_and_keyword(&mut self) -> Token {
        let start = self.cursor;
        while let Some(ch) = self.peek() {
            if !ch.is_alphanumeric() {
                break;
            }
            self.cursor += 1;
        }

        let result = self.content[start - 1..self.cursor]
            .iter()
            .collect::<String>();

        match result.as_str() {
            "roots" => Token::Keyword(Keyword::Roots),
            _ => Token::Identifier(result),
        }
    }

    fn try_parse_number(&mut self) -> Result<Token, LexerError> {
        let start = self.cursor;
        while let Some(ch) = self.peek() {
            if !ch.is_numeric() {
                if ch.is_alphabetic() {
                    return Err(LexerError::InvalidNumberLiteral);
                }
                break;
            }
            self.cursor += 1;
        }

        let result = self.content[start - 1..self.cursor]
            .iter()
            .collect::<String>();

        let number = result
            .parse::<i64>()
            .map_err(|_| LexerError::InvalidNumberLiteral)?;

        Ok(Token::NumberLiteral(number))
    }

    pub fn next(&mut self) -> Result<Token, LexerError> {
        self.chop();
        let consume = self.consume();
        if let Some(current) = consume {
            match current {
                ';' => Ok(Token::SemiColon),
                '(' => Ok(Token::OpenParen),
                ')' => Ok(Token::CloseParen),
                '=' => Ok(Token::Equals),
                ',' => Ok(Token::Colon),
                '\'' | '"' => self.try_parse_string_literal(current),
                ch => {
                    if ch.is_numeric() {
                        self.try_parse_number()
                    } else if ch.is_alphabetic() {
                        Ok(self.parse_identifier_and_keyword())
                    } else {
                        Err(LexerError::UnexpectedChar(ch))
                    }
                }
            }
        } else {
            Err(LexerError::Eof)
        }
    }

    pub fn parse(&mut self) -> Result<Vec<Token>, LexerError> {
        let mut tokens = Vec::new();
        loop {
            match self.next() {
                Ok(token) => tokens.push(token),
                Err(err) => {
                    if err == LexerError::Eof {
                        break;
                    }
                    return Err(err);
                }
            }
        }
        Ok(tokens)
    }
}
