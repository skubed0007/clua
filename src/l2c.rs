use crate::ast::{Ast, Expression, Statement, Value};

pub fn l2c(ast: Ast) -> String {
    let mut c_code = String::from("#include <stdio.h>\n\nint main() {\n");

    for statement in ast.statements {
        match statement {
            Statement::Print(print_stmt) => {
                match *print_stmt.expression {
                    Expression::Value(Value::String(s)) => {
                        c_code.push_str(&format!("    printf(\"%s\\n\", \"{}\");\n", s));
                    }
                    Expression::Value(Value::Number(n)) => {
                        c_code.push_str(&format!("    printf(\"%f\\n\", {});\n", n));
                    }
                    Expression::Value(Value::Boolean(b)) => {
                        c_code.push_str(&format!("    printf(\"%s\\n\", \"{}\");\n", b));
                    }
                    Expression::Value(Value::Nil) => {
                        c_code.push_str("    printf(\"nil\\n\");\n");
                    }
                    Expression::Variable(var_name) => {
                        c_code.push_str(&format!("    // TODO: Handle variable {}\n", var_name));
                    }
                }
            }
            Statement::VariableDeclaration(_) => {
                c_code.push_str("    // TODO: Handle variable declaration\n");
            }
        }
    }

    c_code.push_str("    return 0;\n}\n");
    c_code
}
