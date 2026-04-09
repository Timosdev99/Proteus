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
    If,
    Else,
    For,

    //symbols
    LParen,
    RParen,
    LBrace,
    RBrace,
    Colon,
    Semicolon,
    Comma,
    Arrow,
    Assign,
    Comments,

    //Arithemetics
    Multiplication,
    Addition,
    Substraction,
    Division,
    Equate,
    Less,
    Greater,

    TypeAnnotation(String),

    EOF,
}
