mod lexer;
mod parser;

use lexer::{Lexer, Token};
use parser::{Expression, Parser, Statement};
use std::{collections::HashMap, env, fs};

enum Type {
    Int(i64),
    Str(String),
}

fn print_type(ty: &Type) {
    match ty {
        Type::Int(v) => println!("{}", v),
        Type::Str(v) => println!("{}", v),
    }
}

fn expression_to_type(expression: Expression) -> Type {
    match expression {
        Expression::Literal(token) => match token {
            Token::StringLiteral(value) => Type::Str(value),
            Token::NumberLiteral(value) => Type::Int(value),
            _ => todo!(),
        },
        _ => todo!(),
    }
}

fn main() -> Result<(), ()> {
    let args: Vec<String> = env::args().collect();
    let file_path = args.get(1).expect("file path not provided");
    let content = fs::read_to_string(file_path).expect("could not read the file");

    let mut lexer = Lexer::new(content);
    let mut parser = Parser::new(lexer.parse());

    let mut variables: HashMap<String, Type> = HashMap::new();

    for statment in parser.parse() {
        match statment {
            Statement::VariableDeclaration { name, expression } => {
                let ty = expression_to_type(expression);
                variables.insert(name, ty);
            }
            Statement::ExpressionStatement(expression) => match expression {
                Expression::FunctionCall { name, arguments } => match name.as_str() {
                    "sound" => {
                        for arg in arguments {
                            match arg {
                                Expression::Identifier(identifier) => {
                                    if let Some(ty) = variables.get(&identifier) {
                                        print_type(ty);
                                    }
                                }
                                Expression::Literal(_) => {
                                    print_type(&expression_to_type(arg));
                                }
                                _ => todo!(),
                            }
                        }
                    }
                    _ => todo!(),
                },
                _ => todo!(),
            },
        }
    }

    Ok(())
}
