#[derive(Debug)]
pub enum Expression {
    Identifier(String),
    NumberLiteral(f64),
    StringLiteral(String),

    TypeAnnotation(String),
    BinaryOp {
        left: Box<Expression>,
        right: Box<Expression>,
        op: String,
    },
}

#[derive(Debug)]
pub struct Params {
    pub name: String,
    pub type_annotation: Option<String>,
    pub default: Option<Expression>,
}

#[derive(Debug)]
pub enum Statement {
    Return(Expression),
}

#[derive(Debug)]
pub struct Function {
    pub name: String,
    pub params: Vec<Params>,
    pub return_types: Option<String>,
    pub body: Vec<Statement>,
}

#[derive(Debug)]
struct Field {
    pub name: String,
    pub type_annotation: Expression,
}

#[derive(Debug)]
struct Interface {
    name: String,
    fields: Vec<Field>,
}

pub enum TopLevel {
    Function(Function),
    Interface(Interface),
}

pub struct Program {
    pub body: Vec<TopLevel>,
}
