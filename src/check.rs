use crate::ast::{Ast, Expression, Statement, Value};

pub fn validate(ast: &Ast) -> Result<(), String> {
    for statement in &ast.statements {
        match statement {
            Statement::Print(print_stmt) => {
                match *print_stmt.expression {
                    Expression::Value(_) => Ok(()),
                    Expression::Variable(ref var_name) => {
                        Err(format!("Variable '{}' not yet supported", var_name))
                    }
                }?;
            }
            Statement::VariableDeclaration(_) => {
                return Err("Variable declarations not yet supported".to_string());
            }
        }
    }
    Ok(())
}
