#[derive(Debug, PartialEq)]
pub enum Token {
    //literals
    Identifier(String),
    NumberLiteral(f64),
    StringLiteral(String),

    //keywords
    Function,
    Return,
    Interface,
    Const,
    Let,
    Export,
    //symbols
    LParen,
    RParen,
    LBrace,
    RBrace,
    Colon,
    Semicolon,
    Comma,
    Arrow,

    //Arithemetics
    Multiplication,
    Addition,
    Substraction,
    Division,

    TypeAnnotation(String),

    EOF,
}
