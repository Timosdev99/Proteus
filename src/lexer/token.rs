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

    //Arithemetics
    Multiplication,
    Addition,
    Substraction,
    Division,
    Equate,

    TypeAnnotation(String),

    EOF,
}
