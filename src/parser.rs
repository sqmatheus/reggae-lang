use crate::lexer::{Keyword, Token};

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

    fn try_parse_expression(&mut self) -> Result<Expression, ()> {
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
                _ => return Err(()),
            })
        } else {
            Err(())
        }
    }

    fn try_parse_variable_declaration(&mut self) -> Result<Statement, ()> {
        let variable_name = match self.consume() {
            Some(Token::Identifier(name)) => name,
            _ => return Err(()),
        };

        match self.consume() {
            Some(Token::Equals) => (),
            _ => return Err(()),
        };

        let expression = self.try_parse_expression()?;

        match self.consume() {
            Some(Token::SemiColon) => (),
            _ => return Err(()),
        };

        Ok(Statement::VariableDeclaration {
            name: variable_name,
            expression,
        })
    }

    fn try_parse_keyword(&mut self, begin: Keyword) -> Result<Statement, ()> {
        match begin {
            Keyword::Roots => self.try_parse_variable_declaration(),
            _ => Err(()),
        }
    }

    fn try_parse_funcall(&mut self, name: String) -> Result<Expression, ()> {
        let mut previous_colon = false;
        let mut arguments = Vec::new();
        while let Some(token) = self.peek() {
            match token {
                Token::CloseParen => {
                    if previous_colon {
                        return Err(());
                    }

                    self.cursor += 1;
                    return Ok(Expression::FunctionCall { name, arguments });
                }
                Token::Colon => {
                    if previous_colon {
                        return Err(());
                    }

                    self.cursor += 1;
                    previous_colon = true
                }
                _ => {
                    if arguments.len() > 0 && !previous_colon {
                        return Err(());
                    }

                    arguments.push(self.try_parse_expression()?);
                    previous_colon = false;
                }
            }
        }
        Err(())
    }

    fn try_parse_funcall_statment(&mut self, name: String) -> Result<Statement, ()> {
        let expression = self.try_parse_funcall(name)?;

        match self.consume() {
            Some(Token::SemiColon) => (),
            _ => return Err(()),
        };

        Ok(Statement::ExpressionStatement(expression))
    }

    fn try_parse_identifier(&mut self, name: String) -> Result<Statement, ()> {
        match self.consume() {
            Some(Token::OpenParen) => self.try_parse_funcall_statment(name),
            _ => Err(()),
        }
    }

    fn next(&mut self) -> Result<Statement, ()> {
        if let Some(token) = self.consume() {
            match token {
                Token::Keyword(keyword) => self.try_parse_keyword(keyword),
                Token::Identifier(name) => self.try_parse_identifier(name),
                _ => todo!(),
            }
        } else {
            Err(())
        }
    }

    pub fn parse(&mut self) -> Vec<Statement> {
        let mut statments = Vec::new();
        while let Ok(statement) = self.next() {
            statments.push(statement);
        }
        return statments;
    }
}
