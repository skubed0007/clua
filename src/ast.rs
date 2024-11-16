#[allow(unused)]
#[derive(Debug, Clone)]
pub enum Value {
    Number(f64),
    String(String),
    Boolean(bool),
    Nil,
}

#[allow(unused)]
// Define a variable structure
#[derive(Debug)]
pub struct Variable {
    pub name: String,
    pub value: Value,
}
#[allow(unused)]
#[derive(Debug)]
pub struct Print {
    pub expression: Box<Expression>,
}
#[allow(unused)]
#[derive(Debug)]
pub enum Expression {
    Value(Value),
    Variable(String),
}
#[allow(unused)]
#[derive(Debug)]
pub enum Statement {
    Print(Print),
    VariableDeclaration(Variable),
}
#[allow(unused)]
#[derive(Debug)]
pub struct Ast {
    pub statements: Vec<Statement>,
}

impl Ast {
    pub fn new() -> Self {
        Ast {
            statements: Vec::new(),
        }
    }

    pub fn add_statement(&mut self, statement: Statement) {
        self.statements.push(statement);
    }
}
