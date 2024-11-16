use crate::ast::{Ast, Expression, Print, Statement, Value};

#[allow(unused)]
#[derive(Debug, Clone)]
pub enum Token {
    Print,
    String(String),
    Number(f64),
    Identifier(String),
    LeftParen,
    RightParen,
    Invalid(String),
}

pub fn tokenize(code: &str) -> Vec<Token> {
    let mut tokens = Vec::new();
    for line in code.lines() {
        let line = line.trim();
        if line.starts_with("print") {
            tokens.push(Token::Print);
            if line.contains("(") {
                tokens.push(Token::LeftParen);
                if let Some(content) = line
                    .strip_prefix("print(\"")
                    .and_then(|s| s.strip_suffix("\")"))
                {
                    tokens.push(Token::String(content.to_string()));
                }
                tokens.push(Token::RightParen);
            }
        } else {
            tokens.push(Token::Invalid(line.to_string()));
        }
    }
    tokens
}

pub fn lex(code: &str) -> Ast {
    let tokens = tokenize(code);
    let mut ast = Ast::new();

    let mut i = 0;
    while i < tokens.len() {
        match &tokens[i] {
            Token::Print => {
                if i + 3 <= tokens.len() {
                    if let Token::String(s) = &tokens[i + 2] {
                        ast.add_statement(Statement::Print(Print {
                            expression: Box::new(Expression::Value(Value::String(s.clone()))),
                        }));
                    }
                    i += 4;
                } else {
                    i += 1;
                }
            }
            Token::Invalid(code) => {
                eprintln!("Invalid code at line {}: {}", i, code);
            }
            _ => {}
        }
        i += 1;
    }
    ast
}
