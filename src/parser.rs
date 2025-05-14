use crate::lexer::{Keyword, Token};
use std::{
    error::Error,
    fmt::{Display, Formatter},
};

macro_rules! expect_token {
    ($expr:expr, $token:ident) => {
        if !matches!($expr, Some(Token::$token)) {
            return Err(ParserError::ExpectedToken(Token::$token));
        }
    };
}

#[allow(dead_code)]
#[derive(Debug, PartialEq)]
pub enum ParserError {
    InvalidFuncall,
    ExpectedExpression,
    ExpectedIdentifier,
    ExpectedToken(Token),
    Eof,
    Unknown,
}

impl Display for ParserError {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        match self {
            ParserError::InvalidFuncall => write!(f, "Invalid funcall"),
            ParserError::ExpectedExpression => write!(f, "Expected expression"),
            ParserError::ExpectedIdentifier => write!(f, "Expected identifier"),
            ParserError::ExpectedToken(token) => write!(f, "Expected token: {}", token),
            ParserError::Eof => write!(f, "End of file"),
            ParserError::Unknown => write!(f, "Unknown"),
        }
    }
}

impl Error for ParserError {}

#[derive(Debug, PartialEq)]
pub enum Expression {
    Literal(Token),
    // Binary,
    Identifier(String),
    FunctionCall {
        name: String,
        arguments: Vec<Expression>,
    },
}

#[derive(Debug, PartialEq)]
pub enum Statement {
    VariableDeclaration {
        name: String,
        expression: Expression,
    },
    ExpressionStatement(Expression),
}

pub struct Parser {
    tokens: Vec<Token>,
    cursor: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Parser { tokens, cursor: 0 }
    }

    fn peek(&self) -> Option<Token> {
        if let Some(token) = self.tokens.get(self.cursor) {
            Some(token.clone())
        } else {
            None
        }
    }

    fn consume(&mut self) -> Option<Token> {
        let ch = self.peek();
        self.cursor += 1;
        ch
    }

    fn consume_expect_indentifier(&mut self) -> Result<String, ParserError> {
        let Token::Identifier(identifier) = self.consume().ok_or(ParserError::Eof)? else {
            return Err(ParserError::ExpectedIdentifier);
        };
        Ok(identifier)
    }

    fn try_parse_expression(&mut self) -> Result<Expression, ParserError> {
        if let Some(token) = self.consume() {
            Ok(match token {
                Token::NumberLiteral(_) | Token::StringLiteral(_) => Expression::Literal(token),
                Token::Identifier(identifier) => match self.peek() {
                    Some(Token::OpenParen) => {
                        self.cursor += 1;
                        self.try_parse_funcall(identifier)?
                    }
                    _ => Expression::Identifier(identifier),
                },
                _ => return Err(ParserError::Unknown),
            })
        } else {
            Err(ParserError::Eof)
        }
    }

    fn try_parse_variable_declaration(&mut self) -> Result<Statement, ParserError> {
        let variable_name = self.consume_expect_indentifier()?;
        expect_token!(self.consume(), Equals);

        let expression = self
            .try_parse_expression()
            .map_err(|_| ParserError::ExpectedExpression)?;

        expect_token!(self.consume(), SemiColon);

        Ok(Statement::VariableDeclaration {
            name: variable_name,
            expression,
        })
    }

    fn try_parse_keyword(&mut self, begin: Keyword) -> Result<Statement, ParserError> {
        match begin {
            Keyword::Roots => self.try_parse_variable_declaration(),
            _ => Err(ParserError::Unknown),
        }
    }

    fn try_parse_funcall(&mut self, name: String) -> Result<Expression, ParserError> {
        let mut previous_colon = false;
        let mut arguments = Vec::new();
        while let Some(token) = self.peek() {
            match token {
                Token::CloseParen => {
                    if previous_colon {
                        return Err(ParserError::InvalidFuncall);
                    }

                    self.cursor += 1;
                    return Ok(Expression::FunctionCall { name, arguments });
                }
                Token::Colon => {
                    if previous_colon {
                        return Err(ParserError::InvalidFuncall);
                    }

                    self.cursor += 1;
                    previous_colon = true
                }
                _ => {
                    if arguments.len() > 0 && !previous_colon {
                        return Err(ParserError::InvalidFuncall);
                    }

                    arguments.push(self.try_parse_expression()?);
                    previous_colon = false;
                }
            }
        }
        Err(ParserError::InvalidFuncall)
    }

    fn try_parse_funcall_statment(&mut self, name: String) -> Result<Statement, ParserError> {
        let expression = self.try_parse_funcall(name)?;
        expect_token!(self.consume(), SemiColon);

        Ok(Statement::ExpressionStatement(expression))
    }

    fn try_parse_identifier(&mut self, name: String) -> Result<Statement, ParserError> {
        match self.consume() {
            Some(Token::OpenParen) => self.try_parse_funcall_statment(name),
            _ => Err(ParserError::Unknown),
        }
    }

    pub fn next(&mut self) -> Result<Statement, ParserError> {
        if let Some(token) = self.consume() {
            match token {
                Token::Keyword(keyword) => self.try_parse_keyword(keyword),
                Token::Identifier(name) => self.try_parse_identifier(name),
                _ => Err(ParserError::Unknown),
            }
        } else {
            Err(ParserError::Eof)
        }
    }

    pub fn parse(&mut self) -> Result<Vec<Statement>, ParserError> {
        let mut statments = Vec::new();
        loop {
            match self.next() {
                Ok(statement) => statments.push(statement),
                Err(err) => {
                    if err == ParserError::Eof {
                        break;
                    }
                    return Err(err);
                }
            }
        }
        Ok(statments)
    }
}
