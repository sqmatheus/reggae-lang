use std::{
    error::Error,
    fmt::{Display, Formatter},
};

use crate::lexer::{Keyword, Token};

#[allow(dead_code)]
#[derive(Debug, PartialEq)]
pub enum ParserError {
    Eof,
    Unknown,
}

impl Display for ParserError {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        match self {
            ParserError::Eof => write!(f, "End of file"),
            ParserError::Unknown => write!(f, "Unknown"),
        }
    }
}

impl Error for ParserError {}

#[derive(Debug)]
pub enum Expression {
    Literal(Token),
    // Binary,
    Identifier(String),
    FunctionCall {
        name: String,
        arguments: Vec<Expression>,
    },
}

#[derive(Debug)]
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
            Err(ParserError::Unknown)
        }
    }

    fn try_parse_variable_declaration(&mut self) -> Result<Statement, ParserError> {
        let variable_name = match self.consume() {
            Some(Token::Identifier(name)) => name,
            _ => return Err(ParserError::Unknown),
        };

        match self.consume() {
            Some(Token::Equals) => (),
            _ => return Err(ParserError::Unknown),
        };

        let expression = self.try_parse_expression()?;

        match self.consume() {
            Some(Token::SemiColon) => (),
            _ => return Err(ParserError::Unknown),
        };

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
                        return Err(ParserError::Unknown);
                    }

                    self.cursor += 1;
                    return Ok(Expression::FunctionCall { name, arguments });
                }
                Token::Colon => {
                    if previous_colon {
                        return Err(ParserError::Unknown);
                    }

                    self.cursor += 1;
                    previous_colon = true
                }
                _ => {
                    if arguments.len() > 0 && !previous_colon {
                        return Err(ParserError::Unknown);
                    }

                    arguments.push(self.try_parse_expression()?);
                    previous_colon = false;
                }
            }
        }
        Err(ParserError::Unknown)
    }

    fn try_parse_funcall_statment(&mut self, name: String) -> Result<Statement, ParserError> {
        let expression = self.try_parse_funcall(name)?;

        match self.consume() {
            Some(Token::SemiColon) => (),
            _ => return Err(ParserError::Unknown),
        };

        Ok(Statement::ExpressionStatement(expression))
    }

    fn try_parse_identifier(&mut self, name: String) -> Result<Statement, ParserError> {
        match self.consume() {
            Some(Token::OpenParen) => self.try_parse_funcall_statment(name),
            _ => Err(ParserError::Unknown),
        }
    }

    fn next(&mut self) -> Result<Statement, ParserError> {
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
